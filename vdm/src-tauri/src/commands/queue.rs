use tauri::State;
use sqlx::SqlitePool;
use super::download::CommandResponse;

#[tauri::command]
pub async fn add_to_queue(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Added {} to queue", id)) })
}

#[tauri::command]
pub async fn remove_from_queue(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Removed {} from queue", id)) })
}

#[tauri::command]
pub async fn reorder_queue(id: i32, new_index: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some(format!("Reordered queue item {} to {}", id, new_index)) })
}

#[tauri::command]
pub async fn clear_queue(pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some("Cleared queue".into()) })
}

#[tauri::command]
pub async fn start_queue(pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some("Started queue".into()) })
}

#[tauri::command]
pub async fn pause_queue(pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some("Paused queue".into()) })
}
