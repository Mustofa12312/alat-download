use super::download::CommandResponse;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn get_settings(pool: State<'_, SqlitePool>) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: Some("{}".into()),
        message: None,
    })
}

#[tauri::command]
pub async fn save_settings(
    key: String,
    value: String,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some("Settings saved".into()),
    })
}

#[tauri::command]
pub async fn reset_settings(
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some("Settings reset".into()),
    })
}

#[tauri::command]
pub async fn import_settings(
    path: String,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some("Settings imported".into()),
    })
}

#[tauri::command]
pub async fn export_settings(
    path: String,
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some("Settings exported".into()),
    })
}
