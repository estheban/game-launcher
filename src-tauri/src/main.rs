// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::min;
use std::env::consts;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use futures::stream::StreamExt;
use reqwest::Client;
use std::path::PathBuf;
use tauri::Manager;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use sha2::{Sha256, Digest};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_platform() -> String {
    let os = consts::OS;
    return os.to_string();
}

async fn compute_sha256(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

#[tauri::command]
async fn download_file_to_path(
    url: String,
    path: String,
    hash: String,
    file_permissions: u32,
    app: tauri::AppHandle) -> Result<(), String> {
    fs::create_dir_all(PathBuf::from(path.clone()).parent().unwrap()).await.expect("Failure creating download folder");

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

    // @todo: handle 404 and other errors, if the file don't exist the total_size will fail, this should return a failure to the frontend
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

        //     println!("downloaded => {}", downloaded);
        //     println!("total_size => {}", total_size);

        let progress = serde_json::json!({
            "downloaded": downloaded,
            "hash": hash
        });
        app.emit_all("file_download_progress", progress).unwrap();
    }

    // let permissions = Permissions::from_mode(str::parse::<u32>(&file_permissions).unwrap());
    let permissions = Permissions::from_mode(file_permissions);
    fs::set_permissions(path.clone(), permissions).await.expect("Failed to set permissions");

    // File hash check
    let actual_hash = compute_sha256(&path).await.map_err(|_| format!("Failed to compute SHA-256 hash of `{}` file", &path))?;
    if actual_hash != hash {
        println!("Error: SHA-256 hash of `{}` file does not match expected hash", &path);
        // return Err(format!("SHA-256 hash of `{}` file does not match expected hash", &path));
    }
    println!("Config: {} Actual: {}", hash, actual_hash);

    return Ok(());
}

#[tauri::command]
fn run_program(path: String) -> Result<(), String> {
    println!("Starting: {}", path);
    // let output = std::process::Command::new("open /Users/estheban/git/games/testInstall/SurvivalGame.app")
    // @todo: support windows
    // @todo: get the pid so we can update the ui saying the game is running and allow killing the game.
    std::process::Command::new("open").args(&[path])
        .spawn()
        .expect("Failed to execute command");

    // if !output.status.success() {
    //     return Err(String::from_utf8_lossy(&output.stderr).to_string());
    // }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_platform,
            download_file_to_path,
            run_program
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
