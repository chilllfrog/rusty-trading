use std::str::FromStr;

use crate::base::{ChainTvl, CoinTvlMetrics};
use async_trait::async_trait;
use chrono::Local;
pub use hyper::{body::HttpBody as _, Client, Uri};
use hyper::{client::HttpConnector, Error};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

pub struct DefilLama {
    host_name: &'static str,
    client: Box<Client<HttpsConnector<HttpConnector>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    #[serde(rename = "gecko_id")]
    gecko_id: Option<String>,
    tvl: Option<f32>,
    token_symbol: Option<String>,
    cmc_id: Option<String>,
    name: Option<String>,
}

impl Default for DefilLama {
    fn default() -> Self {
        Self {
            host_name: Default::default(),
            client: Box::new(Client::builder().build(HttpsConnector::new())),
        }
    }
}

impl DefilLama {
    pub fn new() -> Self {
        Self {
            host_name: "https://api.llama.fi",
            ..Default::default()
        }
    }
}

#[async_trait]
impl CoinTvlMetrics for DefilLama {
    async fn get_coins_tvl(&self) -> Result<Vec<ChainTvl>, Error> {
        let str = format!("{}{}", self.host_name, "/v2/chains");
        let uri = Uri::from_str(str.as_str()).unwrap();
        let res = self.client.as_ref().get(uri).await?;
        println!("status: {}", res.status());
        let buf = hyper::body::to_bytes(res).await?;
        println!("body: {:?}", buf);
        let json_result = String::from_utf8(buf.to_vec()).unwrap();
        let chains: Vec<Chain> = serde_json::from_str::<Vec<Chain>>(json_result.as_str()).unwrap();
        let current_timestamp: u64 = Local::now().timestamp() as u64;
        let chain_tvls = chains
            .into_iter()
            .map(|chain| ChainTvl {
                symbol: chain.token_symbol.unwrap_or_default(),
                name: chain.name.unwrap_or_default(),
                tvl: chain.tvl.unwrap_or_default(),
                timestamp: current_timestamp,
            })
            .collect();
        return Ok(chain_tvls);
    }
}
