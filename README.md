<p align="center">
  <img src="https://raw.githubusercontent.com/darkansem12-rgb/zeroclaw-android/main/docs/assets/androidclaw-banner.png" alt="AndroidClaw" width="600" />
</p>

<h1 align="center">🤖 AndroidClaw — Personal AI Assistant for Android</h1>

<p align="center">
  <strong>Zero overhead. Zero compromise. 100% Rust. 100% Agnostic.</strong><br>
  ⚡️ <strong>Runs on your Android device with Termux! Optimized for mobile AI assistance.</strong>
</p>

<p align="center">
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg" alt="License: MIT OR Apache-2.0" /></a>
  <a href="https://github.com/darkansem12-rgb/zeroclaw-android/graphs/contributors"><img src="https://img.shields.io/github/contributors/darkansem12-rgb/zeroclaw-android?color=green" alt="Contributors" /></a>
  <a href="https://buymeacoffee.com/joseluisgom"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=flat&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>
  <a href="https://www.facebook.com/Luis.gomsantana"><img src="https://img.shields.io/badge/Facebook-Luis.gomsantana-1877F2?style=flat&logo=facebook&logoColor=white" alt="Facebook" /></a>
  <a href="https://x.com/zeroclawlabs?s=21"><img src="https://img.shields.io/badge/X-%40zeroclawlabs-000000?style=flat&logo=x&logoColor=white" alt="X: @zeroclawlabs" /></a>
  <a href="https://www.facebook.com/groups/zeroclawlabs"><img src="https://img.shields.io/badge/Facebook-Zeroclaw%20Labs-1877F2?style=flat&logo=facebook&logoColor=white" alt="Facebook Group" /></a>
  <a href="https://discord.com/invite/wDshRVqRjx"><img src="https://img.shields.io/badge/Discord-Join-5865F2?style=flat&logo=discord&logoColor=white" alt="Discord" /></a>
  <a href="https://www.instagram.com/therealzeroclaw"><img src="https://img.shields.io/badge/Instagram-%40therealzeroclaw-E4405F?style=flat&logo=instagram&logoColor=white" alt="Instagram: @therealzeroclaw" /></a>
  <a href="https://www.tiktok.com/@zeroclawlabs"><img src="https://img.shields.io/badge/TikTok-%40zeroclawlabs-000000?style=flat&logo=tiktok&logoColor=white" alt="TikTok: @zeroclawlabs" /></a>
  <a href="https://www.rednote.com/user/profile/69b735e6000000002603927e"><img src="https://img.shields.io/badge/RedNote-Official-FF2442?style=flat" alt="RedNote" /></a>
  <a href="https://www.reddit.com/r/zeroclawlabs/"><img src="https://img.shields.io/badge/Reddit-r%2Fzeroclawlabs-FF4500?style=flat&logo=reddit&logoColor=white" alt="Reddit: r/zeroclawlabs" /></a>
</p>

<p align="center">
  <em>AndroidClaw is a fork of <a href="https://github.com/zeroclaw-labs/zeroclaw">ZeroClaw</a>, optimized for Android devices with Termux.</em>
</p>

<p align="center">
  🌐 <strong>Languages:</strong>
  <a href="README.md">🇺🇸 English</a> ·
  <a href="README.es.md">🇪🇸 Español</a>
</p>

AndroidClaw is a personal AI assistant you run on your own Android device using Termux. It answers you on the channels you already use (WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, and more). It has a web dashboard for real-time control and can connect to hardware peripherals (ESP32, STM32, Arduino, Raspberry Pi). The Gateway is just the control plane — the product is the assistant.

If you want a personal, single-user assistant that feels local, fast, and always-on on your Android device, this is it.

<p align="center">
  <a href="https://github.com/darkansem12-rgb/zeroclaw-android">GitHub</a> ·
  <a href="docs/README.md">Docs</a> ·
  <a href="docs/architecture.md">Architecture</a> ·
  <a href="#quick-start">Getting Started</a> ·
  <a href="docs/ops/troubleshooting.md">Troubleshoot</a> ·
  <a href="https://discord.com/invite/wDshRVqRjx">Discord</a>
</p>

