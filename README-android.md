# AndroidClaw - Instalación en Android

Guía rápida para ejecutar AndroidClaw en tu teléfono Android usando Termux.

## Requisitos

- Teléfono Android con Termux instalado
- Conexión a internet
- ~30-60 minutos para compilar

## Instalación Rápida

### Paso 1: Descargar Termux

Descarga **Termux desde F-Droid** (importante: no desde Play Store):
https://f-droid.org/packages/com.termux/

### Paso 2: Abrir Termux y ejecutar:

```bash
pkg update && pkg upgrade
pkg install git rust clang cmake
```

### Paso 3: Clonar y Compilar

```bash
cd ~
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android
chmod +x termux-setup.sh termux-build.sh termux-run.sh
./termux-build.sh
```

### Paso 4: Ejecutar

```bash
./termux-run.sh
```

## Configuración

### API Key

```bash
mkdir -p ~/.androidclaw
nano ~/.androidclaw/config.toml
```

```toml
default_provider = "openai"
api_key = "tu-api-key-aqui"
```

## Comandos Básicos

```bash
# Chat interactivo
androidclaw agent

# Mensaje único
androidclaw agent -m "Hola"

# Iniciar gateway
androidclaw gateway
```

## Solución de Problemas

### Error de permisos
```bash
chmod +x androidclaw
```

### Memoria insuficiente
```bash
cargo build --release -j1
```

## Desinstalar

```bash
rm -rf ~/zeroclaw-android ~/.androidclaw
```

## Soporte

- Facebook: https://www.facebook.com/Luis.gomsantana
- Issues: https://github.com/darkansem12-rgb/zeroclaw-android/issues
