// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::min;
use futures::stream::StreamExt;
use reqwest::Client;
use tauri::Manager;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn synchronize(app: tauri::AppHandle) {
    // emits the synchronized event to all windows
    app.emit_all("synchronized", ());
}


#[tauri::command]
async fn download_file_to_path(url: String, path: String, app: tauri::AppHandle) -> Result<(), String> {
    // Create directory if it doesn't exist
    // fs::create_dir_all("./download/").expect("Failure creating download folder");

    // let start_time = Instant::now();
    let client = Client::new();
    let res = client
        .get(&url)
        // .header(
        //     USER_AGENT,
        //     "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0",
        // )
        // .header(ACCEPT, "application/octet-stream")
        .send()
        .await
        .map_err(|_| format!("Failed to parse `{}` file", &url))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get the size of the `{}` file", &url))?;

    // let mut file = File::create(&path);
    let mut file = File::create(&path).await.map_err(|_| format!("Failed to create `{}` file", &path))?;
    // let mut file = File::create(&path);
    //.or(Err(format!("Failed to create `{}` file", &path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Failed to download `{}` file", &path)))?;
        file.write(&chunk).await.map_err(|_| format!("Failed to write to `{}` file", &path))?;
        //.or(Err(format!("Failed to write to `{}` file", &path)))?;
        // file.write_all(&chunk).await.map_err(|_| format!("Failed to write to `{}` file", &path)))?;
        downloaded = min(downloaded + (chunk.len() as u64), total_size);
    //     // let duration = start_time.elapsed().as_secs_f64();
    //     // let speed = if duration > 0.0 {
    //     //     Some(downloaded as f64 / duration / 1024.0 / 1024.0)
    //     // } else {
    //     //     None
    //     // };
        println!("downloaded => {}", downloaded);
        println!("total_size => {}", total_size);

        let progress = serde_json::json!({
            "downloaded": downloaded,
            "total_size": total_size
        });
        app.emit_all("file_download_progress", progress).unwrap();
    //     // println!("speed => {:?}", speed);
    }

    return Ok(());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            download_file_to_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
