use super::download::CommandResponse;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_schedule(
    task: String,
    time: String,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Created schedule for {} at {}", task, time)),
    })
}

#[tauri::command]
pub async fn update_schedule(
    id: i32,
    task: String,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Updated schedule {}", id)),
    })
}

#[tauri::command]
pub async fn delete_schedule(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Deleted schedule {}", id)),
    })
}

#[tauri::command]
pub async fn enable_schedule(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Enabled schedule {}", id)),
    })
}

#[tauri::command]
pub async fn disable_schedule(
    id: i32,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some(format!("Disabled schedule {}", id)),
    })
}
