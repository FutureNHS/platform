require("dotenv").config();

/**
 * @param {string} name
 * @returns {string}
 */
function env(name) {
  const value = process.env[name];
  if (!value) {
    console.error(`${name} is not set. Please add it to .env`);
    process.exit(1);
  }
  return value;
}

module.exports = { env };
