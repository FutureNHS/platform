/// <reference types="cypress" />
// ***********************************************************
// This example plugins/index.js can be used to load plugins
//
// You can change the location of this file or turn off loading
// the plugins file with the 'pluginsFile' configuration option.
//
// You can read more here:
// https://on.cypress.io/plugins-guide
// ***********************************************************

/**
 * @type {Cypress.PluginConfig}
 */
module.exports = (_on, config) => {
  try {
    const dotenv = require("dotenv");
    const envResult = dotenv.config();
    if (envResult.error) {
      throw envResult.error;
    }

    Object.assign(config.env, envResult.parsed);
  } catch (err) {
    console.warn("Could not load environment variables from .env file.", err);
  }

  return config;
};
