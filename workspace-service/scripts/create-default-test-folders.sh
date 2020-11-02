#!/bin/bash

set -euo pipefail

cd $(dirname $0)

USAGE="

EXAMPLE USAGES:

$(basename $0) dev-\$FNHSNAME

$(basename $0) local

"

ENVIRONMENT="${1:?"${USAGE}Please specify your environment name as the first parameter, e.g. dev-jane or local"}"
WORKSPACE_TITLE="Selenium Testing"

. ./_context.sh

_verify_environment_and_get_graphql_endpoint $ENVIRONMENT

./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "FutureNHS Case Study Library"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Getting started"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Platform FAQs"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Support"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Get involved"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Enhance your workspace"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Member stories"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Coronavirus (COVID-19)"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Communications resources"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Administration"

./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Strategic Partners"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Uploads"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Workspace Navigation"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Data"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Information and Analysis"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Evidence"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Community, Networks and Learning"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Workspace Insights"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Good News"
./create-folder-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "NHS COVID-19 Data Store"

./create-file-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Data" "Coronavirus Numbers.csv"
./create-file-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Data" "Trust List.docx"
./create-file-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Data" "Infographic.png"
./create-file-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Data" "River.mov"
./create-file-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Data" "Leaflet.pdf"
./create-file-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Data" "Motivational Speech.pptx"
./create-file-if-needed.sh "$ENVIRONMENT" "$WORKSPACE_TITLE" "Data" "Encryption Keys.txt"
