#!/bin/bash
# Quick install script for ZeroClaw on Android/Termux
# Run this in Termux after cloning the repository

set -e

echo "🛡️ ZeroClaw Android Installer"
echo "================================"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if running in Termux
if [ -z "$TERMUX_VERSION" ] && [ -z "$TERMUX_API_VERSION" ]; then
    echo -e "${RED}⚠️  Warning: Not running in Termux${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Step 1: Update packages
echo -e "${YELLOW}📦 Updating packages...${NC}"
pkg update -y

# Step 2: Install dependencies
echo -e "${YELLOW}🔧 Installing dependencies...${NC}"
pkg install -y rust clang lld cmake git openssl

# Step 3: Setup storage
echo -e "${YELLOW}💾 Setting up storage...${NC}"
if [ ! -d "$HOME/storage" ]; then
    termux-setup-storage
fi

# Step 4: Create directories
echo -e "${YELLOW}📁 Creating directories...${NC}"
mkdir -p $HOME/.zeroclaw
mkdir -p $HOME/.zeroclaw/logs
mkdir -p $HOME/zeroclaw-workspace

# Step 5: Build ZeroClaw
echo -e "${YELLOW}🔨 Building ZeroClaw...${NC}"
export RUSTFLAGS="-C linker=clang"
export CC=clang
export CXX=clang++

cargo build --release

# Step 6: Install binary
echo -e "${YELLOW}📥 Installing binary...${NC}"
cp target/release/zeroclaw $PREFIX/bin/zeroclaw
chmod +x $PREFIX/bin/zeroclaw

# Step 7: Setup configuration
echo -e "${YELLOW}⚙️  Setting up configuration...${NC}"

# Copy config files
if [ -f ".env.android" ]; then
    cp .env.android $HOME/.zeroclaw/.env
    echo -e "${GREEN}✓ Created .env${NC}"
fi

if [ -f "config.android.toml" ]; then
    cp config.android.toml $HOME/.zeroclaw/config.toml
    echo -e "${GREEN}✓ Created config.toml${NC}"
fi

if [ -f "protection.toml" ]; then
    cp protection.toml $HOME/.zeroclaw/
    echo -e "${GREEN}✓ Created protection.toml${NC}"
fi

# Step 8: Verify installation
echo -e "${YELLOW}🔍 Verifying installation...${NC}"
if command -v zeroclaw &>/dev/null; then
    VERSION=$(zeroclaw --version)
    echo -e "${GREEN}✓ ZeroClaw installed: $VERSION${NC}"
else
    echo -e "${RED}✗ Installation failed${NC}"
    exit 1
fi

# Step 9: Create convenience aliases
echo -e "${YELLOW}📝 Creating shell aliases...${NC}"
cat >> $HOME/.bashrc << 'EOF'

# ZeroClaw aliases
alias zc='zeroclaw'
alias zc-status='zeroclaw status'
alias zc-doctor='zeroclaw doctor'
alias zc-logs='cat $HOME/.zeroclaw/actions.log'
alias zc-config='nano $HOME/.zeroclaw/config.toml'

EOF

echo -e "${GREEN}✓ Aliases added to .bashrc${NC}"

# Success message
echo ""
echo -e "${GREEN}🎉 ZeroClaw installed successfully!${NC}"
echo ""
echo "Next steps:"
echo "  1. Setup Ollama or configure another provider"
echo "  2. Run: zeroclaw onboard"
echo "  3. Edit: nano $HOME/.zeroclaw/config.toml"
echo "  4. Start: zeroclaw daemon"
echo ""
echo "Security:"
echo "  - Protection sandbox is ENABLED"
echo "  - Actions logged to: $HOME/.zeroclaw/actions.log"
echo "  - Sensitive paths are blocked"
echo ""
echo "Quick commands:"
echo "  zeroclaw --version     # Check version"
echo "  zeroclaw status        # Check status"
echo "  zeroclaw doctor        # Run diagnostics"
echo "  source $HOME/.bashrc   # Reload shell aliases"
echo ""