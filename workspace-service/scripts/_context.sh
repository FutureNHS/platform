#!/bin/bash

set -euo pipefail

_verify_environment_and_get_graphql_endpoint() {
	ENVIRONMENT="$1"
	CURRENT_CONTEXT=$(kubectl config current-context)
	if [ "$ENVIRONMENT" = "local" ]; then
		echo http://localhost:3030/graphql
	elif [ "$ENVIRONMENT" = "$CURRENT_CONTEXT" ]; then
		echo http://workspace-service.workspace-service/graphql
	else
		echo "You want to populate:    $ENVIRONMENT" >&2
		echo "Your current content is: $CURRENT_CONTEXT" >&2
		echo "Please change your current context using:" >&2
		echo "    kubectl config use-context $ENVIRONMENT" >&2
		echo "or" >&2
		echo "    az account set --subscription \$SUBSCRIPTION_ID && az aks get-credentials --resource-group=platform-$ENVIRONMENT --name=$ENVIRONMENT" >&2
		echo "Once that is done, please run:" >&2
		echo "    kubefwd services -n workspace-service" >&2
		echo "in another tab and try again." >&2
		exit 1
	fi
}
