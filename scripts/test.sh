#!/bin/bash
# Test script

echo "🧪 Running tests..."

# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Test with example client (if server is running)
echo "📡 Testing API endpoints..."
cargo run --example test_client

echo "✅ Tests complete!"
