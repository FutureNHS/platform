#!/bin/bash

set -eux0 pipefail

ENVIRONMENT="${1:?"Please specify your email as the first parameter, e.g. jane.doe@red-badger.com"}"

TEMPLATE='{
  "traits": {
    "email": "EMAIL"
  }
}'

EMAIL=david.laban+curl@red-badger.com

curl --header "Content-Type: application/json" \
	--request POST \
	--data "$(echo $TEMPLATE | sed s/EMAIL/$EMAIL/g)" \
	http://kratos-admin.kratos/identities
