pub mod url_analyzer;
pub mod http_client;
pub mod connection_manager;
pub mod segment_manager;
pub mod resume_engine;
pub mod retry_engine;
pub mod buffer_manager;
pub mod disk_writer;
pub mod integrity_checker;
pub mod worker;

use std::sync::Arc;
use tauri::{AppHandle, Manager, Emitter};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use crate::events::DownloadProgressPayload;

pub struct DownloadOrchestrator {
    pub app_handle: AppHandle,
}

impl DownloadOrchestrator {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    /// Spawns a background task to listen to worker events and emit throttled updates
    pub fn spawn_listener(app_handle: AppHandle, mut rx: mpsc::Receiver<worker::WorkerEvent>, id: i32, total_bytes: u64) {
        tokio::spawn(async move {
            let mut downloaded = 0;
            let mut last_emitted_downloaded = 0;
            let mut ticker = interval(Duration::from_millis(100)); // Throttle to 100ms
            
            loop {
                tokio::select! {
                    Some(event) = rx.recv() => {
                        match event {
                            worker::WorkerEvent::Progress { downloaded: chunk, .. } => {
                                downloaded += chunk;
                            },
                            worker::WorkerEvent::Completed { .. } => {
                                // A segment completed. Could check if all segments are done here.
                            },
                            worker::WorkerEvent::Error { error, .. } => {
                                // Emit error status
                                let _ = app_handle.emit("vdm://download-status", crate::events::DownloadStatusPayload {
                                    id,
                                    status: 4,
                                    error_message: Some(error),
                                });
                            }
                        }
                    }
                    _ = ticker.tick() => {
                        // If progress changed, calculate speed and emit
                        if downloaded > last_emitted_downloaded {
                            // Simple speed calculation based on tick interval (100ms)
                            let bytes_diff = downloaded - last_emitted_downloaded;
                            let speed = bytes_diff * 10; // bytes per second
                            
                            let payload = DownloadProgressPayload {
                                id,
                                speed,
                                downloaded_bytes: downloaded,
                                total_bytes,
                            };
                            
                            let _ = app_handle.emit("vdm://download-progress", payload);
                            last_emitted_downloaded = downloaded;
                        }
                    }
                }
            }
        });
    }
}
