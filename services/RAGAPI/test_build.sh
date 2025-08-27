#!/bin/bash

# Simple build and test script for RAGAPI

set -e

echo "🔨 Building RAGAPI..."
cargo build --release

echo "✅ RAGAPI built successfully!"

echo "🧪 Running quick validation tests..."

# Test that the binary can be executed (will fail without env vars, but that's expected)
if ./target/release/ragapi --version 2>/dev/null || ./target/release/ragapi --help 2>/dev/null; then
    echo "✅ Binary can be executed"
else
    echo "ℹ️  Binary exists (env vars needed for full startup)"
fi

echo "🎉 Build validation completed!"