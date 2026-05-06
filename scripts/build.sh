#!/bin/bash
# Build the release binary
set -e

echo "=== Building release binary ==="
cd "$(dirname "$0")/.."
cargo build --release
echo "=== Build complete: target/release/goblin_slop ==="