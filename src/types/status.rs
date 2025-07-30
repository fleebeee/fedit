use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Status {
    pub text: String,
    pub timestamp: Instant,
}

impl Status {
    pub fn new(text: String) -> Self {
        Status {
            text,
            timestamp: Instant::now(),
        }
    }

    pub fn is_fresh(&self) -> bool {
        self.timestamp.elapsed() < Duration::from_secs(3)
    }
}
