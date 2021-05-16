#!/usr/bin/env bash

set -e

./target/release/bunbi-node purge-chain --dev -y
./target/release/bunbi-node --dev
