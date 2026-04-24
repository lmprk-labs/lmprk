#!/usr/bin/env bash
set -euo pipefail

echo "running rust tests"
cargo test --workspace

echo "running typescript tests"
( cd sdk/typescript && npm test )
