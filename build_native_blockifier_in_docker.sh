#!/bin/env bash
set -e

docker_image_name=blockifier-ci

(
    cd scripts
    docker build . -t ${docker_image_name} --file blockifier.Dockerfile
)

docker run \
    --rm \
    --net host \
    -e CARGO_HOME=${HOME}/.cargo \
    -u $UID \
    -v /tmp:/tmp \
    -v "${HOME}:${HOME}" \
    --workdir ${PWD} \
    ${docker_image_name} \
    scripts/build_native_blockifier.sh