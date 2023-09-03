use std::time::SystemTime;

use crate::result::{LimiterError, Result};
use async_trait::async_trait;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct TokenBucket {
    tps: f32,
    bucket_size: i32,
    tokens_left: i32,
    last_time: SystemTime,
    mutex: Mutex<bool>,
}

#[async_trait]
pub trait RateLimiter {
    ///
    /// acquire tokens from bucket
    ///
    async fn acquire(&mut self, tokens: u32) -> Result<()>;
}

impl TokenBucket {
    pub fn new(tps: f32, bucket_size: i32) -> TokenBucket {
        Self {
            tps,
            bucket_size,
            tokens_left: bucket_size,
            last_time: SystemTime::now(),
            mutex: Mutex::new(false),
        }
    }
}

#[async_trait]
impl RateLimiter for TokenBucket {
    async fn acquire(&mut self, permits: u32) -> Result<()> {
        let _ = self.mutex.lock().await;
        let current = SystemTime::now();
        let elapsed_millis = SystemTime::duration_since(&current, self.last_time)
            .expect("time backwards")
            .as_millis() as f32;
        let generated_tokens = (elapsed_millis * self.tps / 1000.0).floor() as i32;
        println!("generated tokens:{}", generated_tokens);
        let tokens = self.tokens_left - (permits as i32) + generated_tokens;
        println!("tokens:{}", tokens);
        let tokens_actual = std::cmp::min(std::cmp::max(tokens, 0), self.bucket_size);
        if tokens >= 0 {
            self.tokens_left = tokens_actual;
            self.last_time = current;
            return Ok(());
        } else {
            return Err(LimiterError::INSUFFICIENTOKEN(self.tokens_left as u64));
        }
    }
}
