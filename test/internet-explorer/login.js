// @ts-check
require("dotenv").config();

const webdriver = require("selenium-webdriver");
const assert = require("assert");

const { capabilities, env, loginIfNeeded } = require("./test-helpers");
const userName = env("BROWSERSTACK_USERNAME");
const accessKey = env("BROWSERSTACK_ACCESS_KEY");

const baseUrl = env("IE_BASE_URL");
const browserstackURL = `https://${userName}:${accessKey}@hub-cloud.browserstack.com/wd/hub`;

describe("Logging in", function () {
  this.timeout(15000);

  const driverPromise = new webdriver.Builder()
    .usingServer(browserstackURL)
    .withCapabilities(capabilities("Login"))
    .build();

  it("should redirect to / after login", async () => {
    const driver = await driverPromise;
    // `/auth/login` *always* needs to login.
    await loginIfNeeded(driver, `${baseUrl}/auth/login`);
    const currentUrl = await driver.getCurrentUrl();
    assert.equal(currentUrl, baseUrl + "/");
  });

  after(function () {
    driverPromise.quit();
  });
});
