#!/usr/bin/env bash
set -euo pipefail

echo "building rust workspace"
cargo build --workspace --release

echo "building typescript sdk"
( cd sdk/typescript && npm run build )

echo "done"
