#!/bin/bash

# Configuration
BUCKET_URL="https://mainnet-ceremony.s3.amazonaws.com"
BUCKET_FOLDER="valid"
XML_RESULT="bucket_contents.xml"
DOWNLOAD_FOLDER="./contributions"

# Ensure dependencies are installed
command -v curl >/dev/null 2>&1 || { echo >&2 "curl is required but it is not installed. Aborting."; exit 1; }
command -v xmllint >/dev/null 2>&1 || { echo >&2 "xmllint is required but it is not installed. Aborting."; exit 1; }
command -v shasum >/dev/null 2>&1 || { echo >&2 "shasum is required but it is not installed. Aborting."; exit 1; }
command -v awk >/dev/null 2>&1 || { echo >&2 "awk is required but it is not installed. Aborting."; exit 1; }

# Fetch the list of objects in the S3 bucket
echo "Fetching list of objects from S3 bucket..."
curl -sL "$BUCKET_URL/?list-type=2&prefix=${BUCKET_FOLDER}/" -o "$XML_RESULT"

# Parse the XML to extract the keys and find the file with the highest index
keys=$(xmllint --xpath '//*[local-name()="Contents"]/*[local-name()="Key"]/text()' "$XML_RESULT" | tr ' ' '\n')

# Clean up XML result file
rm -f "$XML_RESULT"

# Filter keys that have ".csrs" extension and sort by filename
contributions=$(echo "$keys" | grep '.csrs' | sort )

# By default, the last contribution is downloaded
# If $1 is "all", then all contributions will be downloaded
# If instead $1 is a positive integer n, then only the last n contributions will be downloaded
if [[ "$1" =~ ^[0-9]+$ ]]; then
    contributions=$(echo "$contributions" | tail -n $1 )
elif [ "$1" != "all" ]; then
    contributions=$(echo "$contributions" | tail -n 1 )
fi

if [ -z "$contributions" ]; then
    echo "No contributions in The S3 bucket match the provided criteria."
    exit 1
fi

# Convert the contributions string into an array 
IFS=$'\n' read -r -d '' -a contributions <<< "$contributions"

# We create the download folder
mkdir -p $DOWNLOAD_FOLDER

# We download each contribution
contribution_names=()
for contribution in "${contributions[@]}"; do
    contribution_name=${contribution##*/} 
    contribution_names+=("${contribution_name}")
    echo "Downloading contribution ${contribution_name}"
    curl -C - -o "${DOWNLOAD_FOLDER}/${contribution_name}" "${BUCKET_URL}/${contribution}" 
done

# We hash downloaded contributions
echo "Computing hashes for downloaded contributions.."
for contribution in "${contribution_names[@]}"; do
    shasum -a 256 ${DOWNLOAD_FOLDER}/${contribution} | awk -v f="$contribution" '{print f, $1}'
done

echo -e "\033[1mBefore proceeding, please visit \033[4mhttps://ceremony.zircuit.com\033[0m \033[1mand verify that all contribution hashes match"'!'"\033[0m"