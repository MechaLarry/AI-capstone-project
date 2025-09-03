#!/bin/bash
# Setup script for the AI Tutor project

echo "🦀 Setting up AI Tutor project..."

# Create necessary directories
mkdir -p static tests examples src/{handlers,services,models,utils}

# Copy environment template
if [ ! -f .env ]; then
    cp env.example .env
    echo "📝 Created .env file - please add your API keys"
fi

# Install dependencies
echo "📦 Installing Rust dependencies..."
cargo build

echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Edit .env file and add your API key"
echo "2. Run: cargo run"
echo "3. Open: http://localhost:3000"
