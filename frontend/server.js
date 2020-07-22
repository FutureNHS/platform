/// This is taken from the next.js reverse proxy example, and should
/// only be used for local development.
/// https://github.com/vercel/next.js/tree/master/examples/with-custom-reverse-proxy
/* eslint-disable no-console */
const express = require("express");
const next = require("next");

const devProxy = {
  "/hello": {
    target: "http://hello-world.hello-world/",
    changeOrigin: true,
  },
  "/.ory/kratos/public/self-service/browser/flows/login": {
    target: "http://127.0.0.1:4433",
    pathRewrite: { "/.ory/kratos/public": "" },
    changeOrigin: false,
  },
  // another one that hits ory/kratos refer to http-proxy-middleware docs to understand the format
};

const port = parseInt(process.env.PORT, 10) || 4455;
if (process.env.NODE_ENV !== "development") {
  console.error(
    "This server should only be used by `yarn dev`, and should never be used in production."
  );
  process.exit(1);
}

const app = next({
  dir: ".", // base directory where everything is, could move to src later
  dev: true,
});

const handle = app.getRequestHandler();

let server;
app
  .prepare()
  .then(() => {
    server = express();

    // Set up the proxy.
    const { createProxyMiddleware } = require("http-proxy-middleware");
    Object.keys(devProxy).forEach(function (context) {
      server.use(context, createProxyMiddleware(devProxy[context]));
    });

    // Default catch-all handler to allow Next.js to handle all other routes
    server.all("*", (req, res) => handle(req, res));

    server.listen(port, (err) => {
      if (err) {
        throw err;
      }
      console.log(`> Ready on http://localhost:${port} [development]`);
    });
  })
  .catch((err) => {
    console.log("An error occurred, unable to start the server");
    console.log(err);
  });
