#!/usr/bin/env bash

set -e

docker build -t test_lib -f Dockerfile_test .
docker run --rm test_lib 

