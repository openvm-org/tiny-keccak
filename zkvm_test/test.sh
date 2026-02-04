#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== Testing tiny-keccak for circular dependency issues with OpenVM ==="

# Clean previous builds to ensure fresh compilation
echo "Cleaning previous build artifacts..."
cargo clean 2>/dev/null || true

# Build and run the OpenVM program
# This will fail if there are circular dependencies between tiny-keccak
# and openvm-keccak256 since they reference each other
echo "Building and running OpenVM program..."
cargo openvm run

echo "=== Test passed: No circular dependency issues detected ==="
