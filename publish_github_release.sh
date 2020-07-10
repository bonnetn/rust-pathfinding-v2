#!/bin/bash
set -e

# Copied from:
# https://medium.com/@systemglitch/continuous-integration-with-jenkins-and-github-release-814904e20776#:~:text=Go%20to%20Manage%20Jenkins%20%2D%3E%20Manage,name%20as%20the%20git%20repository.
token=$1

# Publish on github
echo "Publishing on Github..."

# Get the last tag name
tag=$(git describe --tags)

# Get the full message associated with this tag
message="$(git for-each-ref refs/tags/$tag --format='%(contents)')"

# Get the title and the description as separated variables
name=$(echo "$message" | head -n1)

description=$(echo "$message" | tail -n +3)
description=$(echo "$description" | sed -z 's/\n/\\n/g') # Escape line breaks to prevent json parsing problems

# Create a release
release=$(curl -XPOST -H "Authorization:token $token" --data "{\"tag_name\": \"$tag\", \"target_commitish\": \"master\", \"name\": \"$name\", \"body\": \"$description\", \"draft\": false, \"prerelease\": true}" https://api.github.com/repos/bonnetn/rust-pathfinding-v2/releases)

# Extract the id of the release from the creation response
id=$(echo "$release" | sed -n -e 's/"id":\ \([0-9]\+\),/\1/p' | head -n 1 | sed 's/[[:blank:]]//g')


for file in "${@:2}"
do
  # Upload the artifact
  echo "Uploading $file"
  result=$(curl -XPOST -H "Authorization:token $token" -H "Content-Type:application/octet-stream" --data-binary "@$file" https://uploads.github.com/repos/bonnetn/rust-pathfinding-v2/releases/$id/assets?name=$file)
done
