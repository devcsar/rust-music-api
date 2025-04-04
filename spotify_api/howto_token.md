#!/bin/bash

# Verifica que las variables de entorno estén definidas
if [[ -z "$SPOTIFY_CLIENT_ID" || -z "$SPOTIFY_CLIENT_SECRET" ]]; then
    echo "Error: Debes definir las variables de entorno SPOTIFY_CLIENT_ID y SPOTIFY_CLIENT_SECRET."
    exit 1
fi

# Realiza la solicitud a la API de Spotify
response=$(curl -s -X POST "https://accounts.spotify.com/api/token" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "grant_type=client_credentials&client_id=$SPOTIFY_CLIENT_ID&client_secret=$SPOTIFY_CLIENT_SECRET")

# Extrae e imprime únicamente el token de acceso
echo "$response" | jq -r '.access_token'
```

## Uso

1. Guarda el script en un archivo, por ejemplo, `obtener_token.sh`.
2. Otorga permisos de ejecución:
   ```bash
   chmod +x obtener_token.sh
   ```
3. Define las variables de entorno antes de ejecutar el script:
   ```bash
   export SPOTIFY_CLIENT_ID="tu_client_id"
   export SPOTIFY_CLIENT_SECRET="tu_client_secret"
   ./obtener_token.sh
   ```
4. Asegúrate de tener `jq` instalado para procesar la salida JSON. Si no lo tienes, puedes instalarlo con Homebrew:
   ```bash
   brew install jq
   ```

Este script imprimirá únicamente el token de acceso.