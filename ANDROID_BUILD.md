# ZeroClaw Android Build Instructions (ARM64)

This guide explains how to build ZeroClaw for Android/Termux with Ollama/TinyLlama support.

## Prerequisites

1. **Android Device Requirements:**
   - Android 5.0+ (API 21+)
   - ARM64 processor (aarch64)
   - At least 2GB RAM recommended
   - 500MB free storage

2. **Termux Installation:**
   - Download Termux from F-Droid (NOT Play Store)
   - URL: https://f-droid.org/packages/com.termux/

## Step 1: Setup Termux Environment

```bash
# Update packages
pkg update && pkg upgrade

# Install required packages
pkg install -y rust clang lld cmake git openssl

# Install build tools
pkg install -y binutils-is-llvm

# Setup storage access
termux-setup-storage

# Create workspace
mkdir -p $HOME/zeroclaw-workspace
mkdir -p $HOME/.zeroclaw
```

## Step 2: Install Ollama (for local LLM)

```bash
# Install Ollama (if available for Termux)
# Note: Ollama may need to be built from source on Termux
# Alternative: Use termux-api or remote Ollama instance

# Option A: Remote Ollama (if you have a server)
# Set OLLAMA_BASE_URL in .env.android

# Option B: Build Ollama from source (advanced)
# See: https://github.com/ollama/ollama/blob/main/docs/development.md

# Download TinyLlama model
ollama pull tinyllama

# Test Ollama
ollama run tinyllama "Hello, are you working?"
```

## Step 3: Build ZeroClaw

### Option A: Build on Device (Recommended for Termux)

```bash
# Clone repository
cd $HOME
git clone https://github.com/zeroclaw-labs/zeroclaw.git
cd zeroclaw

# Checkout your branch
git checkout my_antigravity_mod

# Set up environment
export RUSTFLAGS="-C linker=clang"
export CC=clang
export CXX=clang++

# Build ZeroClaw
cargo build --release

# The binary will be at:
# ./target/release/zeroclaw
```

### Option B: Cross-compile from PC

If you prefer to build on your computer and transfer to Android:

```bash
# On your PC with Android NDK installed:

# Add Android target
rustup target add aarch64-linux-android

# Set NDK path
export ANDROID_NDK_HOME=/path/to/android-ndk
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

# Build
CC=aarch64-linux-android21-clang \
CXX=aarch64-linux-android21-clang++ \
cargo build --release --target aarch64-linux-android

# Transfer to device
adb push target/aarch64-linux-android/release/zeroclaw /data/local/tmp/
adb shell cp /data/local/tmp/zeroclaw $PREFIX/bin/zeroclaw
adb shell chmod +x $PREFIX/bin/zeroclaw
```

## Step 4: Configure ZeroClaw

```bash
# Copy configuration files
cp .env.android $HOME/.zeroclaw/.env
cp config.android.toml $HOME/.zeroclaw/config.toml

# Create Protection config
mkdir -p $HOME/.zeroclaw
cp protection.toml $HOME/.zeroclaw/

# Ensure log directory exists
mkdir -p $HOME/.zeroclaw/logs
```

## Step 5: Run ZeroClaw

```bash
# Verify installation
zeroclaw --version

# Run setup wizard
zeroclaw onboard

# Or manual configuration:
# Edit ~/.zeroclaw/config.toml with your settings

# Start ZeroClaw in agent mode
zeroclaw agent -m "Hello from Android!"

# Start gateway (for REST/WebSocket access)
zeroclaw gateway

# Start full daemon
zeroclaw daemon
```

## Ollama Setup for Android

Since Ollama may not have official Termux support, here are alternatives:

### Alternative 1: Remote Ollama
```bash
# Use Ollama running on another machine
# Set in config.toml:
# [core.ollama]
# base_url = "http://your-server-ip:11434"
```

### Alternative 2: llama.cpp (Termux)
```bash
# Install llama.cpp
pkg install llama-cpp

# Download TinyLlama model
wget https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf

# Run server
llama-server -m tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf --host 127.0.0.1 --port 8080
```

### Alternative 3: Use OpenRouter (Online)
If local LLM doesn't work, use OpenRouter:
```bash
# In config.toml:
provider = "openrouter"
# Set OPENROUTER_API_KEY in .env
```

## Security Notes

1. **Protection Sandbox**: The Protection module is enabled by default and blocks:
   - Access to /system, /data, /sdcard
   - Commands like sudo, reboot, kill
   - Network operations without confirmation

2. **Audit Logging**: All actions are logged to ~/.zeroclaw/actions.log

3. **Autonomy Level**: Set to "supervised" for safety on Android

## Troubleshooting

### "Permission denied" when running zeroclaw
```bash
chmod +x $PREFIX/bin/zeroclaw
```

### "Cannot find Ollama"
- Ensure Ollama is running: `ollama serve &`
- Check port 11434 is available: `netstat -tlnp | grep 11434`
- Verify model is downloaded: `ollama list`

### Build fails with linker error
```bash
# Clean and rebuild
cargo clean
export RUSTFLAGS="-C linker=clang"
cargo build --release
```

### Out of memory during build
```bash
# Reduce parallel jobs
cargo build --release -j1
```

### Cannot access /sdcard
```bash
# Run in Termux
termux-setup-storage
# Then restart Termux
```

## Files Created

After setup, you'll have:
- `$HOME/zeroclaw/` - Source code
- `$HOME/.zeroclaw/` - Config and data
  - `config.toml` - Main configuration
  - `protection.toml` - Protection rules
  - `.env` - Environment variables
  - `actions.log` - Security audit log
  - `memory.db` - SQLite memory database
- `$HOME/zeroclaw-workspace/` - Working directory

## Next Steps

1. Configure channels (Telegram, Discord, etc.)
2. Set up cron jobs
3. Configure memory settings
4. Test Protection sandbox

See docs/setup-guides/ for more information.