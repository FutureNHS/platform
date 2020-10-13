/* eslint-disable import/order */
const express = require("express");
const session = require("express-session");
const pg = require("pg");
const pgSession = require("connect-pg-simple")(session);
const passport = require("passport");
const OIDCStrategy = require("passport-azure-ad").OIDCStrategy;
const next = require("next");
const dotenv = require("dotenv");
const Noop = require("./lib/server/noop-passport-strategy");

const url = require("url");
const { promises: fs } = require("fs");
const { getOrCreateUser } = require("./lib/server/graphql");
const { provider } = require("./tracing");
const api = require("@opentelemetry/api");
const { requireEnv } = require("./lib/server/requireEnv");

const setupSessionStore = async () => {
  const pgUrl = process.env.PG_URL;
  if (!pgUrl) {
    console.warn(
      "Environment variable PG_URL not set. Using in memory session storage."
    );
    return;
  }

  const pgParams = url.parse(pgUrl);
  const [pgUser, pgPassword] = pgParams.auth.split(":");
  const pgPool = new pg.Pool({
    user: pgUser,
    password: pgPassword,
    host: pgParams.hostname,
    port: parseInt(pgParams.port, 10),
    database: pgParams.pathname.split("/")[1],
    ssl: true,
  });
  await runSessionDbMigration(pgPool);

  return new pgSession({
    pool: pgPool,
  });
};

const runSessionDbMigration = async (pool) => {
  const client = await pool.connect();
  try {
    await client.query(await fs.readFile("database/table-schema.sql", "utf8"));
  } finally {
    client.release();
  }
};

async function main() {
  const env = process.env.NODE_ENV || "development";
  const dev = env !== "production";
  // dotenv will never modify any environment variables that have already been set,
  // so if values are defined in multiple places, we prefer the .local one.
  // This is done to emulate how next.js does things.
  dotenv.config({ path: `.env.local` });
  dotenv.config({ path: `.env.${env}.local` });
  dotenv.config({ path: `.env.${env}` });

  const port = parseInt(process.env.PORT, 10) || 3000;
  const sessionStore = await setupSessionStore();
  const sessionCookieSecret = requireEnv("COOKIE_SECRET");

  const app = next({
    dir: ".", // base directory where everything is, could move to src later
    dev,
  });
  const handle = app.getRequestHandler();
  await app.prepare();

  const server = express();

  // Login session
  const aadb2cStrategy = "aadb2c";
  const configureUserFlow = (flow) => (req, _res, next) => {
    req.session["auth.next"] = req.query.next;
    req.query.p = flow;
    next();
  };
  const authenticateWithAADB2C = passport.authenticate(aadb2cStrategy, {
    prompt: "login",
    failureRedirect: "/auth/failed",
  });
  const redirectAuthSuccess = (req, res) => {
    const next = req.session["auth.next"] || "/";
    delete req.session["auth.next"];
    res.redirect(next);
  };

  passport.serializeUser((user, done) => {
    done(null, JSON.stringify(user));
  });
  passport.deserializeUser((user, done) => {
    done(null, JSON.parse(user));
  });
  passport.use(
    aadb2cStrategy,
    dev
      ? new Noop()
      : new OIDCStrategy(
          {
            identityMetadata:
              "https://futurenhsplatform.b2clogin.com/futurenhsplatform.onmicrosoft.com/v2.0/.well-known/openid-configuration",
            clientID: requireEnv("AAD_B2C_CLIENT_ID"),
            clientSecret: requireEnv("AAD_B2C_CLIENT_SECRET"),
            responseType: "code",
            responseMode: "query",
            redirectUrl: `${requireEnv("ORIGIN")}/auth/callback`,
            passReqToCallback: false,
            allowHttpForRedirectUrl: dev,
            isB2C: true,
          },
          async (profile, done) => {
            try {
              const response = await getOrCreateUser({
                authId: profile.sub,
                name: profile.displayName,
              });
              const {
                id,
                name,
                authId,
                isPlatformAdmin,
              } = response.getOrCreateUser;
              done(null, {
                id,
                authId,
                name,
                isPlatformAdmin,
                emails: profile.emails,
              });
            } catch (err) {
              done(err);
            }
          }
        )
  );
  if (!dev) {
    // In production our frontend runs behind a Kubernetes ingress. We need to
    // tell Express.js, so it can use X-Forwarded-* headers, etc.
    server.set("trust proxy", 1);
  }
  server.use(
    session({
      store: sessionStore,
      secret: sessionCookieSecret,
      resave: false,
      saveUninitialized: false,
      cookie: {
        maxAge: 30 * 24 * 60 * 60 * 1000, // 30 days
        sameSite: "lax",
        secure: !dev,
      },
    })
  );
  server.use(passport.initialize());
  server.use(passport.session());
  server.get(
    "/auth/login",
    configureUserFlow("b2c_1_signin"),
    authenticateWithAADB2C,
    redirectAuthSuccess
  );
  server.get(
    "/auth/resetpassword",
    configureUserFlow("b2c_1_passwordreset"),
    authenticateWithAADB2C
  );
  server.get("/auth/callback", authenticateWithAADB2C, redirectAuthSuccess);
  server.get("/auth/logout", (req, res) => {
    req.session.destroy((err) => {
      err = new Error("go away");

      if (err) {
        const tracer = api.trace.getTracer("dunno");
        const span = tracer.getCurrentSpan();
        if (span) {
          span.recordException(err);
        } else {
          const span = tracer.startSpan("new span");
          span.recordException(err);
          span.end();
          console.log("logout error", err);
        }
      }
      // res.redirect("/");
    });
  });

  // Default catch-all handler to allow Next.js to handle all other routes
  server.all("*", (req, res) => handle(req, res));

  server.listen(port, (err) => {
    if (err) {
      throw err;
    }
    console.log(`> Ready on http://localhost:${port}`);
  });
}

main().catch((err) => {
  console.error("An error occurred, unable to start the server");
  console.error(err);
  process.exit(1);
});
