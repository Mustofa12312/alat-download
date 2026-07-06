use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use std::str::FromStr;

pub mod models;
pub mod repository;

pub async fn init_db(app_handle: &tauri::AppHandle) -> Result<SqlitePool, sqlx::Error> {
    // We will store the database in the app data directory
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    std::fs::create_dir_all(&app_dir).unwrap_or(());

    let db_path = app_dir.join("vdm_data.db");
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

    let options = SqliteConnectOptions::from_str(&db_url)?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .foreign_keys(true)
        .busy_timeout(std::time::Duration::from_millis(5000));

    let pool = SqlitePool::connect_with(options).await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| sqlx::Error::Configuration(e.into()))?;

    Ok(pool)
}
