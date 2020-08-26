const withImages = require("next-images");
const isProd = process.env.NODE_ENV === "production";

module.exports = withImages({
  inlineImageLimit: 0,
  assetPrefix: isProd ? "https://beta.future.nhs.uk" : "",
});
