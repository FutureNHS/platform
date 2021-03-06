require("dotenv").config();

const { Builder, By } = require("selenium-webdriver");
const assert = require("assert");

const { capabilities, delay, env, loginIfNeeded } = require("./test-helpers");

const userName = env("BROWSERSTACK_USERNAME");
const accessKey = env("BROWSERSTACK_ACCESS_KEY");

const baseUrl = env("IE_BASE_URL");
const browserstackURL = `https://${userName}:${accessKey}@hub-cloud.browserstack.com/wd/hub`;

const TEST_WORKSPACE_NAME = env("TEST_WORKSPACE_NAME");

describe("Creating a workspace and navigating to it", function () {
  this.timeout(15000);

  const driverPromise = new Builder()
    .usingServer(browserstackURL)
    .withCapabilities(capabilities("Workspaces"))
    .build();

  it("should render workspace creation page", async () => {
    const targetUrl = `${baseUrl}/admin/create-workspace`;
    const driver = await driverPromise;
    const expected = "Create a workspace";

    await loginIfNeeded(driver, targetUrl);
    await driver.get(targetUrl);
    const h1 = await driver.findElement(By.css("h1"));
    const result = await h1.getText();
    assert.equal(result, expected);
  });

  it("should render workspace directory", async () => {
    const targetUrl = `${baseUrl}/workspaces/directory`;
    const driver = await driverPromise;
    const expected = "My workspaces";

    await loginIfNeeded(driver, targetUrl);
    await driver.get(targetUrl);
    const h1 = await driver.findElement(By.css("h1"));
    const result = await h1.getText();
    assert.equal(result, expected);
  });

  it("clicking workspace directory item should render workspace", async () => {
    const targetUrl = `${baseUrl}/workspaces/directory`;
    const driver = await driverPromise;
    const expected = TEST_WORKSPACE_NAME;

    await loginIfNeeded(driver, targetUrl);
    await driver.get(targetUrl);

    await driver.findElement(By.linkText(expected)).click();
    await delay(1000);

    const h1 = await driver.findElement(By.css("h1"));
    const h1Result = await h1.getText();
    assert.equal(h1Result, expected);

    const h2 = await driver.findElement(By.css("h2"));
    const h2Result = await h2.getText();
    assert.equal(h2Result, "Most recent items");
  });

  after(function () {
    driverPromise.quit();
  });
});
