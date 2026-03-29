# AndroidClaw

<h1 align="center">🤖 AndroidClaw</h1>

<p align="center">
  <strong>Personal AI Assistant for Android</strong><br>
  Run a powerful AI assistant on your Android device with Termux.
</p>

<p align="center">
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg" alt="License" /></a>
  <a href="https://buymeacoffee.com/joseluisgom"><img src="https://img.shields.io/badge/Donate-Buy%20Me%20a%20Coffee-yellow.svg" alt="Donate" /></a>
  <a href="https://www.facebook.com/Luis.gomsantana"><img src="https://img.shields.io/badge/Facebook-Luis.gomsantana-1877F2?style=flat" alt="Facebook" /></a>
  <a href="https://discord.com/invite/wDshRVqRjx"><img src="https://img.shields.io/badge/Discord-Support-5865F2?style=flat" alt="Discord" /></a>
</p>

---

## What is AndroidClaw?

AndroidClaw is a personal AI assistant that runs on your Android device using Termux. It connects to AI providers (OpenAI, Anthropic, Gemini, etc.) and can communicate through various channels.

---

## Installation

### 1. Install Termux

Download Termux from one of these sources:
- **Google Play Store**: Search for "Termux"
- **F-Droid**: https://f-droid.org/packages/com.termux/

### 2. In Termux, install dependencies:

```bash
pkg update && pkg upgrade
pkg install git rust clang cmake
```

### 3. Clone and build:

```bash
cd ~
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android
chmod +x termux-setup.sh termux-build.sh termux-run.sh
./termux-build.sh
```

Build time: ~30-60 minutes on Android.

### 4. First-time setup:

```bash
zeroclaw onboard
```

### 5. Run the assistant:

```bash
zeroclaw agent
```

---

## Quick Commands

```bash
# Interactive chat
zeroclaw agent

# Single message
zeroclaw agent -m "Hello!"

# Web dashboard
zeroclaw gateway

# Check status
zeroclaw status
```

---

## Documentation

For more details, see:
- [Android Build Guide](ANDROID_BUILD.md)
- [Full Documentation](docs/README.md)

---

## Support

- **Facebook:** https://www.facebook.com/Luis.gomsantana
- **Discord:** https://discord.com/invite/wDshRVqRjx

If you find this useful, consider donating:
https://buymeacoffee.com/joseluisgom

---

*Based on [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) - MIT/Apache-2.0 License*
