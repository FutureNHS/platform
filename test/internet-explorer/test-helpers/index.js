const { env } = require("./env-vars");
const { loginIfNeeded } = require("./login-if-needed");
const { takeScreenshot } = require("./take-screenshot");

module.exports = { env, loginIfNeeded, takeScreenshot };
