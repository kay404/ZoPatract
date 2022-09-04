#!/bin/bash

# Exit if any subcommand fails
set -e

export ZOPATRACT_STDLIB=$PWD/zopatract_stdlib/stdlib
cargo +nightly build -p zopatract_cli --release
