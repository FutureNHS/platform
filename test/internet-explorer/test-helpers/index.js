const { capabilities } = require("./capabilities");
const { env } = require("./env-vars");
const { loginIfNeeded } = require("./login-if-needed");
const { takeScreenshot } = require("./take-screenshot");

module.exports = { capabilities, env, loginIfNeeded, takeScreenshot };
