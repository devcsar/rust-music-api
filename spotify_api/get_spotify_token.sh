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
