#!/usr/bin/env bash
# exit on error
set -o errexit

# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Build the application
cargo build --release

# Copy the binary to a known location
cp target/release/ai-tutor ./ai-tutor
