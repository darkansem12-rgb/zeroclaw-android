#!/bin/bash
#==============================================================================
# ZeroClaw Android Build Script for Termux
#==============================================================================

set -e

echo "=============================================="
echo "  ZeroClaw - Building..."
echo "=============================================="
echo ""

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🔨 Starting Rust compilation..."
echo "   This may take 30-60 minutes on first run."
echo ""

# Build with memory-efficient flags
export CARGO_BUILD_JOBS=1
export RUSTFLAGS="-C opt-level=z"

cargo build --release

echo ""
echo "=============================================="
echo "  ✅ Build Complete!"
echo "=============================================="
echo ""

# Find the binary
BINARY="$SCRIPT_DIR/target/release/zeroclaw"
if [ -f "$BINARY" ]; then
    echo "Binary location: $BINARY"
    ls -lh "$BINARY"
    echo ""
    echo "Next step: ./termux-run.sh"
else
    echo "❌ Build failed - binary not found"
    exit 1
fi
