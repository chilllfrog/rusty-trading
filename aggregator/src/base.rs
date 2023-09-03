use async_trait::async_trait;
use hyper::Error;
use serde_derive::{Deserialize, Serialize};

#[async_trait]
pub trait CoinTvlMetrics {
    async fn get_coins_tvl(&self) -> Result<Vec<ChainTvl>, Error>;
}
#[derive(Serialize, Deserialize)]
pub struct ChainTvl {
    pub symbol: String,
    pub name: String,
    pub tvl: f32,
    pub timestamp: u64,
}
