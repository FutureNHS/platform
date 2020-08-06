#!/bin/bash

# https://stackoverflow.com/a/1482133
cd $(dirname "$0")

brew install openapi-generator

openapi-generator generate -i schema.yaml -g rust -c config-rust.yaml -o rust/
openapi-generator generate -i schema.yaml -g typescript-node -c config-ts.yaml -o ts/