> **Preferred setup:** run `androidclaw onboard` in your terminal. AndroidClaw Onboard guides you step by step through setting up the gateway, workspace, channels, and provider. It is the recommended setup path and works on Android via Termux. New install? Start here: [Getting started](#quick-start)

### Subscription Auth (OAuth)

- **OpenAI Codex** (ChatGPT subscription)
- **Gemini** (Google OAuth)
- **Anthropic** (API key or auth token)

Model note: while many providers/models are supported, for the best experience use the strongest latest-generation model available to you. See [Onboarding](#quick-start).

Models config + CLI: [Providers reference](docs/reference/api/providers-reference.md)
Auth profile rotation (OAuth vs API keys) + failover: [Model failover](docs/reference/api/providers-reference.md)

## Install (recommended)

Runtime: Rust stable toolchain. Single binary, no runtime dependencies.

### Termux (Android)

```bash
# Install Termux from F-Droid (recommended)
# Clone the repository
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android

# Run the setup script
bash termux-setup.sh

# Build and install
bash termux-build.sh
```

### One-click bootstrap

```bash
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android
./install.sh
```

`androidclaw onboard` runs automatically after install to configure your workspace and provider.

## Quick start (TL;DR)

Full beginner guide (auth, pairing, channels): [Getting started](docs/setup-guides/one-click-bootstrap.md)

```bash
# Install + onboard
./install.sh --api-key "sk-..." --provider openrouter

# Start the gateway (webhook server + web dashboard)
androidclaw gateway                # default: 127.0.0.1:42617
androidclaw gateway --port 0       # random port (security hardened)

# Talk to the assistant
androidclaw agent -m "Hello, AndroidClaw!"

# Interactive mode
androidclaw agent

# Start full autonomous runtime (gateway + channels + cron + hands)
androidclaw daemon

# Check status
androidclaw status

# Run diagnostics
androidclaw doctor
```

Upgrading? Run `androidclaw doctor` after updating.

### From source (development)

```bash
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android

cargo build --release --locked
cargo install --path . --force --locked

androidclaw onboard
```

> **Dev fallback (no global install):** prefix commands with `cargo run --release --` (example: `cargo run --release -- status`).

## Migrating from OpenClaw

AndroidClaw can import your OpenClaw workspace, memory, and configuration:

```bash
# Preview what will be migrated (safe, read-only)
androidclaw migrate openclaw --dry-run

# Run the migration
androidclaw migrate openclaw
```

This migrates your memory entries, workspace files, and configuration from `~/.openclaw/` to `~/.androidclaw/`. Config is converted from JSON to TOML automatically.

## Security defaults (DM access)

AndroidClaw connects to real messaging surfaces. Treat inbound DMs as untrusted input.

Full security guide: [SECURITY.md](SECURITY.md)

Default behavior on all channels:

- **DM pairing** (default): unknown senders receive a short pairing code and the bot does not process their message.
- Approve with: `androidclaw pairing approve <channel> <code>` (then the sender is added to a local allowlist).
- Public inbound DMs require an explicit opt-in in `config.toml`.
- Run `androidclaw doctor` to surface risky or misconfigured DM policies.

**Autonomy levels:**

| Level | Behavior |
|-------|----------|
| `ReadOnly` | Agent can observe but not act |
| `Supervised` (default) | Agent acts with approval for medium/high risk operations |
| `Full` | Agent acts autonomously within policy bounds |

**Sandboxing layers:** workspace isolation, path traversal blocking, command allowlisting, forbidden paths (`/etc`, `/root`, `~/.ssh`), rate limiting (max actions/hour, cost/day caps).

### 📢 Announcements

Use this board for important notices (breaking changes, security advisories, maintenance windows, and release blockers).

| Date (UTC) | Level       | Notice                                                                                                                                                                                                                                                                                                                                                 | Action                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-02-19 | *Critical*  | We are **not affiliated** with `openagen/zeroclaw`, `zeroclaw.org` or `zeroclaw.net`. The `zeroclaw.org` and `zeroclaw.net` domains currently points to the `openagen/zeroclaw` fork, and that domain/repository are impersonating our official website/project.                                                                                                 | Do not trust information, binaries, fundraising, or announcements from those sources. Use only [this repository](https://github.com/darkansem12-rgb/zeroclaw-android) and our verified social accounts.                                                                                                                                                                                                                                                                                                                                                                                                                    |
| 2026-02-21 | *Important* | Our official website is now live: [zeroclawlabs.ai](https://zeroclawlabs.ai). Thanks for your patience while we prepared the launch. We are still seeing impersonation attempts, so do **not** join any investment or fundraising activity claiming the ZeroClaw name unless it is published through our official channels.                                                    | Use [this repository](https://github.com/darkansem12-rgb/zeroclaw-android) as the single source of truth. Follow [X (@zeroclawlabs)](https://x.com/zeroclawlabs?s=21), [Facebook (Group)](https://www.facebook.com/groups/zeroclawlabs), and [Reddit (r/zeroclawlabs)](https://www.reddit.com/r/zeroclawlabs/) for official updates.                                                                                                                                                                                                                      |
| 2026-02-19 | *Important* | Anthropic updated the Authentication and Credential Use terms on 2026-02-19. Claude Code OAuth tokens (Free, Pro, Max) are intended exclusively for Claude Code and Claude.ai; using OAuth tokens from Claude Free/Pro/Max in any other product, tool, or service (including Agent SDK) is not permitted and may violate the Consumer Terms of Service.                | Please temporarily avoid Claude Code OAuth integrations to prevent potential loss. Original clause: [Authentication and Credential Use](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use).                                                                                                                                                                                                                                                          |

<!-- BEGIN:WHATS_NEW -->
<!-- END:WHATS_NEW -->

## Highlights

- **Lean Runtime by Default** — common CLI and status workflows run in a few-megabyte memory envelope on release builds.
- **Cost-Efficient Deployment** — designed for Android devices via Termux, no heavyweight runtime dependencies.
- **Fast Cold Starts** — single-binary Rust runtime keeps command and daemon startup near-instant.
- **Portable Architecture** — one binary across ARM, x86, and RISC-V with swappable providers/channels/tools.
- **Local-first Gateway** — single control plane for sessions, channels, tools, cron, SOPs, and events.
- **Multi-channel inbox** — WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WebSocket, and more.
- **Multi-agent orchestration (Hands)** — autonomous agent swarms that run on schedule and grow smarter over time.
- **Standard Operating Procedures (SOPs)** — event-driven workflow automation with MQTT, webhook, cron, and peripheral triggers.
- **Web Dashboard** — React 19 + Vite web UI with real-time chat, memory browser, config editor, cron manager, and tool inspector.
- **Hardware peripherals** — ESP32, STM32 Nucleo, Arduino, Raspberry Pi GPIO via the `Peripheral` trait.
- **First-class tools** — shell, file I/O, browser, git, web fetch/search, MCP, Jira, Notion, Google Workspace, and 70+ more.
- **Lifecycle hooks** — intercept and modify LLM calls, tool executions, and messages at every stage.
- **Skills platform** — bundled, community, and workspace skills with security auditing.
- **Tunnel support** — Cloudflare, Tailscale, ngrok, OpenVPN, and custom tunnels for remote access.

### Why AndroidClaw?

- **Android-optimized:** built and tested for Termux on Android devices.
- **Lean by default:** small Rust binary, fast startup, low memory footprint.
- **Secure by design:** pairing, strict sandboxing, explicit allowlists, workspace scoping.
- **Fully swappable:** core systems are traits (providers, channels, tools, memory, tunnels).
- **No lock-in:** OpenAI-compatible provider support + pluggable custom endpoints.

## Benchmark Snapshot (AndroidClaw vs OpenClaw)

Android device benchmark using Termux.

| Feature | OpenClaw | AndroidClaw 🤖 |
|---------|----------|----------------|
| **Platform** | Desktop | Android + Termux |
| **Language** | TypeScript | Rust |
| **RAM** | > 1GB | < 10MB |
| **Binary Size** | ~28MB | ~8.8 MB |
| **Startup** | > 500s | < 1s |

### Reproducible local measurement

```bash
cargo build --release
ls -lh target/release/androidclaw

/usr/bin/time -l target/release/androidclaw --help
/usr/bin/time -l target/release/androidclaw status
```

## Everything we built so far

### Core platform

- Gateway HTTP/WS/SSE control plane with sessions, presence, config, cron, webhooks, web dashboard, and pairing.
- CLI surface: `gateway`, `agent`, `onboard`, `doctor`, `status`, `service`, `migrate`, `auth`, `cron`, `channel`, `skills`.
- Agent orchestration loop with tool dispatch, prompt construction, message classification, and memory loading.
- Session model with security policy enforcement, autonomy levels, and approval gating.
- Resilient provider wrapper with failover, retry, and model routing across 20+ LLM backends.

### Channels

Channels: WhatsApp (native), Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, DingTalk, Lark, Mattermost, Nextcloud Talk, Nostr, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WATI, Mochat, Linq, Notion, WebSocket, ClawdTalk.

Feature-gated: Matrix (`channel-matrix`), Lark (`channel-lark`), Nostr (`channel-nostr`).

### Web dashboard

React 19 + Vite 6 + Tailwind CSS 4 web dashboard served directly from the Gateway:

- **Dashboard** — system overview, health status, uptime, cost tracking
- **Agent Chat** — interactive chat with the agent
- **Memory** — browse and manage memory entries
- **Config** — view and edit configuration
- **Cron** — manage scheduled tasks
- **Tools** — browse available tools
- **Logs** — view agent activity logs
- **Cost** — token usage and cost tracking
- **Doctor** — system health diagnostics
- **Integrations** — integration status and setup
- **Pairing** — device pairing management

### Firmware targets

| Target | Platform | Purpose |
|--------|----------|---------|
| ESP32 | Espressif ESP32 | Wireless peripheral agent |
| ESP32-UI | ESP32 + Display | Agent with visual interface |
| STM32 Nucleo | STM32 (ARM Cortex-M) | Industrial peripheral |
| Arduino | Arduino | Basic sensor/actuator bridge |
| Uno Q Bridge | Arduino Uno | Serial bridge to agent |

### Tools + automation

- **Core:** shell, file read/write/edit, git operations, glob search, content search
- **Web:** browser control, web fetch, web search, screenshot, image info, PDF read
- **Integrations:** Jira, Notion, Google Workspace, Microsoft 365, LinkedIn, Composio, Pushover, Weather (wttr.in)
- **MCP:** Model Context Protocol tool wrapper + deferred tool sets
- **Scheduling:** cron add/remove/update/run, schedule tool
- **Memory:** recall, store, forget, knowledge, project intel
- **Advanced:** delegate (agent-to-agent), swarm, model switch/routing, security ops, cloud ops
- **Hardware:** board info, memory map, memory read (feature-gated)

### Runtime + safety

- **Autonomy levels:** ReadOnly, Supervised (default), Full.
- **Sandboxing:** workspace isolation, path traversal blocking, command allowlists, forbidden paths, Landlock (Linux), Bubblewrap.
- **Rate limiting:** max actions per hour, max cost per day (configurable).
- **Approval gating:** interactive approval for medium/high risk operations.
- **E-stop:** emergency shutdown capability.
- **129+ security tests** in automated CI.

### Ops + packaging

- Web dashboard served directly from the Gateway.
- Tunnel support: Cloudflare, Tailscale, ngrok, OpenVPN, custom command.
- Docker runtime adapter for containerized execution.
- CI/CD: beta (auto on push) → stable (manual dispatch) → Docker, crates.io, Homebrew, Termux.
- Pre-built binaries for Linux (x86_64, aarch64, armv7), macOS (x86_64, aarch64), Windows (x86_64), Android (Termux).

## Configuration

Minimal `~/.androidclaw/config.toml`:

```toml
default_provider = "anthropic"
api_key = "sk-ant-..."
```

Full configuration reference: [docs/reference/api/config-reference.md](docs/reference/api/config-reference.md).

### Channel configuration

**Telegram:**

```toml
[channels.telegram]
bot_token = "123456:ABC-DEF..."
```

**Discord:**

```toml
[channels.discord]
token = "your-bot-token"
```

**Slack:**

```toml
[channels.slack]
bot_token = "xoxb-..."
app_token = "xapp-..."
```

**WhatsApp:**

```toml
[channels.whatsapp]
enabled = true
```

**Matrix:**

```toml
[channels.matrix]
homeserver_url = "https://matrix.org"
username = "@bot:matrix.org"
password = "..."
```

**Signal:**

```toml
[channels.signal]
phone_number = "+1234567890"
```

### Tunnel configuration

```toml
[tunnel]
kind = "cloudflare"  # or "tailscale", "ngrok", "openvpn", "custom", "none"
```

Details: [Channel reference](docs/reference/api/channels-reference.md) · [Config reference](docs/reference/api/config-reference.md)

### Runtime support (current)

- **`native`** (default) — direct process execution, fastest path, ideal for trusted environments.
- **`docker`** — full container isolation, enforced security policies, requires Docker.

Set `runtime.kind = "docker"` for strict sandboxing or network isolation.

## Subscription Auth (OpenAI Codex / Claude Code / Gemini)

AndroidClaw supports subscription-native auth profiles (multi-account, encrypted at rest).

- Store file: `~/.androidclaw/auth-profiles.json`
- Encryption key: `~/.androidclaw/.secret_key`
- Profile id format: `<provider>:<profile_name>` (example: `openai-codex:work`)

```bash
# OpenAI Codex OAuth (ChatGPT subscription)
androidclaw auth login --provider openai-codex --device-code

# Gemini OAuth
androidclaw auth login --provider gemini --profile default

# Anthropic setup-token
androidclaw auth paste-token --provider anthropic --profile default --auth-kind authorization

# Check / refresh / switch profile
androidclaw auth status
androidclaw auth refresh --provider openai-codex --profile default
androidclaw auth use --provider openai-codex --profile work

# Run the agent with subscription auth
androidclaw agent --provider openai-codex -m "hello"
androidclaw agent --provider anthropic -m "hello"
```

## Agent workspace + skills

Workspace root: `~/.androidclaw/workspace/` (configurable via config).

Injected prompt files:

- `IDENTITY.md` — agent personality and role
- `USER.md` — user context and preferences
- `MEMORY.md` — long-term facts and lessons
- `AGENTS.md` — session conventions and initialization rules
- `SOUL.md` — core identity and operating principles

Skills: `~/.androidclaw/workspace/skills/<skill>/SKILL.md` or `SKILL.toml`.

```bash
# List installed skills
androidclaw skills list

# Install from git
androidclaw skills install https://github.com/user/my-skill.git

# Security audit before install
androidclaw skills audit https://github.com/user/my-skill.git

# Remove a skill
androidclaw skills remove my-skill
```

## CLI commands

```bash
# Workspace management
androidclaw onboard              # Guided setup wizard
androidclaw status               # Show daemon/agent status
androidclaw doctor               # Run system diagnostics

# Gateway + daemon
androidclaw gateway              # Start gateway server (127.0.0.1:42617)
androidclaw daemon               # Start full autonomous runtime

# Agent
androidclaw agent                # Interactive chat mode
androidclaw agent -m "message"   # Single message mode

# Service management
androidclaw service install      # Install as OS service (launchd/systemd)
androidclaw service start|stop|restart|status

# Channels
androidclaw channel list         # List configured channels
androidclaw channel doctor       # Check channel health
androidclaw channel bind-telegram 123456789

# Cron + scheduling
androidclaw cron list            # List scheduled jobs
androidclaw cron add "*/5 * * * *" --prompt "Check system health"
androidclaw cron remove <id>

# Memory
androidclaw memory list          # List memory entries
androidclaw memory get <key>     # Retrieve a memory
androidclaw memory stats         # Memory statistics

# Auth profiles
androidclaw auth login --provider <name>
androidclaw auth status
androidclaw auth use --provider <name> --profile <profile>

# Hardware peripherals
androidclaw hardware discover    # Scan for connected devices
androidclaw peripheral list      # List connected peripherals
androidclaw peripheral flash     # Flash firmware to device

# Migration
androidclaw migrate openclaw --dry-run
androidclaw migrate openclaw

# Shell completions
source <(androidclaw completions bash)
androidclaw completions zsh > ~/.zfunc/_androidclaw
```

Full commands reference: [docs/reference/cli/commands-reference.md](docs/reference/cli/commands-reference.md)

## Prerequisites

**Android / Termux**

#### Required

1. **Termux** (install from [F-Droid](https://f-droid.org/en/packages/com.termux/))
2. **Rust toolchain:**
   ```bash
   pkg update && pkg install rust
   ```
3. **Build essentials:**
   ```bash
   pkg install build-essential
   ```

**Windows**

#### Required

1. **Visual Studio Build Tools** (provides the MSVC linker and Windows SDK):

   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

   During installation (or via the Visual Studio Installer), select the **"Desktop development with C++"** workload.

2. **Rust toolchain:**

   ```powershell
   winget install Rustlang.Rustup
   ```

   After installation, open a new terminal and run `rustup default stable` to ensure the stable toolchain is active.

3. **Verify** both are working:

   ```bash
   rustc --version
   cargo --version
   ```

#### Optional

- **Docker Desktop** — required only if using the [Docker sandboxed runtime](#runtime-support-current) (`runtime.kind = "docker"`). Install via `winget install Docker.DockerDesktop`.

**Linux / macOS**

#### Required

1. **Build essentials:**
   - **Linux (Debian/Ubuntu):** `sudo apt install build-essential pkg-config`
   - **Linux (Fedora/RHEL):** `sudo dnf group install development-tools && sudo dnf install pkg-config`
   - **macOS:** Install Xcode Command Line Tools: `xcode-select --install`
2. **Rust toolchain:**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   See [rustup.rs](https://rustup.rs) for details.

## Uninstall

```bash
# Remove binary
cargo uninstall androidclaw

# Remove configuration and workspace (optional)
rm -rf ~/.androidclaw
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

### Based on ZeroClaw

AndroidClaw is a fork of [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw), created by the ZeroClaw Labs team. ZeroClaw is an open-source project licensed under MIT/Apache-2.0.

**ZeroClaw Authors:** Built by students and members of the Harvard, MIT, and Sundai.Club communities.

For more information about ZeroClaw, visit [zeroclawlabs.ai](https://zeroclawlabs.ai).
