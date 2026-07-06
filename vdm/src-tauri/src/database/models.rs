use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct DownloadRecord {
    pub id: i32,
    pub url: String,
    pub file_name: String,
    pub file_path: String,
    pub total_size: i64,
    pub downloaded_size: i64,
    pub download_speed: f64,
    pub status: i32,
    pub priority: i32,
    pub category: Option<String>,
    pub checksum: Option<String>,
    pub created_at: Option<String>, // Simplest way to handle datetime from sqlite in typical structs
    pub updated_at: Option<String>,
}
