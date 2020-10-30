#!/bin/bash

set -euo pipefail

cd $(dirname $0)

USAGE="

USAGE: $(basename $0) (dev-\$FNHSNAME|local) WORKSPACE_TITLE FOLDER_TITLE [FOLDER_DESCRIPTION]

"

ENVIRONMENT="${1:?"${USAGE}Please specify your environment name as the first parameter, e.g. dev-jane or local"}"
WORKSPACE_TITLE=${2:?"${USAGE}Please give workspace title as second parameter."}
FOLDER_TITLE=${3:?"${USAGE}Please give folder title as third parameter."}
FOLDER_DESCRIPTION=${4:-"Test folder with title $FOLDER_TITLE"}

. ./_context.sh

WORKSPACE_SERVICE_GRAPHQL_ENDPOINT="$(_verify_environment_and_get_graphql_endpoint $ENVIRONMENT)"

workspace=$(./create-workspace-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE")
if [ $workspace = "null" ]; then
	echo "Something went wrong finding/creating your workspace"
	exit 1
fi

body=$(
	jq \
		--null-input \
		--arg workspace "$workspace" \
		'{
		"query": "query FoldersByWorkspace($workspace: ID!) { foldersByWorkspace(workspace: $workspace) { title, id } }",
		"variables": {
			"workspace": $workspace,
		}
	}'
)
existing_folders=$(
	curl \
		--silent \
		--show-error \
		-XPOST \
		$WORKSPACE_SERVICE_GRAPHQL_ENDPOINT \
		-H 'Content-Type: application/json' \
		-d "$body"
)

found=$(
	echo "$existing_folders" |
		jq \
			-r \
			--arg title "$FOLDER_TITLE" \
			'.data.foldersByWorkspace | map(select(.title == $title))[0].id'
)
if [ "$found" != "null" ]; then
	echo $found
	exit 0
fi

body=$(
	jq \
		--null-input \
		--arg workspace "$workspace" \
		--arg title "$FOLDER_TITLE" \
		--arg description "$FOLDER_DESCRIPTION" \
		'{
			"query": "mutation CreateFolder($workspace: ID!, $title: String!, $description: String!) { createFolder(newFolder: { workspace: $workspace, title: $title,  description: $description }) { id } }",
			"variables": {
				"workspace": $workspace,
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
		-H 'Content-Type: application/json' \
		-d "$body"
)
id=$(echo "$response" | jq -r '.data.createFolder.id')
if [ $id = "null" ]; then
	echo "something went wrong! $response"
	exit 1
fi

echo $id
