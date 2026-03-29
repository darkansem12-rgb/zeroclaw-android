<p align="center">
  <img src="https://raw.githubusercontent.com/darkansem12-rgb/zeroclaw-android/main/docs/assets/androidclaw-banner.png" alt="AndroidClaw" width="600" />
</p>

<h1 align="center">🤖 AndroidClaw — Asistente Personal de IA para Android</h1>

<p align="center">
  <strong>Cero sobrecarga. Cero compromisos. 100% Rust. 100% Agnóstico.</strong><br>
  ⚡️ <strong>¡Funciona en tu dispositivo Android con Termux! Optimizado para asistencia de IA móvil.</strong>
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
  <em>AndroidClaw es una bifurcación de <a href="https://github.com/darkansem12-rgb/zeroclaw-android">ZeroClaw</a>, optimizada para dispositivos Android con Termux.</em>
</p>

<p align="center">
  🌐 <strong>Idiomas:</strong>
  <a href="README.md">🇺🇸 English</a> ·
  <a href="README.es.md">🇪🇸 Español</a>
</p>

AndroidClaw es un asistente personal de IA que ejecutas en tu propio dispositivo Android usando Termux. Te responde en los canales que ya usas (WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work y más). Tiene un panel web para control en tiempo real y puede conectarse a periféricos de hardware (ESP32, STM32, Arduino, Raspberry Pi). El Gateway es solo el plano de control — el producto es el asistente.

Si quieres un asistente personal, de un solo usuario, que se sienta local, rápido y siempre activo en tu dispositivo Android, esto es lo que buscas.

<p align="center">
  <a href="https://github.com/darkansem12-rgb/zeroclaw-android">GitHub</a> ·
  <a href="docs/README.md">Documentación</a> ·
  <a href="docs/architecture.md">Arquitectura</a> ·
  <a href="#inicio-rápido">Primeros pasos</a> ·
  <a href="docs/ops/troubleshooting.md">Solución de problemas</a> ·
  <a href="https://discord.com/invite/wDshRVqRjx">Discord</a>
</p>

