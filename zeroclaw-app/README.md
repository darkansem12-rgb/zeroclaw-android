# ZeroClaw Android App

APK nativa para conectar a ZeroClaw corriendo en Termux.

## Requisitos

- Android Studio instalado
- JDK 17+
- ZeroClaw corriendo en Termux (`./zeroclaw daemon`)

## Instalación de Android Studio

1. Descarga Android Studio: https://developer.android.com/studio
2. Instala con configuración predeterminada
3. Asegúrate de tener Android SDK 34 instalado

## Abrir el Proyecto

1. Abre Android Studio
2. File → Open
3. Selecciona la carpeta `zeroclaw-app`
4. Espera a que Gradle sincronice

## Construir la APK

### Opción 1: Desde Android Studio (Recomendado)

1. Menú Build → Build Bundle(s) / APK(s) → Build APK(s)
2. Espera a que compile
3. La APK estará en `app/build/outputs/apk/debug/app-debug.apk`

### Opción 2: Desde Línea de Comandos

```bash
cd zeroclaw-app
./gradlew assembleDebug
```

La APK estará en: `app/build/outputs/apk/debug/app-debug.apk`

## Instalar en el Teléfono

1. Transfer the APK to your phone
2. Enable "Install from unknown sources" in Settings
3. Tap the APK to install
4. Open ZeroClaw app

## Uso

1. Asegúrate de que ZeroClaw esté corriendo en Termux:
   ```bash
   ./zeroclaw daemon
   ```

2. Abre la app ZeroClaw en tu teléfono

3. La app verificará la conexión y mostrará el dashboard

## Solución de Problemas

### "No se puede conectar"
- Verifica que ZeroClaw esté corriendo en Termux
- Ejecuta `./zeroclaw daemon` en Termux

### La app se cierra
- Verifica que tienes Android 7.0 o superior
- Verifica que Termux esté instalado

### La pantalla está en blanco
- Espera unos segundos a que cargue
- Verifica tu conexión a internet

## Arquitectura

```
┌─────────────────────────────────────────────────┐
│              APK ZeroClaw (este proyecto)         │
├─────────────────────────────────────────────────┤
│  MainActivity.kt                                │
│  - WebView que conecta a localhost:42617        │
│  - Verificación de conexión                      │
│  - Manejo de errores                            │
└─────────────────────────────────────────────────┘
                    ↓ localhost:42617
┌─────────────────────────────────────────────────┐
│              Termux + ZeroClaw                   │
│  ./zeroclaw daemon                              │
└─────────────────────────────────────────────────┘
```

## Permisos

- INTERNET: Para conectar al daemon de ZeroClaw
- ACCESS_NETWORK_STATE: Verificar estado de red
- ACCESS_WIFI_STATE: Verificar conexión WiFi

## Licencia

MIT License
