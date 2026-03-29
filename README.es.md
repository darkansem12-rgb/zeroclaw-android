# AndroidClaw

<h1 align="center">🤖 AndroidClaw</h1>

<p align="center">
  <strong>Asistente de IA Personal para Android</strong><br>
  Ejecuta un asistente de IA poderoso en tu dispositivo Android con Termux.
</p>

<p align="center">
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/licencia-MIT%20OR%20Apache%202.0-blue.svg" alt="Licencia" /></a>
  <a href="https://buymeacoffee.com/joseluisgom"><img src="https://img.shields.io/badge/Donar-Buy%20Me%20a%20Coffee-yellow.svg" alt="Donar" /></a>
  <a href="https://www.facebook.com/Luis.gomsantana"><img src="https://img.shields.io/badge/Facebook-Luis.gomsantana-1877F2?style=flat" alt="Facebook" /></a>
  <a href="https://discord.com/invite/wDshRVqRjx"><img src="https://img.shields.io/badge/Discord-Soporte-5865F2?style=flat" alt="Discord" /></a>
</p>

---

## ¿Qué es AndroidClaw?

AndroidClaw es un asistente de IA personal que se ejecuta en tu dispositivo Android usando Termux. Se conecta a proveedores de IA (OpenAI, Anthropic, Gemini, etc.) y puede comunicarse a través de varios canales.

---

## Instalación

### 1. Instalar Termux

Descarga Termux desde una de estas fuentes:
- **Google Play Store**: Busca "Termux"
- **F-Droid**: https://f-droid.org/packages/com.termux/

### 2. En Termux, instala las dependencias:

```bash
pkg update && pkg upgrade
pkg install git rust clang cmake
```

### 3. Clona y compila:

```bash
cd ~
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android
chmod +x termux-setup.sh termux-build.sh termux-run.sh
./termux-build.sh
```

Tiempo de compilación: ~30-60 minutos en Android.

### 4. Primera configuración:

```bash
zeroclaw onboard
```

### 5. Ejecutar el asistente:

```bash
zeroclaw agent
```

---

## Comandos Rápidos

```bash
# Chat interactivo
zeroclaw agent

# Mensaje único
zeroclaw agent -m "¡Hola!"

# Panel web
zeroclaw gateway

# Ver estado
zeroclaw status
```

---

## Documentación

Para más detalles, consulta:
- [Guía de Construcción Android](ANDROID_BUILD.md)
- [Documentación Completa](docs/README.md)

---

## Soporte

- **Facebook:** https://www.facebook.com/Luis.gomsantana
- **Discord:** https://discord.com/invite/wDshRVqRjx

Si esto te es útil, considera donate:
https://buymeacoffee.com/joseluisgom

---

*Basado en [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) - Licencia MIT/Apache-2.0*
