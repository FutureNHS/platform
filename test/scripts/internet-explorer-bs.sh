#!/bin/bash

set -eu

REPO_ROOT="$(git rev-parse --show-toplevel)"

ENV="${1:?"Please enter which env you want to test in (local/dev/prod) as the first argument."}"

if [[ "$ENV" == local ]]; then
	IE_BASE_URL="http://localhost:3000"
elif [[ "$ENV" == dev ]]; then
	NAME="${2:?"If you wish to use a dev cluster, please enter your name as the second argument."}"
	IE_BASE_URL="https://fnhs-dev-$NAME.westeurope.cloudapp.azure.com/"
elif [[ "$ENV" == prod ]]; then
	IE_BASE_URL="https://beta.future.nhs.uk/"
fi

cd $REPO_ROOT/test && IE_BASE_URL=$IE_BASE_URL yarn mocha ./internet-explorer/internet_explorer_hello_world.js
