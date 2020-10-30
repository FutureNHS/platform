#!/bin/bash

set -euo pipefail

cd $(dirname $0)

USAGE="

USAGE: $(basename $0) (dev-\$FNHSNAME|local) WORKSPACE_TITLE FOLDER_TITLE FILE_NAME

"

. ./_context.sh
. ./_mime_type.sh

ENVIRONMENT="${1:?"${USAGE}Please specify your environment name as the first parameter, e.g. dev-jane or local"}"
WORKSPACE_TITLE=${2:?"${USAGE}Please give workspace title as second parameter."}
FOLDER_TITLE=${3:?"${USAGE}Please give folder title as third parameter."}
FILE_NAME=${4:?"${USAGE}Please provide a file name, e.g. data-file.xlsx."}
FILE_TYPE=$(_get_mime_type "./files/$FILE_NAME")
FILE_TITLE=$(echo "$FILE_NAME" | sed -e 's/[.][^.]*$//')
WORKSPACE_SERVICE_GRAPHQL_ENDPOINT="$(_verify_environment_and_get_graphql_endpoint $ENVIRONMENT)"

folder=$(./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "$FOLDER_TITLE")
if [ $folder = "null" ]; then
	echo "Something went wrong finding/creating your folder"
	exit 1
fi

body=$(
	jq \
		--null-input \
		--arg folder "$folder" \
		'{
		"query": "query FilesByFolder($folder: ID!) { filesByFolder(folder: $folder) { title, id } }",
		"variables": {
			"folder": $folder,
		}
	}'
)
existing_files=$(
	curl \
		--silent \
		--show-error \
		-XPOST \
		$WORKSPACE_SERVICE_GRAPHQL_ENDPOINT \
		-H 'Content-Type: application/json' \
		-d "$body"
)

found=$(
	echo "$existing_files" |
		jq \
			-r \
			--arg title "$FILE_TITLE" \
			'.data.filesByFolder | map(select(.title == $title))[0].id'
)
if [ "$found" != "null" ]; then
	echo $found
	exit 0
fi

# 1. Get file URL
file_upload_url=$(
	curl \
		--silent \
		--show-error \
		-XPOST \
		$WORKSPACE_SERVICE_GRAPHQL_ENDPOINT \
		-H 'Content-Type: application/json' \
		-d '{ "query": "{ fileUploadUrls(count: 1) }" }' |
		jq -r '.data.fileUploadUrls[0]'
)

# 2. PUT file
curl \
	--fail \
	--silent \
	--show-error \
	-X PUT \
	-H 'x-ms-version: 2015-02-21' \
	-H "x-ms-date: $(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
	-H 'x-ms-blob-type: BlockBlob' \
	--upload-file "./files/$FILE_NAME" \
	"$file_upload_url"

# 3. CreateFile
body=$(
	jq \
		--null-input \
		--arg folder "$folder" \
		--arg title "$FILE_TITLE" \
		--arg description "$FILE_NAME" \
		--arg folder "$FILE_NAME" \
		--arg fileName "$FILE_NAME" \
		--arg fileType "$FILE_TYPE" \
		--arg temporaryBlobStoragePath "$file_upload_url" \
		'{
			"query": "mutation CreateFile($title: String!, $description: String!, $folder: String!, $fileName: String!, $fileType: String!, $temporaryBlobStoragePath: String!) { createFile(newFile: { title: $title,  description: $description, folder: $folder, fileName: $fileName, fileType: $fileType, temporaryBlobStoragePath: $temporaryBlobStoragePath }) { id } }",
			"variables": {
				"folder": $folder,
				"title": $title,
				"description": $description,
				"folder": $folder,
				"fileName": $fileName,
				"fileType": $fileType,
				"temporaryBlobStoragePath": $temporaryBlobStoragePath,
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
		-H 'X-User-Auth-ID: feedface-0000-0000-0000-000000000000' \
		-d "$body"
)
id=$(echo "$response" | jq -r '.data.createFile.id')
if [ $id = "null" ]; then
	echo "something went wrong! $FILE_NAME, $FILE_TYPE, $response"
	exit 1
fi

echo $id
