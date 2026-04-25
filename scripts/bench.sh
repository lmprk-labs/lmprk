#!/usr/bin/env bash
set -euo pipefail

echo "running release-mode integration tests as a stand-in for cargo bench"
cargo test --workspace --release -- --nocapture
