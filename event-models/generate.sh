#!/bin/bash

# https://stackoverflow.com/a/1482133
cd $(dirname "$0")

brew install openapi-generator

rm -r rust/
openapi-generator generate -i schema.yaml -g rust -c config-rust.yaml -o rust/
rm -r ts/
openapi-generator generate -i schema.yaml -g typescript-fetch -c config-ts.yaml -o ts/
