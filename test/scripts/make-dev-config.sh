#!/bin/bash

set -eu

REPO_ROOT="$(git rev-parse --show-toplevel)"

NAME="${1:?"Please enter your name as first argument"}"

cat $REPO_ROOT/test/cypress-prod.json |
	sed "s/beta.future.nhs.uk/fnhs-dev-$NAME.westeurope.cloudapp.azure.com/" \
		>$REPO_ROOT/test/cypress.json
