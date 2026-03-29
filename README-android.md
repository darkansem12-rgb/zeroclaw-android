# AndroidClaw - Instalación en Android

Guía rápida para ejecutar AndroidClaw en tu teléfono Android usando Termux.

## Requisitos

- Teléfono Android con Termux instalado
- Conexión a internet
- ~30-60 minutos para compilar

## Instalación Rápida

### Paso 1: Descargar Termux

Descarga Termux desde una de estas fuentes:
- **Google Play Store**: Busca "Termux"
- **F-Droid**: https://f-droid.org/packages/com.termux/

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

### Paso 4: Primera configuración

```bash
zeroclaw onboard
```

### Paso 5: Ejecutar

```bash
zeroclaw agent
```

## Comandos Básicos

```bash
# Chat interactivo
zeroclaw agent

# Mensaje único
zeroclaw agent -m "Hola"

# Iniciar gateway
zeroclaw gateway

# Ver estado
zeroclaw status
```

## Solución de Problemas

### Error de permisos
```bash
chmod +x zeroclaw
```

### Memoria insuficiente durante compilación
```bash
cargo build --release -j1
```

## Desinstalar

```bash
rm -rf ~/zeroclaw-android ~/.zeroclaw
```

## Soporte

- Facebook: https://www.facebook.com/Luis.gomsantana
- Issues: https://github.com/darkansem12-rgb/zeroclaw-android/issues
