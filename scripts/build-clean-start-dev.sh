#!/usr/bin/env bash

set -e

cargo build --release
./target/release/bunbi-node purge-chain --dev -y
./target/release/bunbi-node --dev
