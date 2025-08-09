#!/bin/bash

# Build script for Solana USDC Indexer

set -e

echo "🏗️  Building Solana USDC Indexer..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build in release mode
echo "⚙️  Building in release mode..."
cargo build --release

# Check if binary was created
if [ -f "target/release/indexer" ]; then
    echo "✅ Build successful!"
    echo "📍 Binary location: target/release/indexer"
    echo ""
    echo "🚀 Usage examples:"
    echo "  ./target/release/indexer --wallet=7cMEhpt9y3inBNVv8fNnuaEbx7hKHZnLvR1KWKKxuDDU"
    echo "  ./target/release/indexer --wallet=YOUR_WALLET --hours=48 --output=json"
    echo ""
    echo "📖 Run './target/release/indexer --help' for more options"
else
    echo "❌ Build failed - binary not found"
    exit 1
fi
