require("dotenv").config();

const webdriver = require("selenium-webdriver");
const assert = require("assert");

const { loginIfNeeded } = require("./login");

const userName = process.env.BROWSERSTACK_USERNAME;
const accessKey = process.env.BROWSERSTACK_ACCESS_KEY;

const baseUrl = process.env.IE_BASE_URL;
const browserstackURL = `https://${userName}:${accessKey}@hub-cloud.browserstack.com/wd/hub`;

describe("Creating a workspace and navigating to it", function () {
  this.timeout(15000);

  const capabilities = {
    os: "Windows",
    os_version: "10",
    browserName: "IE",
    browser_version: "11",
    "browserstack.local": "true",
    "browserstack.console": "errors",

    name: "Workspaces",
  };
  const driverPromise = new webdriver.Builder()
    .usingServer(browserstackURL)
    .withCapabilities(capabilities)
    .build();

  it("should render workspace creation page", async () => {
    const targetUrl = `${baseUrl}/admin/create-workspace`;
    const driver = await driverPromise;
    const expected = "Create a workspace";

    await loginIfNeeded(driver, targetUrl);
    await driver.get(targetUrl);
    const h1 = await driver.findElement(webdriver.By.css("h1"));
    const result = await h1.getText();
    assert.equal(result, expected);
  });

  it("should render workspace directory", async () => {
    const targetUrl = `${baseUrl}/workspaces/directory`;
    const driver = await driverPromise;
    const expected = "My workspaces";

    await loginIfNeeded(driver, targetUrl);
    await driver.get(targetUrl);
    const h1 = await driver.findElement(webdriver.By.css("h1"));
    const result = await h1.getText();
    assert.equal(result, expected);
  });

  it("clicking workspace directory item should render workspace", async () => {
    const targetUrl = `${baseUrl}/workspaces/directory`;
    const driver = await driverPromise;
    const expected = process.env.TEST_WORKSPACE_NAME;
    const expectedUrl = `${baseUrl}/workspaces/${process.env.TEST_WORKSPACE_ID}`;

    await loginIfNeeded(driver, targetUrl);
    await driver.get(targetUrl);

    await driver.findElement(webdriver.By.linkText(expected)).click();

    const currentUrl = await driver.getCurrentUrl();
    assert.equal(currentUrl, expectedUrl);

    const h1 = await driver.findElement(webdriver.By.css("h1"));
    const h1Result = await h1.getText();
    assert.equal(h1Result, expected);

    const h2 = await driver.findElement(webdriver.By.css("h2"));
    const h2Result = await h2.getText();
    assert.equal(h2Result, "Most recent items");
  });

  after(function () {
    driverPromise.quit();
  });
});
