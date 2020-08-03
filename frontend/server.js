/// This is taken from the next.js reverse proxy example, and should
/// only be used for local development.
/// https://github.com/vercel/next.js/tree/master/examples/with-custom-reverse-proxy
/* eslint-disable no-console */
const fs = require("fs");
const express = require("express");
const https = require("https");
const next = require("next");
const process = require("process");

const devProxy = {
  "/hello": {
    target: "http://hello-world.hello-world/",
    changeOrigin: true,
  },
  "/.ory/kratos/public/self-service/browser/flows/login": {
    target: "http://kratos-public.kratos",
    pathRewrite: { "/.ory/kratos/public": "" },
    changeOrigin: false,
  },
  // another one that hits ory/kratos refer to http-proxy-middleware docs to understand the format
};

const port = parseInt(process.env.PORT, 10) || 4433;
if (process.env.NODE_ENV !== "development") {
  console.error(
    "This server should only be used by `yarn dev`, and should never be used in production."
  );
  process.exit(1);
}

try {
  var httpsOptions = {
    key: fs.readFileSync("./localhost+2-key.pem"),
    cert: fs.readFileSync("./localhost+2.pem"),
  };
} catch (error) {
  console.warn("Please run `yarn mkcert` to generate localhost*.pem");
  process.exit(1);
}

const nextApp = next({
  dir: ".", // base directory where everything is, could move to src later
  dev: true,
});

nextApp
  .prepare()
  .then(async () => {
    // This is an express app.
    const expressApp = express();

    const server = https.createServer(httpsOptions, expressApp);

    // Set up the proxy.
    const { createProxyMiddleware } = require("http-proxy-middleware");
    Object.keys(devProxy).forEach(function (context) {
      expressApp.use(context, createProxyMiddleware(devProxy[context]));
    });

    // Default catch-all handler to allow Next.js to handle all other routes
    const handle = nextApp.getRequestHandler();
    expressApp.all("*", (req, res) => handle(req, res));

    server.listen(port, (err) => {
      if (err) {
        throw err;
      }
      console.log(`> Ready on https://localhost:${port} [development]`);
    });
  })
  .catch((err) => {
    console.log("An error occurred, unable to start the server");
    console.log(err);
  });
