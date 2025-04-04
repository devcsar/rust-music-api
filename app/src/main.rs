use reqwest;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    tracks: Tracks,
}

#[derive(Debug, Deserialize)]
struct Tracks {
    items: Vec<Track>,
}

#[derive(Debug, Deserialize)]
struct Track {
    name: String,
    artists: Vec<Artist>,
    album: Album,
    duration_ms: u32,
}

#[derive(Debug, Deserialize)]
struct Artist {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Album {
    name: String,
    release_date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar las variables de entorno desde el archivo .env
    dotenv().ok();

    // Obtener el token desde la variable de entorno "token"
    let access_token = env::var("token")
        .expect("Debes definir la variable de entorno 'token' en el archivo .env");

    // Crear cliente HTTP
    let client = reqwest::Client::new();

    // Búsqueda de la canción "Darkstar" de Hans Zimmer
    let query = "Darkstar Hans Zimmer";
    let search_url = format!(
        "https://api.spotify.com/v1/search?q={}&type=track&limit=1",
        urlencoding::encode(query)
    );

    let search_response = client.get(&search_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<SearchResponse>()
        .await?;

    // Mostrar los datos de la canción si se encuentra algún resultado
    if let Some(track) = search_response.tracks.items.first() {
        println!("Datos de la canción encontrada:");
        println!("Título  : {}", track.name);
        let artist_names: Vec<String> = track.artists.iter().map(|a| a.name.clone()).collect();
        println!("Artista : {}", artist_names.join(", "));
        println!("Álbum  : {}", track.album.name);
        println!("Fecha de lanzamiento: {}", track.album.release_date);
        // Convertir la duración de milisegundos a minutos y segundos
        let duration_sec = track.duration_ms / 1000;
        let minutes = duration_sec / 60;
        let seconds = duration_sec % 60;
        println!("Duración: {}:{:02}", minutes, seconds);
    } else {
        println!("No se encontró la canción.");
    }

    Ok(())
}
