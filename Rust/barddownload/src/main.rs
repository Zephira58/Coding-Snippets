use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
    sync::Arc,
};

use futures::stream::{self, StreamExt};
use reqwest::Client;
use tokio::io::AsyncWriteExt;

const LINK_FILE: &str = "bard_music_player_links.txt";
const OUTPUT_DIR: &str = "downloads";
const CONCURRENCY: usize = 12; // adjust: 8–16 is safe

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(OUTPUT_DIR)?;

    let file = File::open(LINK_FILE)?;
    let reader = BufReader::new(file);

    let links: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .collect();

    let client = Arc::new(Client::new());

    println!("Downloading {} files...", links.len());

    stream::iter(links)
        .for_each_concurrent(CONCURRENCY, |url| {
            let client = client.clone();
            async move {
                if let Err(e) = download_one(&client, &url).await {
                    eprintln!("❌ {} -> {}", url, e);
                }
            }
        })
        .await;

    println!("✅ All downloads finished");
    Ok(())
}

async fn download_one(client: &Client, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;
    let headers = response.headers();

    // Try to extract filename from headers
    let filename = headers
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|v| v.to_str().ok())
        .and_then(parse_filename)
        .unwrap_or_else(|| fallback_filename(url));

    let path = Path::new(OUTPUT_DIR).join(&filename);

    if path.exists() {
        return Ok(()); // skip existing
    }

    let mut file = tokio::fs::File::create(&path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

    println!("✔ {}", filename);
    Ok(())
}

fn parse_filename(header: &str) -> Option<String> {
    header
        .split(';')
        .find(|s| s.trim().starts_with("filename="))
        .map(|s| s.trim().replace("filename=", "").replace('"', ""))
}

fn fallback_filename(url: &str) -> String {
    let id = url.split("dl=").nth(1).unwrap_or("unknown");
    format!("{id}.mid")
}
