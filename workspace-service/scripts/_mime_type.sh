#!/bin/bash

set -euo pipefail

_get_mime_type() {
	FILE_NAME="$1"
	if [[ "$FILE_NAME" == *.csv ]]; then
		echo "text/csv"
	else
		file --brief --mime-type "$FILE_NAME"
	fi
}
