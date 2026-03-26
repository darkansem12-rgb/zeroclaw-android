#!/bin/bash
#==============================================================================
# ZeroClaw Android Run Script for Termux
#==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$SCRIPT_DIR/target/release/zeroclaw"

if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found! Run termux-build.sh first."
    exit 1
fi

echo "=============================================="
echo "  ZeroClaw - Starting..."
echo "=============================================="
echo ""
echo "Access URL: http://127.0.0.1:42617"
echo "Press Ctrl+C to stop"
echo ""

# Run the daemon
"$BINARY" daemon
