require("dotenv").config();

const { env } = require("./env-vars");

const webdriver = require("selenium-webdriver");

const userName = env("BROWSERSTACK_USERNAME");
const accessKey = env("BROWSERSTACK_ACCESS_KEY");

const baseUrl = env("IE_BASE_URL");
const browserstackURL = `https://${userName}:${accessKey}@hub-cloud.browserstack.com/wd/hub`;

const TEST_LOGIN_EMAIL_ADDRESS = env("TEST_LOGIN_EMAIL_ADDRESS");
const TEST_LOGIN_PASSWORD = env("TEST_LOGIN_PASSWORD");

/**
 * @param {webdriver.WebDriver} driver
 * @param {string} targetUrl
 */
async function loginIfNeeded(driver, targetUrl) {
  await driver.get(targetUrl);
  const currentUrl = await driver.getCurrentUrl();
  if (currentUrl === targetUrl) {
    return;
  }

  const emailInput = await driver.findElement(webdriver.By.css("#email"));
  const passwordInput = await driver.findElement(webdriver.By.css("#password"));
  const signInButton = await driver.findElement(webdriver.By.css("#next"));

  await emailInput.sendKeys(TEST_LOGIN_EMAIL_ADDRESS);
  await passwordInput.sendKeys(TEST_LOGIN_PASSWORD);
  await signInButton.click();

  await driver.wait(webdriver.until.urlContains(baseUrl));
}

module.exports = { loginIfNeeded };
