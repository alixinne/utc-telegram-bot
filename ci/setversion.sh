#!/bin/bash

set -xeuo pipefail

sed -i '/utc-telegram-bot/,/version =/{s/version = ".*"/version = "'"$1"'"/}' Cargo.toml Cargo.lock
