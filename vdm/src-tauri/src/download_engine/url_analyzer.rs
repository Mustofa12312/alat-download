use reqwest::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlMetadata {
    pub url: String,
    pub file_name: String,
    pub total_size: u64,
    pub mime_type: String,
    pub supports_resume: bool,
}

pub struct UrlAnalyzer;

impl UrlAnalyzer {
    pub async fn analyze(client: &reqwest::Client, url: &str) -> Result<UrlMetadata, String> {
        let resp = client
            .head(url)
            .send()
            .await
            .map_err(|e| format!("Failed to send HEAD request: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Server returned error: {}", resp.status()));
        }

        let headers = resp.headers();

        // 1. Content-Length
        let total_size = if let Some(len) = headers.get(CONTENT_LENGTH) {
            let len_str = len.to_str().unwrap_or("0");
            u64::from_str(len_str).unwrap_or(0)
        } else {
            0
        };

        // 2. Accept-Ranges
        let supports_resume = headers
            .get(ACCEPT_RANGES)
            .and_then(|val| val.to_str().ok())
            .map(|s| s.contains("bytes"))
            .unwrap_or(false);

        // 3. Content-Type
        let mime_type = headers
            .get(CONTENT_TYPE)
            .and_then(|val| val.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();

        // 4. File Name (from Content-Disposition or URL)
        let file_name = Self::extract_filename(url, headers.get(CONTENT_DISPOSITION));

        Ok(UrlMetadata {
            url: url.to_string(),
            file_name,
            total_size,
            mime_type,
            supports_resume,
        })
    }

    fn extract_filename(url: &str, disposition: Option<&reqwest::header::HeaderValue>) -> String {
        if let Some(disp) = disposition {
            if let Ok(disp_str) = disp.to_str() {
                if let Some(idx) = disp_str.find("filename=") {
                    let mut name = disp_str[idx + 9..].trim().to_string();
                    if name.starts_with('"') && name.ends_with('"') {
                        name = name[1..name.len() - 1].to_string();
                    }
                    if !name.is_empty() {
                        return name;
                    }
                }
            }
        }

        // Fallback to URL parsing
        if let Ok(parsed_url) = reqwest::Url::parse(url) {
            if let Some(segments) = parsed_url.path_segments() {
                if let Some(last) = segments.last() {
                    if !last.is_empty() {
                        return last.to_string();
                    }
                }
            }
        }

        "downloaded_file".to_string()
    }
}
