pub mod rate_limiter;
pub mod result;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::rate_limiter::RateLimiter;

    #[tokio::test]
    async fn token_bucket_test() {
        use crate::rate_limiter::TokenBucket;
        let mut token_bucket = TokenBucket::new(1.0, 10000);
        println!("start call some method..");
        for _ in 0..10000 {
            let result = token_bucket.acquire(1).await;
            std::thread::sleep(Duration::from_millis(200));
            if result.is_err() {
                eprintln!("failed to call the method, error:{:?}", result.unwrap_err());
            } else {
                println!("successfully get the permits");
            }
        }
    }
}
