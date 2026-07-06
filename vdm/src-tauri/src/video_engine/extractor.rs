use tokio::process::Command;
use serde_json::Value;

pub struct Extractor;

impl Extractor {
    pub async fn analyze_url(url: &str) -> Result<Value, String> {
        // Run yt-dlp -J to get JSON metadata
        // For production, the yt-dlp executable might be bundled as a sidecar
        let output = Command::new("yt-dlp")
            .arg("-J")
            .arg(url)
            .output()
            .await
            .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("yt-dlp error: {}", err));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let parsed: Value = serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(parsed)
    }
}
