use tauri::State;
use sqlx::SqlitePool;
use super::download::CommandResponse;

#[tauri::command]
pub async fn get_notifications(pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: Some("[]".into()), message: None })
}

#[tauri::command]
pub async fn mark_as_read(id: i32, pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some("Notification marked as read".into()) })
}

#[tauri::command]
pub async fn clear_notifications(pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse { success: true, data: None, message: Some("Notifications cleared".into()) })
}
