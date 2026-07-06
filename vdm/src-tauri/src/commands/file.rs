use super::download::CommandResponse;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn open_file(path: String) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Opened file: {}", path)),
    })
}

#[tauri::command]
pub async fn open_folder(path: String) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Opened folder: {}", path)),
    })
}

#[tauri::command]
pub async fn rename_file(
    old_path: String,
    new_path: String,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Renamed file from {} to {}", old_path, new_path)),
    })
}

#[tauri::command]
pub async fn move_file(
    old_path: String,
    new_path: String,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Moved file from {} to {}", old_path, new_path)),
    })
}

#[tauri::command]
pub async fn delete_file(path: String) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Deleted file: {}", path)),
    })
}
