# Automated Testing

## Setup

To get started, you should have [node](https://nodejs.org/en/) and [npm](https://www.npmjs.com/get-npm) installed. If you don't have it already, install Yarn `npm install -g yarn`.

Run `yarn` in the `/test/` directory to install dependencies.
Add your BrowserStack username and access key to the .env file.

Access key can be found by going to the [BrowserStack Automate Dashboard](https://automate.browserstack.com/dashboard/v2/) and clicking on the ACCESS KEY dropdown.

Install the BrowserStack Local executable `brew cask install browserstacklocal`.

## Running Tests

### Local Browser

If you wish to test locally, before running the Cypress or Internet Explorer tests you will need to have the frontend server running (the API server is only necessary for the hello_world_rust.js Cypress test at this point).

[Local API server instructions](../hello-world/README.md)

[Local frontend instructions](../frontend)

Currently, only Cypress tests will run in a local browser.

To run tests, execute the desired command from the `/test/` directory:

- a local server: `yarn cypress:local`.
- a dev cluster: `yarn cypress:dev <dev cluster name>` i.e. `yarn cypress:dev foo`.
- prod: `yarn cypress:prod`.

This will open the test runner. You can then select a test you want to run and which browser (Chrome/Firefox/Chromium Edge) you would like to run it in.

Workspaces tests currently only work against Prod. For local you need to either

- manually create a Workspace named "Selenium Testing" in your local/dev cluster environment
- For local the name must match what is set in your .env file against "TEST_WORKSPACE_NAME"
- In prod, it is explicitly set to "Selenium Testing"
  OR
- run script
  ```bash
  ./workspace-service/scripts/create-workspace-if-needed.sh
  ```

### BrowserStack

Before running local tests on BrowserStack make sure the BrowserStack Local executable is [installed](#setup).
Execute `yarn local` (from the /test/ directory) to launch the executable before running any of the following tests.

#### Cypress

##### Cypress Configuration

Before running Cypress on BrowserStack, duplicate and edit the browserstack.json example, adding your BrowserStack username and access key. Check the [Setup](#setup) section for instructions on getting your access key.

You can also modify which browsers you would like to run the tests in by modifying the `"browsers"` object in the browserstack.json file. See the BrowserStack [browsers and OS](https://www.browserstack.com/docs/automate/cypress/browsers-and-os) documentation for more information.

##### Run tests

To run tests, execute the desired command from the `/test/` directory:

- a local server: `yarn cypress-bs:local`.
- a dev cluster: `yarn cypress-bs:dev <dev cluster name>` i.e. `yarn cypress-bs:dev foo`.
- prod: `yarn cypress-bs:prod`.

#### Internet Explorer

To run tests, execute the desired command from the `/test/` directory:

- a local server: `yarn internet-explorer-bs:local`.
- a dev cluster: `yarn internet-explorer-bs:dev <dev cluster name>` i.e. `yarn internet-explorer-bs:dev foo`.
- prod: `yarn internet-explorer-bs:prod`.
