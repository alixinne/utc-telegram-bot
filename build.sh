#!/bin/bash

set -eu

DOCKER=podman

$DOCKER build -t vtavernier/utc-telegram-bot-build:latest .
$DOCKER run -it --rm \
	-v $PWD:/src:Z \
	-v $HOME/.cargo:/usr/local/cargo:Z \
	vtavernier/utc-telegram-bot-build:latest \
	/bin/bash -c 'cd /src && cargo build --target x86_64-unknown-linux-gnu --release'
