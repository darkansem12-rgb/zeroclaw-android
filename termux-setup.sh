#!/bin/bash
#==============================================================================
# ZeroClaw Android Setup Script for Termux
#==============================================================================

set -e

echo "=============================================="
echo "  ZeroClaw - Termux Setup"
echo "=============================================="
echo ""

# Check if we're in Termux
if [ ! -d "$PREFIX" ]; then
    echo "❌ This script must be run in Termux!"
    exit 1
fi

echo "📦 Updating packages..."
pkg update -y

echo "📦 Installing Rust and dependencies..."
pkg install -y rust clang make git wget unzip curl

echo "🔧 Configuring Rust for Android/ARM64..."
# Add ARM64 target if needed (for native Termux compilation)
rustup target add aarch64-unknown-linux-gnu 2>/dev/null || true

echo ""
echo "=============================================="
echo "  ✅ Setup Complete!"
echo "=============================================="
echo ""
echo "Next steps:"
echo "  1. Run: ./termux-build.sh"
echo "  2. Wait for compilation (~30-60 min)"
echo "  3. Run: ./termux-run.sh"
echo ""
