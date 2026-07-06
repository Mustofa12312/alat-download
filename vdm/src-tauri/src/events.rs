use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgressPayload {
    pub id: i32,
    pub speed: u64, // bytes per second
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadStatusPayload {
    pub id: i32,
    pub status: i32, // e.g. 0: Pending, 1: Downloading, 2: Paused, 3: Completed, 4: Error
    pub error_message: Option<String>,
}
