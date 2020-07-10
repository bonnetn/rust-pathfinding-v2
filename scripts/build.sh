#!/usr/bin/env bash

set -e

docker build -t build_lib .

mkdir -p build
docker run --rm build_lib tar -cC /artifact . | tar -xC ./build

