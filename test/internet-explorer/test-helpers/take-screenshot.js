const fs = require("fs");

/**
 * @param {webdriver.WebDriver} driver
 * @param {string} filename
 */
async function takeScreenshot(driver, filename) {
  await fs.promises.writeFile(
    filename,
    await driver.takeScreenshot(),
    "base64"
  );
  console.log(`Screenshot captured to ${filename}.`);
}

module.exports = { takeScreenshot };
