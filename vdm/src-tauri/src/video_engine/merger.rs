use tokio::process::Command;

pub struct Merger;

impl Merger {
    pub async fn merge_video_audio(video_path: &str, audio_path: &str, output_path: &str) -> Result<(), String> {
        let output = Command::new("ffmpeg")
            .arg("-y") // Overwrite
            .arg("-i")
            .arg(video_path)
            .arg("-i")
            .arg(audio_path)
            .arg("-c")
            .arg("copy")
            .arg(output_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("ffmpeg error: {}", err));
        }

        Ok(())
    }
}
