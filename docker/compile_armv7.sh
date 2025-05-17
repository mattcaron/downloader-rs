#!/usr/bin/env bash

set -euo pipefail

cd "${0%/*}/.."

export DOCKER_BUILDKIT=1
docker build \
    --platform linux/arm/v7 \
    -f docker/Dockerfile.armv7 \
    --target dist \
    --output=type=local,dest=./dist \
    .
