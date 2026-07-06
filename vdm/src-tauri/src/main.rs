#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod database;
mod download_engine;
pub mod events;
mod intelligence;
pub mod torrent_engine;
pub mod video_engine;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match database::init_db(&handle).await {
                    Ok(pool) => {
                        println!("Database initialized successfully");
                        handle.manage(pool);
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize database: {}", e);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(commands::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
