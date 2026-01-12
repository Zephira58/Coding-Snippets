use std::{
    collections::{HashSet, VecDeque},
    fs,
    fs::OpenOptions,
    io::Write,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Mutex,
    },
};

use chrono::Local;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use rusqlite::{params, Connection};
use scraper::{Html, Selector};
use tokio::{io::AsyncWriteExt, signal};

const BASE_URL: &str = "https://songs.bardmusicplayer.com/";
const DOWNLOAD_BASE: &str = "https://songs.bardmusicplayer.com/?dl=";
const OUTPUT_DIR: &str = "downloads";
const DB_FILE: &str = "bard_ids.sqlite";
const CONCURRENCY: usize = 12;

#[derive(Clone)]
struct SongEntry {
    id: String,
    download_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(OUTPUT_DIR)?;

    let mut log_file = init_logger()?;
    log(&mut log_file, "Application started");

    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_signal = shutdown.clone();

    tokio::spawn(async move {
        let _ = signal::ctrl_c().await;
        shutdown_signal.store(true, Ordering::Relaxed);
    });

    let client = Arc::new(Client::new());
    let mut conn = Connection::open(DB_FILE)?;
    init_db(&conn)?;

    log(&mut log_file, "Indexing HTML");
    let html = fetch_index(&client).await?;

    log(&mut log_file, "Parsing IDs");
    let songs = extract_songs(&html);

    log(
        &mut log_file,
        &format!("Parsed {} IDs", songs.len()),
    );

    log(&mut log_file, "Storing IDs in database");
    store_songs(&mut conn, &songs)?;

    let urls = load_urls(&conn)?;
    let total = urls.len();

    log(&mut log_file, "Starting downloads");

    let progress = ProgressBar::new(total as u64);
    progress.set_style(
        ProgressStyle::with_template(
            "\x1b[1;34mDownloading\x1b[0m [{bar:10}] {pos}/{len}: {msg}"
        )
        .unwrap()
        .progress_chars("=> "),
    );

    let completed = Arc::new(AtomicUsize::new(0));
    let active_ids = Arc::new(Mutex::new(HashSet::<String>::new()));
    let failures = Arc::new(Mutex::new(VecDeque::<(String, String)>::new()));

    stream::iter(urls)
        .for_each_concurrent(CONCURRENCY, |url| {
            let client = client.clone();
            let progress = progress.clone();
            let completed = completed.clone();
            let active_ids = active_ids.clone();
            let failures = failures.clone();
            let shutdown = shutdown.clone();

            async move {
                if shutdown.load(Ordering::Relaxed) {
                    return;
                }

                let id = url.split("dl=").nth(1).unwrap_or("").to_string();

                {
                    let mut active = active_ids.lock().unwrap();
                    active.insert(id.clone());
                    let msg = active.iter().cloned().collect::<Vec<_>>().join(", ");
                    progress.set_message(msg);
                }

                let result = download_one(&client, &url).await;

                {
                    let mut active = active_ids.lock().unwrap();
                    active.remove(&id);
                    let msg = active.iter().cloned().collect::<Vec<_>>().join(", ");
                    progress.set_message(msg);
                }

                match result {
                    Ok(_) => {
                        completed.fetch_add(1, Ordering::Relaxed);
                        progress.inc(1);
                    }
                    Err(e) => {
                        failures.lock().unwrap().push_back((id, e.to_string()));
                        progress.inc(1);
                    }
                }
            }
        })
        .await;

    progress.finish_and_clear();

    if shutdown.load(Ordering::Relaxed) {
        log(&mut log_file, "Shutdown requested via Ctrl+C");
    } else {
        log(&mut log_file, "Downloads completed normally");
    }

    let failures = failures.lock().unwrap();
    if !failures.is_empty() {
        log(&mut log_file, "Failed downloads:");
        for (id, reason) in failures.iter() {
            let _ = writeln!(log_file, "  {} -> {}", id, reason);
        }
    } else {
        log(&mut log_file, "No download failures");
    }

    log(
        &mut log_file,
        &format!(
            "Summary: {} completed, {} failed",
            completed.load(Ordering::Relaxed),
            failures.len()
        ),
    );

    println!("Done");
    Ok(())
}

async fn fetch_index(client: &Client) -> Result<String, reqwest::Error> {
    client.get(BASE_URL).send().await?.text().await
}

fn extract_songs(html: &str) -> Vec<SongEntry> {
    let document = Html::parse_document(html);
    let a_selector = Selector::parse(r#"a[href*="?dl="]"#).unwrap();

    document
        .select(&a_selector)
        .filter_map(|a| {
            let href = a.value().attr("href")?;
            let id = href.split("dl=").nth(1)?.to_string();

            Some(SongEntry {
                id: id.clone(),
                download_url: format!("{DOWNLOAD_BASE}{id}"),
            })
        })
        .collect()
}

fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS songs (
            id TEXT PRIMARY KEY,
            download_url TEXT NOT NULL
        )
        "#,
        [],
    )?;
    Ok(())
}

fn store_songs(conn: &mut Connection, songs: &[SongEntry]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(
            r#"
            INSERT OR IGNORE INTO songs
            (id, download_url)
            VALUES (?1, ?2)
            "#,
        )?;

        for s in songs {
            stmt.execute(params![s.id, s.download_url])?;
        }
    }
    tx.commit()?;
    Ok(())
}

fn load_urls(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT download_url FROM songs")?;
    let rows = stmt.query_map([], |row| row.get(0))?;

    let mut urls = Vec::new();
    for row in rows {
        urls.push(row?);
    }
    Ok(urls)
}

async fn download_one(client: &Client, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;
    let headers = response.headers();

    let filename = headers
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|v| v.to_str().ok())
        .and_then(parse_filename)
        .unwrap_or_else(|| fallback_filename(url));

    let path = Path::new(OUTPUT_DIR).join(&filename);

    if path.exists() {
        return Ok(());
    }

    let mut file = tokio::fs::File::create(&path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

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

fn init_logger() -> std::io::Result<std::fs::File> {
    fs::create_dir_all("logs")?;
    let filename = format!(
        "logs/run_{}.txt",
        Local::now().format("%Y-%m-%d_%H-%M-%S")
    );

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
}

fn log(file: &mut std::fs::File, msg: &str) {
    let _ = writeln!(
        file,
        "[{}] {}",
        Local::now().format("%H:%M:%S"),
        msg
    );
}
