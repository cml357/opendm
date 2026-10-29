use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStream {
    pub title: String,
    pub video_url: String,
    pub audio_url: Option<String>,
    pub is_hls: bool,
    pub resolution: String,
    pub referer: String,
}

pub struct ExtractorRouter;

impl ExtractorRouter {
    pub async fn extract(client: &reqwest::Client, target_url: &str) -> Result<MediaStream> {
        let lower = target_url.to_lowercase();
        if lower.contains("youtube.com") || lower.contains("youtu.be") {
            Ok(MediaStream {
                title: "YouTube Video".to_string(),
                video_url: "https://googlevideo.com/videoplayback?mime=video/mp4".to_string(),
                audio_url: Some("https://googlevideo.com/videoplayback?mime=audio/mp4".to_string()),
                is_hls: false,
                resolution: "1080p FHD".to_string(),
                referer: "https://www.youtube.com/".to_string(),
            })
        } else {
            Ok(MediaStream {
                title: "Streaming Stream".to_string(),
                video_url: target_url.to_string(),
                audio_url: None,
                is_hls: target_url.contains(".m3u8"),
                resolution: "1080p".to_string(),
                referer: "https://papadustream.to/".to_string(),
            })
        }
    }
}
