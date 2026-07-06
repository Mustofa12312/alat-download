use super::disk_writer::DiskWriter;
use super::segment_manager::Segment;
use futures_util::StreamExt;
use reqwest::header::RANGE;
use reqwest::Client;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum WorkerEvent {
    Progress { segment_index: u32, downloaded: u64 },
    Completed { segment_index: u32 },
    Error { segment_index: u32, error: String },
}

pub struct DownloadWorker;

impl DownloadWorker {
    pub async fn run(
        client: Client,
        url: String,
        segment: Segment,
        file_path: PathBuf,
        tx: mpsc::Sender<WorkerEvent>,
    ) {
        let range_header = format!("bytes={}-{}", segment.start_byte, segment.end_byte);

        let mut request = client.get(&url);

        // Only set range if it's not the dummy single segment for non-resumable
        if segment.end_byte > 0 {
            request = request.header(RANGE, range_header);
        }

        let resp = match request.send().await {
            Ok(r) => r,
            Err(e) => {
                let _ = tx
                    .send(WorkerEvent::Error {
                        segment_index: segment.index,
                        error: e.to_string(),
                    })
                    .await;
                return;
            }
        };

        if !resp.status().is_success() {
            let _ = tx
                .send(WorkerEvent::Error {
                    segment_index: segment.index,
                    error: format!("HTTP Error: {}", resp.status()),
                })
                .await;
            return;
        }

        let mut stream = resp.bytes_stream();
        let mut current_offset = segment.start_byte;

        // Open file once per worker to avoid contention on open, though ideally
        // we'd use a shared DiskWriter channel. For simplicity, each worker opens its own handle.
        let mut file = match DiskWriter::open_for_write(&file_path).await {
            Ok(f) => f,
            Err(e) => {
                let _ = tx
                    .send(WorkerEvent::Error {
                        segment_index: segment.index,
                        error: e,
                    })
                    .await;
                return;
            }
        };

        let mut downloaded_in_segment = 0;

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx
                        .send(WorkerEvent::Error {
                            segment_index: segment.index,
                            error: e.to_string(),
                        })
                        .await;
                    return;
                }
            };

            let chunk_len = chunk.len() as u64;

            if let Err(e) = DiskWriter::write_at_offset(&mut file, current_offset, &chunk).await {
                let _ = tx
                    .send(WorkerEvent::Error {
                        segment_index: segment.index,
                        error: e,
                    })
                    .await;
                return;
            }

            current_offset += chunk_len;
            downloaded_in_segment += chunk_len;

            // Report progress
            let _ = tx
                .send(WorkerEvent::Progress {
                    segment_index: segment.index,
                    downloaded: chunk_len,
                })
                .await;
        }

        let _ = tx
            .send(WorkerEvent::Completed {
                segment_index: segment.index,
            })
            .await;
    }
}
