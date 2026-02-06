#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Running host test to verify keccak chip usage ==="
cargo run --release --bin verify-keccak-chips
echo "=== Host test passed: Keccak chips verified ==="
