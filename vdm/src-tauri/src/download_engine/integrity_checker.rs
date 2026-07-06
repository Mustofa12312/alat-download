use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use sha2::{Sha256, Digest};

pub struct IntegrityChecker;

impl IntegrityChecker {
    pub async fn verify_sha256<P: AsRef<Path>>(path: P, expected_hash: &str) -> Result<bool, String> {
        let mut file = File::open(path)
            .await
            .map_err(|e| format!("Failed to open file for verification: {}", e))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let count = file.read(&mut buffer)
                .await
                .map_err(|e| format!("Failed to read file: {}", e))?;
            
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }

        let result = hasher.finalize();
        let hash_hex = format!("{:x}", result);

        Ok(hash_hex.eq_ignore_ascii_case(expected_hash))
    }
}
