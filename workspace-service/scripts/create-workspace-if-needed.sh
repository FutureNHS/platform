#!/bin/bash

set -euo pipefail

cd $(dirname $0)

USAGE="

USAGE: $(basename $0) (dev-\$FNHSNAME|local) WORKSPACE_TITLE [WORKSPACE_DESCRIPTION]

"

ENVIRONMENT="${1:?"${USAGE}Please specify your environment name as the first parameter, e.g. dev-jane or local"}"
WORKSPACE_TITLE=${2:?"${USAGE}Please give workspace title as second parameter."}
WORKSPACE_DESCRIPTION=${3:-"Test workspace with title $WORKSPACE_TITLE"}

. ./_context.sh

WORKSPACE_SERVICE_GRAPHQL_ENDPOINT="$(_verify_environment_and_get_graphql_endpoint $ENVIRONMENT)"

existing_workspaces=$(
	curl \
		--silent \
		--show-error \
		-XPOST \
		$WORKSPACE_SERVICE_GRAPHQL_ENDPOINT \
		-H 'Content-Type: application/json' \
		-d '{"query": "{workspaces { title, id }}"}'
)
found=$(
	echo "$existing_workspaces" |
		jq \
			-r \
			--arg title "$WORKSPACE_TITLE" \
			'.data.workspaces | map(select(.title == $title))[0].id'
)
if [ "$found" != "null" ]; then
	echo $found
	exit 0
fi

body=$(
	jq \
		--null-input \
		--arg title "$WORKSPACE_TITLE" \
		--arg description "$WORKSPACE_DESCRIPTION" \
		'{
			"query": "mutation CreateWorkspace($title: String!, $description: String!) { createWorkspace(newWorkspace: { title: $title,  description: $description }) { id } }",
			"variables": {
				"title": $title,
				"description": $description
			}
		}'
)
response=$(
	curl \
		--silent \
		--show-error \
		-XPOST \
		$WORKSPACE_SERVICE_GRAPHQL_ENDPOINT \
		-H 'x-user-auth-id: feedface-0000-0000-0000-000000000000' \
		-H 'Content-Type: application/json' \
		-d "$body"
)
echo "$response" | jq -r '.data.createWorkspace.id'
