# Proyecto AndroidClaw - Registro del Proyecto

## Resumen del Proyecto

| Campo | Valor |
|-------|-------|
| **Nombre** | AndroidClaw |
| **Repo** | https://github.com/darkansem12-rgb/zeroclaw-android |
| **Fork de** | ZeroClaw (https://github.com/zeroclaw-labs/zeroclaw) |
| **Descripción** | Asistente de IA personal para Android usando Termux |
| **Licencia** | MIT OR Apache-2.0 |

---

## Información del Creador

| Campo | Valor |
|-------|-------|
| **Nombre** | Luis Gom (darkansem12-rgb) |
| **Facebook** | https://www.facebook.com/Luis.gomsantana |
| **Donaciones** | https://buymeacoffee.com/joseluisgom |
| **GitHub** | https://github.com/darkansem12-rgb |

---

## COMANDOS PRINCIPALES

### El binario compilado se llama `zeroclaw` (NO `androidclaw`)

```bash
# Configuración inicial (OBLIGATORIO - primera vez)
zeroclaw onboard

# Ejecutar el asistente
zeroclaw agent

# Chat con mensaje
zeroclaw agent -m "tu mensaje aqui"

# Gateway/web dashboard
zeroclaw gateway

# Ver estado
zeroclaw status

# Diagnósticos
zeroclaw doctor
```

---

## Instalación en Termux (Resumen)

```bash
# 1. Instalar Termux (Play Store o F-Droid)
# 2. En Termux:
pkg update && pkg upgrade
pkg install git rust clang cmake

# 3. Clonar y compilar
cd ~
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android
chmod +x termux-setup.sh termux-build.sh termux-run.sh
./termux-build.sh

# 4. Primera configuración (OBLIGATORIO)
zeroclaw onboard

# 5. Ejecutar
zeroclaw agent
```

---

## Estructura del Proyecto

```
zeroclaw-android/
├── src/                          # Código fuente Rust
├── Cargo.toml                    # Config del crate
├── docs/assets/
│   └── androidclaw-banner.png    # Banner
├── README.md                     # Principal (inglés)
├── README.es.md                  # Español
├── README-android.md             # Guía rápida Android
├── README.*.md                   # Otros idiomas (27 archivos)
├── ANDROID_BUILD.md             # Instrucciones compilación
├── AGENTS.md                    # Instrucciones agentes IA
├── CLAUDE.md                    # Config Claude Code
├── config.android.toml          # Config para Android
├── termux-setup.sh             # Script setup
├── termux-build.sh             # Script compilación
├── termux-run.sh               # Script ejecución
└── install.sh                  # Instalación general
```

---

## Cambios Realizados al Fork

### Marca y documentación:
- [x] Nombre: ZeroClaw → AndroidClaw
- [x] Banner creado: `docs/assets/androidclaw-banner.png`
- [x] README.md reescrito completamente
- [x] README.es.md versión en español
- [x] README-android.md simplificado
- [x] 27 README localizados actualizados
- [x] Cargo.toml: nombre, repo URL

### Links y redes:
- [x] Donaciones → buymeacoffee.com/joseluisgom
- [x] Facebook → facebook.com/Luis.gomsantana
- [x] Nota "Based on ZeroClaw" mantenida
- [x] Discord de soporte Zeroclaw mantenido

---

## Pendiente / Por Hacer

- [ ] Verificar que el banner se vea en GitHub
- [ ] Probar compilación en Termux
- [ ] Crear releases pre-compiladas para Android
- [ ] Posible cambio de nombre binario a `androidclaw`

---

## Notas Técnicas

1. **Binario = `zeroclaw`** - Nombre hardcodeado en el código
2. **`zeroclaw onboard` es obligatorio** - Única forma de configurar
3. **Termux funciona desde Play Store** - F-Droid opcional
4. **Compilación ~30-60 min** en Android

---

## GitHub

**Token:** Solicitar nuevo token si no funciona

**Comandos git:**
```bash
# Clonar
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git

# Configurar
git config user.email "tu-email@email.com"
git config user.name "tu-nombre"

# Commit y push
git add -A
git commit -m "mensaje"
git push origin main
```

---

## Historial de Commits

| # | Mensaje | Descripción |
|---|---------|-------------|
| 1 | refactor: rebranding to AndroidClaw | Primer rebranding |
| 2 | refactor: simplify documentation | Simplificación docs |
| 3 | fix: correct installation commands | Corrección comandos |

---

## Referencias

- Repo original ZeroClaw: https://github.com/zeroclaw-labs/zeroclaw
- Website ZeroClaw: https://zeroclawlabs.ai
- Discord soporte: https://discord.com/invite/wDshRVqRjx
