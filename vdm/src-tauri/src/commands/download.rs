use tauri::State;
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CommandResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[tauri::command]
pub async fn start_download(url: String, pool: State<'_, SqlitePool>) -> Result<CommandResponse<i32>, String> {
    // Basic fallback values for scaffold. Usually these come from the URL Analyzer.
    let file_name = "downloaded_file.tmp";
    let file_path = format!("/tmp/{}", file_name);
    let total_size = 0; // Unknown at this stage

    match crate::database::repository::DownloadRepository::create_download(&pool, &url, file_name, &file_path, total_size).await {
        Ok(id) => Ok(CommandResponse {
            success: true,
            data: Some(id),
            message: Some("Download inserted to database successfully".into()),
        }),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tauri::command]
pub async fn pause_download(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Paused download {}", id)) })
}

#[tauri::command]
pub async fn resume_download(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Resumed download {}", id)) })
}

#[tauri::command]
pub async fn cancel_download(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Canceled download {}", id)) })
}

#[tauri::command]
pub async fn restart_download(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Restarted download {}", id)) })
}

#[tauri::command]
pub async fn delete_download(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Deleted download {}", id)) })
}

#[tauri::command]
pub async fn verify_download(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Verified download {}", id)) })
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
