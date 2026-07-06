use super::download::CommandResponse;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn get_statistics(
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: Some("{}".into()),
        message: None,
    })
}

#[tauri::command]
pub async fn reset_statistics(
    pool: State<'_, SqlitePool>,
) -> Result<CommandResponse<String>, String> {
    Ok(CommandResponse {
        success: true,
        data: None,
        message: Some("Statistics reset".into()),
    })
}
