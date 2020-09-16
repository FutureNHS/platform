require("dotenv").config();

const webdriver = require("selenium-webdriver");
const assert = require("assert");

const userName = process.env.BROWSERSTACK_USERNAME;
const accessKey = process.env.BROWSERSTACK_ACCESS_KEY;

const baseUrl = process.env.IE_BASE_URL;
const browserstackURL = `https://${userName}:${accessKey}@hub-cloud.browserstack.com/wd/hub`;

describe("Page loads", function () {
  this.timeout(15000);

  const capabilities = {
    os: "Windows",
    os_version: "10",
    browserName: "IE",
    browser_version: "11",
    "browserstack.local": "true",
    "browserstack.console": "errors",

    name: "IE Hello World",
  };
  const driverPromise = new webdriver.Builder()
    .usingServer(browserstackURL)
    .withCapabilities(capabilities)
    .build();

  it("should render hype page", async () => {
    const driver = await driverPromise;
    const expected = "The new Future is coming...";

    await driver.get(baseUrl);
    const p = await driver.findElement(webdriver.By.css("p"));
    const result = await p.getText();
    assert.equal(result, expected);
  });

  after(function () {
    driverPromise.quit();
  });
});
