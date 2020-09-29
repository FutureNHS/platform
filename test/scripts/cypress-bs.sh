#!/bin/bash

# This is a delicious hack because BrowserStack doesn't let us specify a config file

set -eu

REPO_ROOT="$(git rev-parse --show-toplevel)"

CONFIG_FILE="${1:?"Please enter a cypress-*.json OR your name"}"

if [[ "$CONFIG_FILE" == *.json ]]; then
	cp $CONFIG_FILE $REPO_ROOT/test/cypress.json
else
	echo "making dev config file"
	NAME=$CONFIG_FILE
	$REPO_ROOT/test/scripts/make-dev-config.sh $NAME
fi

cd $REPO_ROOT/test

BROWSERSTACK_USERNAME=$(grep BROWSERSTACK_USERNAME .env | xargs)
BROWSERSTACK_USERNAME=${BROWSERSTACK_USERNAME#*=}
export BROWSERSTACK_USERNAME
BROWSERSTACK_ACCESS_KEY=$(grep BROWSERSTACK_ACCESS_KEY .env | xargs)
BROWSERSTACK_ACCESS_KEY=${BROWSERSTACK_ACCESS_KEY#*=}
export BROWSERSTACK_ACCESS_KEY

# Locally we can load environment variables from the .env file with a Cypress
# plugin. The .env file is not uploaded to BrowserStack, though. So we need to
# parse and specify them here.
TEST_LOGIN_EMAIL_ADDRESS="$(grep TEST_LOGIN_EMAIL_ADDRESS .env | xargs)"
TEST_LOGIN_EMAIL_ADDRESS="${TEST_LOGIN_EMAIL_ADDRESS#*=}"
TEST_LOGIN_PASSWORD="$(grep TEST_LOGIN_PASSWORD .env | xargs)"
TEST_LOGIN_PASSWORD="${TEST_LOGIN_PASSWORD#*=}"
TEST_WORKSPACE_ID="$(grep TEST_WORKSPACE_ID .env | xargs)"
TEST_WORKSPACE_ID="${TEST_WORKSPACE_ID#*=}"
TEST_WORKSPACE_NAME="$(grep TEST_WORKSPACE_NAME .env | xargs)"
TEST_WORKSPACE_NAME="${TEST_WORKSPACE_NAME#*=}"

jq \
	--null-input \
	--arg TEST_LOGIN_EMAIL_ADDRESS "$TEST_LOGIN_EMAIL_ADDRESS" \
	--arg TEST_LOGIN_PASSWORD "$TEST_LOGIN_PASSWORD" \
	--arg TEST_WORKSPACE_ID "$TEST_WORKSPACE_ID" \
	--arg TEST_WORKSPACE_NAME "$TEST_WORKSPACE_NAME" \
	'{
    "TEST_LOGIN_EMAIL_ADDRESS": $TEST_LOGIN_EMAIL_ADDRESS,
    "TEST_LOGIN_PASSWORD": $TEST_LOGIN_PASSWORD,
    "TEST_WORKSPACE_ID": $TEST_WORKSPACE_ID,
    "TEST_WORKSPACE_NAME": $TEST_WORKSPACE_NAME
  }' >cypress.env.json

yarn browserstack-cypress run
rm cypress.json cypress.env.json
