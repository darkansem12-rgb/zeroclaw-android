# ZeroClaw en Android via Termux

Guía completa para ejecutar ZeroClaw en tu teléfono Android usando Termux.

## Requisitos

- 📱 Teléfono Android con Termux instalado
- 📶 Conexión a internet (para descargar e instalar)
- 🔋 Batería suficiente (~30-60 min de compilación)

## Método 1: Compilar en Termux (Recomendado)

### Si ya tienes el repo clonado (actualizar scripts):

```bash
cd zeroclaw-android
git pull
chmod +x termux-*.sh
./termux-setup.sh
./termux-build.sh
```

### Si es la primera vez:

```bash
# Descargar archivos del repo
pkg install git
rm -rf zeroclaw-android  # Si hay carpeta antigua con errores
git clone https://github.com/darkansem12-rgb/zeroclaw-android.git
cd zeroclaw-android

# Dar permisos a los scripts
chmod +x termux-setup.sh termux-build.sh termux-run.sh

# Ejecutar setup (instala rust, clang)
./termux-setup.sh

# Compilar (~30-60 minutos)
./termux-build.sh

# Ejecutar
./termux-run.sh
```

### Si ya compilaste antes y quieres recompilar:

```bash
cd zeroclaw-android
./termux-build.sh
./termux-run.sh
```

---

## Método 2: Descargar binario pre-compilado

Aún no disponible - el binario se compilará con GitHub Actions.

---

## Configuración de ZeroClaw

### Emparejamiento (Pairing)

Al iniciar ZeroClaw por primera vez, verás un código de 6 dígitos en la terminal:

```
🔐 Pairing code: 123456
```

1. Abre http://127.0.0.1:42617 en Chrome
2. Ingresa el código de emparejamiento
3. ¡Listo!

### Configurar API Key

```bash
# Crear archivo de configuración
mkdir -p ~/.config/zeroclaw
cat > ~/.config/zeroclaw/config.toml << 'EOF'
[providers]
default = "openai"

[providers.openai]
api_key = "tu-api-key-aqui"

[server]
port = 42617
host = "0.0.0.0"
EOF
```

### Usar Ollama (IA Local)

Si tienes Ollama instalado en tu teléfono:

```bash
# En otra sesión de Termux
pkg install ollama
ollama serve

# En ZeroClaw, usar provider "ollama"
```

---

## Solución de Problemas

### Error: "Permission denied"

```bash
chmod +x zeroclaw
```

### Error: "Web dashboard not available"

Los archivos `web/dist/` deben estar junto al binario o embebidos en él. Verifica que los descargaste correctamente.

### Error: "Out of memory" durante compilación

```bash
# Usar solo 1 job de compilación
export CARGO_BUILD_JOBS=1
cargo build --release
```

### Teléfono se calienta mucho

- Cierra otras apps durante la compilación
- Conecta el teléfono al cargador
- Usa `cargo build --release -j 1` para usar menos recursos

---

## Acceso Remoto

Para acceder a ZeroClaw desde otro dispositivo:

### Opción 1: Túnel SSH (requiere PC)

```bash
# En Termux
pkg install openssh
ssh -R 42617:localhost:42617 tu-servidor.com
```

### Opción 2: ngrok

```bash
pkg install ngrok
ngrok http 42617
```

---

## Estructura de Archivos

```
zeroclaw-android/
├── zeroclaw              # Binario compilado
├── web/dist/             # Archivos del dashboard
├── Cargo.toml            # Configuración de Rust
├── termux-setup.sh       # Script de instalación
├── termux-build.sh       # Script de compilación
├── termux-run.sh         # Script de ejecución
└── README-android.md     # Este archivo
```

---

## Desinstalar

```bash
cd ~
rm -rf zeroclaw zeroclaw-android
rm -rf ~/.config/zeroclaw
```

---

## Soporte

- 📖 Documentación: https://github.com/darkansem12-rgb/zeroclaw-android
- 🐛 Issues: https://github.com/darkansem12-rgb/zeroclaw-android/issues
