use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct TopTracksResponse {
    items: Vec<Track>,
}

#[derive(Deserialize, Debug)]
struct Track {
    name: String,
    // Puedes agregar más campos, como "artists", "album", etc.
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Reemplaza con tu token de acceso válido de Spotify
    let access_token = "BQCsTRbcWneXgl6H8ChZ0aSoib5pzcKVvKTZwQ3eRbd35FWQqWx8lpJa8CAvmbZVG2OxsvPNzSFJeEnD_EvZnAdeV2SFuIJT_6fCkTys5-SBcgufk3ieYb7FcCS1jvcPGrCgaHknSeA";

    // Endpoint para obtener las canciones top del usuario, limitando la respuesta a 5 resultados
    let url = "https://api.spotify.com/v1/me/top/tracks?limit=5";

    // Crea el cliente HTTP
    let client = reqwest::Client::new();

    // Realiza la petición GET a la API de Spotify
    let response = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await?;

    // Verifica si la petición fue exitosa
    if response.status().is_success() {
        let top_tracks: TopTracksResponse = response.json().await?;
        println!("Tus 5 canciones principales:");
        for (i, track) in top_tracks.items.iter().enumerate() {
            println!("{}: {}", i + 1, track.name);
        }
    } else {
        println!("Error en la consulta: {}", response.status());
    }

    Ok(())
}

