use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

/// Limiteur de vitesse basé sur l'algorithme "Token Bucket" (Seau à jetons)
/// Permet un bridage précis (ex: 500 Ko/s, 2 Mo/s) ou le Mode Silencieux
#[derive(Clone)]
pub struct BandwidthLimiter {
    max_bytes_per_sec: Arc<AtomicU64>,
    tokens: Arc<Mutex<f64>>,
    last_leak: Arc<Mutex<Instant>>,
}

impl BandwidthLimiter {
    /// 0 = Illimité
    pub fn new(max_bytes_per_sec: u64) -> Self {
        Self {
            max_bytes_per_sec: Arc::new(AtomicU64::new(max_bytes_per_sec)),
            tokens: Arc::new(Mutex::new(max_bytes_per_sec as f64)),
            last_leak: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Change dynamiquement la limite en plein téléchargement
    pub fn set_limit(&self, bytes_per_sec: u64) {
        self.max_bytes_per_sec.store(bytes_per_sec, Ordering::Relaxed);
    }

    /// Régule le débit d'un paquet de données
    pub async fn throttle(&self, bytes_to_consume: usize) {
        let max_speed = self.max_bytes_per_sec.load(Ordering::Relaxed);
        if max_speed == 0 {
            return; // Débit illimité
        }

        let mut tokens_lock = self.tokens.lock().await;
        let mut last_leak_lock = self.last_leak.lock().await;

        let now = Instant::now();
        let elapsed = now.duration_since(*last_leak_lock).as_secs_f64();
        *last_leak_lock = now;

        // Régénérer les jetons disponibles
        *tokens_lock = (*tokens_lock + elapsed * max_speed as f64).min(max_speed as f64 * 2.0);

        if *tokens_lock < bytes_to_consume as f64 {
            let deficit = bytes_to_consume as f64 - *tokens_lock;
            let wait_secs = deficit / max_speed as f64;
            *tokens_lock = 0.0;
            sleep(Duration::from_secs_f64(wait_secs)).await;
        } else {
            *tokens_lock -= bytes_to_consume as f64;
        }
    }
}
