const capabilities = (name) => ({
  os: "Windows",
  os_version: "10",
  browserName: "IE",
  browser_version: "11",
  "browserstack.local": process.env.BROWSERSTACK_LOCAL,
  "browserstack.console": "errors",
  name,
});

module.exports = { capabilities };