> **Configuración recomendada:** ejecuta `androidclaw onboard` en tu terminal. AndroidClaw Onboard te guía paso a paso en la configuración del gateway, workspace, canales y proveedor. Es la ruta de configuración recomendada y funciona en Android vía Termux. ¿Nueva instalación? Empieza aquí: [Primeros pasos](#inicio-rápido)

### Autenticación por suscripción (OAuth)

- **OpenAI Codex** (suscripción ChatGPT)
- **Gemini** (Google OAuth)
- **Anthropic** (clave API o token de autenticación)

Nota sobre modelos: aunque se soportan muchos proveedores/modelos, para la mejor experiencia usa el modelo de última generación más potente disponible. Ver [Onboarding](#inicio-rápido).

Configuración de modelos + CLI: [Referencia de proveedores](docs/reference/api/providers-reference.md)
Rotación de perfiles de autenticación (OAuth vs claves API) + failover: [Failover de modelos](docs/reference/api/providers-reference.md)

## Instalación (recomendada)

Requisito: toolchain estable de Rust. Un solo binario, sin dependencias de runtime.

### Termux (Android)

```bash
# Instalar Termux desde F-Droid (recomendado)
# Clonar el repositorio
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android

# Ejecutar el script de configuración
bash termux-setup.sh

# Compilar e instalar
bash termux-build.sh
```

### Bootstrap con un clic

```bash
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android
./install.sh
```

`androidclaw onboard` se ejecuta automáticamente después de la instalación para configurar tu workspace y proveedor.

## Inicio rápido (TL;DR)

Guía completa para principiantes (autenticación, emparejamiento, canales): [Primeros pasos](docs/setup-guides/one-click-bootstrap.md)

```bash
# Instalar + onboard
./install.sh --api-key "sk-..." --provider openrouter

# Iniciar el gateway (servidor webhook + panel web)
androidclaw gateway                # por defecto: 127.0.0.1:42617
androidclaw gateway --port 0       # puerto aleatorio (seguridad reforzada)

# Hablar con el asistente
androidclaw agent -m "¡Hola, AndroidClaw!"

# Modo interactivo
androidclaw agent

# Iniciar runtime autónomo completo (gateway + canales + cron + hands)
androidclaw daemon

# Verificar estado
androidclaw status

# Ejecutar diagnósticos
androidclaw doctor
```

¿Actualizando? Ejecuta `androidclaw doctor` después de actualizar.

### Desde el código fuente (desarrollo)

```bash
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android

cargo build --release --locked
cargo install --path . --force --locked

androidclaw onboard
```

> **Alternativa para desarrollo (sin instalación global):** antepón `cargo run --release --` a los comandos (ejemplo: `cargo run --release -- status`).

## Migración desde OpenClaw

AndroidClaw puede importar tu workspace, memoria y configuración de OpenClaw:

```bash
# Vista previa de lo que se migrará (seguro, solo lectura)
androidclaw migrate openclaw --dry-run

# Ejecutar la migración
androidclaw migrate openclaw
```

Esto migra tus entradas de memoria, archivos del workspace y configuración de `~/.openclaw/` a `~/.androidclaw/`. La configuración se convierte de JSON a TOML automáticamente.

## Valores predeterminados de seguridad (acceso por DM)

AndroidClaw se conecta a superficies de mensajería reales. Trata los DMs entrantes como entrada no confiable.

Guía completa de seguridad: [SECURITY.md](SECURITY.md)

Comportamiento predeterminado en todos los canales:

- **Emparejamiento por DM** (predeterminado): los remitentes desconocidos reciben un código de emparejamiento corto y el bot no procesa su mensaje.
- Aprobar con: `androidclaw pairing approve <channel> <code>` (luego el remitente se agrega a una lista de permitidos local).
- Los DMs públicos entrantes requieren una activación explícita en `config.toml`.
- Ejecuta `androidclaw doctor` para detectar políticas de DM riesgosas o mal configuradas.

**Niveles de autonomía:**

| Nivel | Comportamiento |
|-------|----------------|
| `ReadOnly` | El agente puede observar pero no actuar |
| `Supervised` (predeterminado) | El agente actúa con aprobación para operaciones de riesgo medio/alto |
| `Full` | El agente actúaológánomamente dentro de los límites de la política |

**Capas de sandboxing:** aislamiento del workspace, bloqueo de traversal de rutas, listas de comandos permitidos, rutas prohibidas (`/etc`, `/root`, `~/.ssh`), limitación de velocidad (máximo de acciones/hora, topes de costo/día).

### 📢 Anuncios

Usa este tablero para avisos importantes (cambios incompatibles, avisos de seguridad, ventanas de mantenimiento y bloqueadores de lanzamiento).

| Fecha (UTC) | Nivel       | Aviso                                                                                                                                                                                                                                                                                                                                                 | Acción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-02-19 | _Crítico_  | **No estamos afiliados** con `openagen/zeroclaw`, `zeroclaw.org` ni `zeroclaw.net`. Los dominios `zeroclaw.org` y `zeroclaw.net` actualmente apuntan al fork `openagen/zeroclaw`, y ese dominio/repositorio están suplantando nuestro sitio web/proyecto oficial.                                                                                       | No confíes en información, binarios, recaudaciones de fondos o anuncios de esas fuentes. Usa solo [este repositorio](https://github.com/darkansem12-rgb/zeroclaw-android) y nuestras cuentas sociales verificadas.                                                                                                                                                                                                                                                                                                                                                                                                                       |
| 2026-02-21 | _Importante_ | Nuestro sitio web oficial ya está en línea: [zeroclawlabs.ai](https://zeroclawlabs.ai). Gracias por tu paciencia mientras preparábamos el lanzamiento. Seguimos viendo intentos de suplantación, así que **no** te unas a ninguna actividad de inversión o recaudación que use el nombre de ZeroClaw a menos que se publique a través de nuestros canales oficiales.                            | Usa [este repositorio](https://github.com/darkansem12-rgb/zeroclaw-android) como la única fuente de verdad. Sigue [X (@zeroclawlabs)](https://x.com/zeroclawlabs?s=21), [Facebook (Grupo)](https://www.facebook.com/groups/zeroclawlabs) y [Reddit (r/zeroclawlabs)](https://www.reddit.com/r/zeroclawlabs/) para actualizaciones oficiales. |

## Características destacadas

- **Runtime ligero por defecto** — los flujos de trabajo comunes de CLI y estado se ejecutan en una envolvente de memoria de pocos megabytes en compilaciones release.
- **Despliegue económico** — diseñado para dispositivos Android vía Termux, sin dependencias de runtime pesadas.
- **Arranque en frío rápido** — el runtime de Rust con un solo binario mantiene el inicio de comandos y del daemon casi instantáneo.
- **Arquitectura portable** — un binario para ARM, x86 y RISC-V con proveedores/canales/herramientas intercambiables.
- **Gateway local-first** — un solo plano de control para sesiones, canales, herramientas, cron, SOPs y eventos.
- **Bandeja de entrada multicanal** — WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WebSocket y más.
- **Orquestación multi-agente (Hands)** — enjambres de agentes autónomos que se ejecutan según programación y se vuelven más inteligentes con el tiempo.
- **Procedimientos Operativos Estándar (SOPs)** — automatización de flujos de trabajo dirigida por eventos con MQTT, webhook, cron y disparadores de periféricos.
- **Panel web** — interfaz web React 19 + Vite con chat en tiempo real, explorador de memoria, editor de configuración, gestor de cron e inspector de herramientas.
- **Periféricos de hardware** — ESP32, STM32 Nucleo, Arduino, Raspberry Pi GPIO a través del trait `Peripheral`.
- **Herramientas de primera clase** — shell, E/S de archivos, navegador, git, web fetch/search, MCP, Jira, Notion, Google Workspace y más de 70 más.
- **Hooks de ciclo de vida** — intercepta y modifica llamadas LLM, ejecuciones de herramientas y mensajes en cada etapa.
- **Plataforma de skills** — skills incluidos, comunitarios y del workspace con auditoría de seguridad.
- **Soporte de túneles** — Cloudflare, Tailscale, ngrok, OpenVPN y túneles personalizados para acceso remoto.

### Por qué AndroidClaw?

- **Optimizado para Android:** construido y probado para Termux en dispositivos Android.
- **Ligero por defecto:** binario pequeño de Rust, arranque rápido, bajo consumo de memoria.
- **Seguro por diseño:** emparejamiento, sandboxing estricto, listas de permitidos explícitas, alcance del workspace.
- **Totalmente intercambiable:** los sistemas centrales son traits (proveedores, canales, herramientas, memoria, túneles).
- **Sin dependencia de proveedor:** soporte de proveedores compatibles con OpenAI + endpoints personalizados conectables.

## Resumen de benchmarks (AndroidClaw vs OpenClaw)

Benchmark en dispositivo Android usando Termux.

| Característica | OpenClaw | AndroidClaw 🤖 |
|----------------|----------|----------------|
| **Plataforma** | Desktop | Android + Termux |
| **Lenguaje** | TypeScript | Rust |
| **RAM** | > 1GB | < 10MB |
| **Tamaño del binario** | ~28MB | ~8.8 MB |
| **Arranque** | > 500s | < 1s |

### Medición local reproducible

```bash
cargo build --release
ls -lh target/release/androidclaw

/usr/bin/time -l target/release/androidclaw --help
/usr/bin/time -l target/release/androidclaw status
```

## Todo lo que hemos construido hasta ahora

### Plataforma central

- Plano de control Gateway HTTP/WS/SSE con sesiones, presencia, configuración, cron, webhooks, panel web y emparejamiento.
- Superficie CLI: `gateway`, `agent`, `onboard`, `doctor`, `status`, `service`, `migrate`, `auth`, `cron`, `channel`, `skills`.
- Bucle de orquestación del agente con despacho de herramientas, construcción de prompts, clasificación de mensajes y carga de memoria.
- Modelo de sesión con aplicación de políticas de seguridad, niveles de autonomía y aprobación condicional.
- Wrapper de proveedor resiliente con failover, reintentos y enrutamiento de modelos a través de más de 20 backends LLM.

### Canales

Canales: WhatsApp (nativo), Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, DingTalk, Lark, Mattermost, Nextcloud Talk, Nostr, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WATI, Mochat, Linq, Notion, WebSocket, ClawdTalk.

Habilitados por feature gate: Matrix (`channel-matrix`), Lark (`channel-lark`), Nostr (`channel-nostr`).

### Panel web

Panel web React 19 + Vite 6 + Tailwind CSS 4 servido directamente desde el Gateway:

- **Dashboard** — resumen del sistema, estado de salud, tiempo de actividad, seguimiento de costos
- **Chat del agente** — chat interactivo con el agente
- **Memoria** — explorar y gestionar entradas de memoria
- **Configuración** — ver y editar configuración
- **Cron** — gestionar tareas programadas
- **Herramientas** — explorar herramientas disponibles
- **Registros** — ver registros de actividad del agente
- **Costos** — uso de tokens y seguimiento de costos
- **Doctor** — diagnósticos de salud del sistema
- **Integraciones** — estado y configuración de integraciones
- **Emparejamiento** — gestión de emparejamiento de dispositivos

### Objetivos de firmware

| Objetivo | Plataforma | Propósito |
|----------|------------|-----------|
| ESP32 | Espressif ESP32 | Agente periférico inalámbrico |
| ESP32-UI | ESP32 + Display | Agente con interfaz visual |
| STM32 Nucleo | STM32 (ARM Cortex-M) | Periférico industrial |
| Arduino | Arduino | Puente básico de sensores/actuadores |
| Uno Q Bridge | Arduino Uno | Puente serial al agente |

### Herramientas + automatización

- **Core:** shell, lectura/escritura/edición de archivos, operaciones git, búsqueda glob, búsqueda de contenido
- **Web:** control de navegador, web fetch, web search, captura de pantalla, información de imagen, lectura de PDF
- **Integraciones:** Jira, Notion, Google Workspace, Microsoft 365, LinkedIn, Composio, Pushover
- **MCP:** Model Context Protocol tool wrapper + conjuntos de herramientas diferidos
- **Programación:** cron add/remove/update/run, herramienta de programación
- **Memoria:** recall, store, forget, knowledge, project intel
- **Avanzado:** delegate (agente a agente), swarm, cambio/enrutamiento de modelos, operaciones de seguridad, operaciones en la nube
- **Hardware:** board info, memory map, memory read (habilitado por feature gate)

### Runtime + seguridad

- **Niveles de autonomía:** ReadOnly, Supervised (predeterminado), Full.
- **Sandboxing:** aislamiento del workspace, bloqueo de traversal de rutas, listas de comandos permitidos, rutas prohibidas, Landlock (Linux), Bubblewrap.
- **Limitación de velocidad:** máximo de acciones por hora, máximo de costo por día (configurable).
- **Aprobación condicional:** aprobación interactiva para operaciones de riesgo medio/alto.
- **Parada de emergencia:** capacidad de apagado de emergencia.
- **129+ pruebas de seguridad** en CI automatizado.

### Operaciones + empaquetado

- Panel web servido directamente desde el Gateway.
- Soporte de túneles: Cloudflare, Tailscale, ngrok, OpenVPN, comando personalizado.
- Adaptador de runtime Docker para ejecución en contenedores.
- CI/CD: beta (automático al hacer push) → stable (dispatch manual) → Docker, crates.io, Homebrew, Termux.
- Binarios preconstruidos para Linux (x86_64, aarch64, armv7), macOS (x86_64, aarch64), Windows (x86_64), Android (Termux).

## Configuración

`~/.androidclaw/config.toml` mínimo:

```toml
default_provider = "anthropic"
api_key = "sk-ant-..."
```

Referencia completa de configuración: [docs/reference/api/config-reference.md](docs/reference/api/config-reference.md).

### Configuración de canales

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

### Configuración de túneles

```toml
[tunnel]
kind = "cloudflare"  # o "tailscale", "ngrok", "openvpn", "custom", "none"
```

Detalles: [Referencia de canales](docs/reference/api/channels-reference.md) · [Referencia de configuración](docs/reference/api/config-reference.md)

### Soporte de runtime (actual)

- **`native`** (predeterminado) — ejecución directa de procesos, la ruta más rápida, ideal para entornos de confianza.
- **`docker`** — aislamiento completo en contenedores, políticas de seguridad forzadas, requiere Docker.

Establece `runtime.kind = "docker"` para sandboxing estricto o aislamiento de red.

## Autenticación por suscripción (OpenAI Codex / Claude Code / Gemini)

AndroidClaw soporta perfiles de autenticación nativos de suscripción (multi-cuenta, cifrados en reposo).

- Archivo de almacenamiento: `~/.androidclaw/auth-profiles.json`
- Clave de cifrado: `~/.androidclaw/.secret_key`
- Formato de id de perfil: `<provider>:<profile_name>` (ejemplo: `openai-codex:work`)

```bash
# OpenAI Codex OAuth (suscripción ChatGPT)
androidclaw auth login --provider openai-codex --device-code

# Gemini OAuth
androidclaw auth login --provider gemini --profile default

# Anthropic setup-token
androidclaw auth paste-token --provider anthropic --profile default --auth-kind authorization

# Verificar / refrescar / cambiar perfil
androidclaw auth status
androidclaw auth refresh --provider openai-codex --profile default
androidclaw auth use --provider openai-codex --profile work

# Ejecutar el agente con autenticación por suscripción
androidclaw agent --provider openai-codex -m "hello"
androidclaw agent --provider anthropic -m "hello"
```

## Workspace del agente + skills

Raíz del workspace: `~/.androidclaw/workspace/` (configurable vía config).

Archivos de prompt inyectados:
- `IDENTITY.md` — personalidad y rol del agente
- `USER.md` — contexto y preferencias del usuario
- `MEMORY.md` — hechos y lecciones a largo plazo
- `AGENTS.md` — convenciones de sesión y reglas de inicialización
- `SOUL.md` — identidad central y principios operativos

Skills: `~/.androidclaw/workspace/skills/<skill>/SKILL.md` o `SKILL.toml`.

```bash
# Listar skills instalados
androidclaw skills list

# Instalar desde git
androidclaw skills install https://github.com/user/my-skill.git

# Auditoría de seguridad antes de instalar
androidclaw skills audit https://github.com/user/my-skill.git

# Eliminar un skill
androidclaw skills remove my-skill
```

## Comandos CLI

```bash
# Gestión del workspace
androidclaw onboard              # Asistente de configuración guiada
androidclaw status               # Mostrar estado del daemon/agente
androidclaw doctor               # Ejecutar diagnósticos del sistema

# Gateway + daemon
androidclaw gateway              # Iniciar servidor gateway (127.0.0.1:42617)
androidclaw daemon               # Iniciar runtime autónomo completo

# Agente
androidclaw agent                # Modo de chat interactivo
androidclaw agent -m "message"   # Modo de mensaje único

# Gestión de servicios
androidclaw service install      # Instalar como servicio del SO (launchd/systemd)
androidclaw service start|stop|restart|status

# Canales
androidclaw channel list         # Listar canales configurados
androidclaw channel doctor       # Verificar salud de los canales
androidclaw channel bind-telegram 123456789

# Cron + programación
androidclaw cron list            # Listar trabajos programados
androidclaw cron add "*/5 * * * *" --prompt "Check system health"
androidclaw cron remove <id>

# Memoria
androidclaw memory list          # Listar entradas de memoria
androidclaw memory get <key>     # Recuperar una memoria
androidclaw memory stats         # Estadísticas de memoria

# Perfiles de autenticación
androidclaw auth login --provider <name>
androidclaw auth status
androidclaw auth use --provider <name> --profile <profile>

# Periféricos de hardware
androidclaw hardware discover    # Escanear dispositivos conectados
androidclaw peripheral list      # Listar periféricos conectados
androidclaw peripheral flash     # Flashear firmware al dispositivo

# Migración
androidclaw migrate openclaw --dry-run
androidclaw migrate openclaw

# Completado de shell
source <(androidclaw completions bash)
androidclaw completions zsh > ~/.zfunc/_androidclaw
```

Referencia completa de comandos: [docs/reference/cli/commands-reference.md](docs/reference/cli/commands-reference.md)

## Prerrequisitos

**Android / Termux**

#### Requerido

1. **Termux** (instalar desde [F-Droid](https://f-droid.org/en/packages/com.termux/))
2. **Toolchain de Rust:**
   ```bash
   pkg update && pkg install rust
   ```
3. **Herramientas de compilación:**
   ```bash
   pkg install build-essential
   ```

**Windows**

#### Requerido

1. **Visual Studio Build Tools** (proporciona el enlazador MSVC y el SDK de Windows):

   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

   Durante la instalación (o a través del Visual Studio Installer), selecciona la carga de trabajo **"Desarrollo de escritorio con C++"**.

2. **Toolchain de Rust:**

   ```powershell
   winget install Rustlang.Rustup
   ```

   Después de la instalación, abre una nueva terminal y ejecuta `rustup default stable` para asegurarte de que el toolchain estable esté activo.

3. **Verifica** que ambos funcionen:
   ```bash
   rustc --version
   cargo --version
   ```

#### Opcional

- **Docker Desktop** — requerido solo si usas el [runtime sandbox con Docker](#soporte-de-runtime-actual) (`runtime.kind = "docker"`). Instala vía `winget install Docker.DockerDesktop`.

**Linux / macOS**

#### Requerido

1. **Herramientas de compilación esenciales:**
   - **Linux (Debian/Ubuntu):** `sudo apt install build-essential pkg-config`
   - **Linux (Fedora/RHEL):** `sudo dnf group install development-tools && sudo dnf install pkg-config`
   - **macOS:** Instala Xcode Command Line Tools: `xcode-select --install`

2. **Toolchain de Rust:**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Ver [rustup.rs](https://rustup.rs) para detalles.

## Desinstalar

```bash
# Eliminar binario
cargo uninstall androidclaw

# Eliminar configuración y workspace (opcional)
rm -rf ~/.androidclaw
```

## Contribuir

¿Nuevo en AndroidClaw? Consulta nuestra [Guía de contribución](CONTRIBUTING.md) para saber cómo empezar. ¡PRs con IA/vibe-coded son bienvenidos! 🤖

Ver [CONTRIBUTING.md](CONTRIBUTING.md).

## Licencia

AndroidClaw tiene doble licencia para máxima apertura y protección de los contribuidores:

| Licencia | Caso de uso |
|---|---|
| [MIT](LICENSE-MIT) | Código abierto, investigación, académico, uso personal |
| [Apache 2.0](LICENSE-APACHE) | Protección de patentes, institucional, despliegue comercial |

Puedes elegir cualquiera de las licencias. **Los contribuidores otorgan automáticamente derechos bajo ambas** — ver [CLA.md](docs/contributing/cla.md) para el acuerdo completo de contribuidores.

---

### Basado en ZeroClaw

AndroidClaw es una bifurcación de [ZeroClaw](https://github.com/darkansem12-rgb/zeroclaw-android), creado por el equipo de ZeroClaw Labs. ZeroClaw es un proyecto de código abierto bajo licencia MIT/Apache-2.0.

**Autores de ZeroClaw:** Construido por estudiantes y miembros de las comunidades de Harvard, MIT y Sundai.Club.

Para más información sobre ZeroClaw, visita [zeroclawlabs.ai](https://zeroclawlabs.ai).

---

**AndroidClaw** — Tu asistente de IA personal en Android. 🦀

## Contribuidores

<a href="https://github.com/darkansem12-rgb/zeroclaw-android/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=darkansem12-rgb/zeroclaw-android" alt="AndroidClaw contributors" />
</a>

Esta lista se genera a partir del gráfico de contribuidores de GitHub y se actualiza automáticamente.
