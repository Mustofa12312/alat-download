use std::path::Path;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};

pub struct DiskWriter;

impl DiskWriter {
    /// Pre-allocates a sparse file of the required size.
    pub async fn allocate_file<P: AsRef<Path>>(path: P, total_size: u64) -> Result<File, String> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .await
            .map_err(|e| format!("Failed to create file: {}", e))?;

        if total_size > 0 {
            file.set_len(total_size)
                .await
                .map_err(|e| format!("Failed to pre-allocate file size: {}", e))?;
        }

        Ok(file)
    }

    /// Opens an existing file for writing chunks at specific offsets.
    pub async fn open_for_write<P: AsRef<Path>>(path: P) -> Result<File, String> {
        OpenOptions::new()
            .write(true)
            .open(path)
            .await
            .map_err(|e| format!("Failed to open file for write: {}", e))
    }

    /// Writes data at a specific offset in the file.
    pub async fn write_at_offset(file: &mut File, offset: u64, data: &[u8]) -> Result<(), String> {
        file.seek(SeekFrom::Start(offset))
            .await
            .map_err(|e| format!("Failed to seek to offset {}: {}", offset, e))?;
            
        file.write_all(data)
            .await
            .map_err(|e| format!("Failed to write data at offset {}: {}", offset, e))?;
            
        Ok(())
    }
}
