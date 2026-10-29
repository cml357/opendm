use crate::limiter::BandwidthLimiter;
use crate::segment::Segment;
use crate::storage::FileStorage;
use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use reqwest::header::{HeaderValue, ACCEPT_RANGES, CONTENT_LENGTH, RANGE, REFERER};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::OpenOptions;
use tokio::sync::{mpsc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadOptions {
    pub url: String,
    pub output_path: PathBuf,
    pub threads: usize,
    pub referer: Option<String>,
    pub max_bytes_sec: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentProgress {
    pub id: usize,
    pub current_byte: u64,
    pub end_byte: u64,
    pub speed_bytes_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub speed_bytes_sec: u64,
    pub segments: Vec<SegmentProgress>,
    pub is_finished: bool,
}

pub struct DownloadJob {
    pub options: DownloadOptions,
    pub segments: Arc<RwLock<Vec<Segment>>>,
    pub limiter: BandwidthLimiter,
    pub is_paused: Arc<AtomicBool>,
    pub is_canceled: Arc<AtomicBool>,
    pub total_downloaded: Arc<AtomicU64>,
}

impl DownloadJob {
    pub fn new(options: DownloadOptions) -> Self {
        let max_speed = options.max_bytes_sec.unwrap_or(0);
        Self {
            options,
            segments: Arc::new(RwLock::new(Vec::new())),
            limiter: BandwidthLimiter::new(max_speed),
            is_paused: Arc::new(AtomicBool::new(false)),
            is_canceled: Arc::new(AtomicBool::new(false)),
            total_downloaded: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn start(&self, progress_tx: mpsc::Sender<DownloadProgress>) -> Result<()> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(32)
            .build()?;

        // Résolution de taille
        let res = client.head(&self.options.url).send().await?;
        let total_size = res.headers().get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

        let initial_segments = Segment::partition(total_size, self.options.threads);
        {
            let mut seg_guard = self.segments.write().await;
            *seg_guard = initial_segments.clone();
        }

        let storage = FileStorage::new(&self.options.output_path);
        storage.preallocate(total_size).await?;

        let mut tasks = Vec::new();
        for seg in initial_segments {
            let client = client.clone();
            let url = self.options.url.clone();
            let path = self.options.output_path.clone();
            let limiter = self.limiter.clone();
            let is_paused = Arc::clone(&self.is_paused);
            let is_canceled = Arc::clone(&self.is_canceled);
            let total_downloaded = Arc::clone(&self.total_downloaded);

            let task = tokio::spawn(async move {
                let mut file = OpenOptions::new().write(true).open(&path).await?;
                let range = format!("bytes={}-{}", seg.start_byte, seg.end_byte);
                let resp = client.get(&url).header(RANGE, HeaderValue::from_str(&range)?).send().await?;
                let mut stream = resp.bytes_stream();
                let mut pos = seg.start_byte;

                while let Some(chunk_res) = stream.next().await {
                    if is_canceled.load(Ordering::Relaxed) { return Ok(()); }
                    while is_paused.load(Ordering::Relaxed) { tokio::time::sleep(Duration::from_millis(200)).await; }

                    let chunk = chunk_res?;
                    limiter.throttle(chunk.len()).await; // Régulation de vitesse

                    FileStorage::write_at(&mut file, pos, &chunk).await?;
                    pos += chunk.len() as u64;
                    total_downloaded.fetch_add(chunk.len() as u64, Ordering::Relaxed);
                }
                Ok::<(), anyhow::Error>(())
            });
            tasks.push(task);
        }

        for t in tasks { t.await??; }
        Ok(())
    }
}
