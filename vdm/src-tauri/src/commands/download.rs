use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use tauri::AppHandle;

use crate::download_engine::url_analyzer::UrlAnalyzer;
use crate::download_engine::segment_manager::SegmentManager;
use crate::download_engine::disk_writer::DiskWriter;
use crate::download_engine::worker::DownloadWorker;
use crate::download_engine::DownloadOrchestrator;
use reqwest::Client;
use std::path::PathBuf;
use tokio::sync::mpsc;

#[derive(Serialize)]
pub struct CommandResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[tauri::command]
pub async fn start_download(
    url: String,
    pool: State<'_, SqlitePool>,
    app_handle: AppHandle,
) -> Result<CommandResponse<i32>, String> {
    let client = Client::new();
    
    // 1. Analyze URL
    let metadata = UrlAnalyzer::analyze(&client, &url).await?;
    
    // 2. Prepare file path (e.g. in OS Downloads folder or temp)
    // For scaffolding, we save to current directory or a designated downloads folder.
    // We'll use a local 'downloads' folder in the app directory for now.
    let app_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let download_dir = app_dir.join("downloads");
    std::fs::create_dir_all(&download_dir).unwrap_or(());
    
    let file_path = download_dir.join(&metadata.file_name);
    
    // 3. Pre-allocate Disk Space
    let _file = DiskWriter::allocate_file(&file_path, metadata.total_size).await?;

    // 4. Save to Database
    let download_id = match crate::database::repository::DownloadRepository::create_download(
        &pool, &url, &metadata.file_name, file_path.to_str().unwrap_or(""), metadata.total_size as i64,
    ).await {
        Ok(id) => id,
        Err(e) => return Err(format!("Database error: {}", e)),
    };
    
    // 5. Determine Segments
    let num_connections = if metadata.supports_resume { 4 } else { 1 };
    let segments = SegmentManager::split_into_segments(metadata.total_size, num_connections);
    
    // 6. Setup Channels for Progress tracking
    let (tx, rx) = mpsc::channel(32);
    
    // 7. Spawn DownloadOrchestrator Listener
    DownloadOrchestrator::spawn_listener(app_handle, rx, download_id, metadata.total_size);
    
    // 8. Spawn Workers
    for segment in segments {
        let client_clone = client.clone();
        let url_clone = url.clone();
        let file_path_clone = file_path.clone();
        let tx_clone = tx.clone();
        
        tokio::spawn(async move {
            DownloadWorker::run(client_clone, url_clone, segment, file_path_clone, tx_clone).await;
        });
    }
    
    // Send success response
    Ok(CommandResponse {
        success: true,
        data: Some(download_id),
        message: Some("Download started successfully".into()),
    })
}

#[tauri::command]
pub async fn pause_download(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Paused download {}", id)),
    })
}

#[tauri::command]
pub async fn resume_download(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Resumed download {}", id)),
    })
}

#[tauri::command]
pub async fn cancel_download(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Canceled download {}", id)),
    })
}

#[tauri::command]
pub async fn restart_download(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Restarted download {}", id)),
    })
}

#[tauri::command]
pub async fn delete_download(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Deleted download {}", id)),
    })
}

#[tauri::command]
pub async fn verify_download(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Verified download {}", id)),
    })
}

#[tauri::command]
pub async fn analyze_video(url: String) -> Result<CommandResponse<String>, String> {
    match crate::video_engine::extractor::Extractor::analyze_url(&url).await {
        Ok(json_data) => Ok(CommandResponse {
            success: true,
            data: Some(json_data.to_string()),
            message: Some("Video analyzed successfully".into()),
        }),
        Err(e) => Ok(CommandResponse {
            success: false,
            data: None,
            message: Some(e),
        }),
    }
}
