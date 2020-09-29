require("dotenv").config();

const webdriver = require("selenium-webdriver");
const assert = require("assert");

const { capabilities, env } = require("./test-helpers");

const userName = env("BROWSERSTACK_USERNAME");
const accessKey = env("BROWSERSTACK_ACCESS_KEY");

const baseUrl = env("IE_BASE_URL");
const browserstackURL = `https://${userName}:${accessKey}@hub-cloud.browserstack.com/wd/hub`;

describe("Hype page loads", function () {
  this.timeout(15000);

  const driverPromise = new webdriver.Builder()
    .usingServer(browserstackURL)
    .withCapabilities(capabilities("Hype Page"))
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
