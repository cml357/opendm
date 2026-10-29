use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: usize,
    pub start_byte: u64,
    pub end_byte: u64,
    pub current_byte: u64,
    pub is_completed: bool,
}

impl Segment {
    pub fn new(id: usize, start_byte: u64, end_byte: u64) -> Self {
        Self {
            id,
            start_byte,
            end_byte,
            current_byte: start_byte,
            is_completed: false,
        }
    }

    pub fn progress_pct(&self) -> f64 {
        let total = self.end_byte.saturating_sub(self.start_byte) + 1;
        if total == 0 { return 100.0; }
        let done = self.current_byte.saturating_sub(self.start_byte);
        (done as f64 / total as f64) * 100.0
    }

    pub fn check_done(&mut self) -> bool {
        if self.current_byte > self.end_byte {
            self.is_completed = true;
        }
        self.is_completed
    }

    pub fn partition(total_bytes: u64, count: usize) -> Vec<Segment> {
        if count == 0 || total_bytes == 0 { return Vec::new(); }
        let chunk_size = total_bytes / count as u64;
        let mut segments = Vec::with_capacity(count);

        for i in 0..count {
            let start = i as u64 * chunk_size;
            let end = if i == count - 1 { total_bytes - 1 } else { start + chunk_size - 1 };
            segments.push(Segment::new(i, start, end));
        }
        segments
    }
}
