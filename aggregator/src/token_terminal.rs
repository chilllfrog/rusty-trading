use std::str::FromStr;

use crate::base::{ChainTvl, CoinTvlMetrics};
use async_trait::async_trait;
use chrono::Local;
pub use hyper::{body::HttpBody as _, Client, Uri};
use hyper::{client::HttpConnector, Error};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};

pub struct TokenTerminal {
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

impl TokenTerminal {
    pub fn new() -> Self {
        Self {
            client: Box::new(Client::builder().build(HttpsConnector::new())),
            host_name: "https://api.tokenterminal.com",
        }
    }
}

#[async_trait]
impl CoinTvlMetrics for TokenTerminal {
    async fn get_coins_tvl(&self) -> Result<Vec<ChainTvl>, Error> {
        let str = format!("{}{}", self.host_name, "/v2/projects");
        let uri = Uri::from_str(str.as_str()).unwrap();
        let res = self.client.as_ref().get(uri).await?;
        println!("status: {}", res.status());
        let buf = hyper::body::to_bytes(res).await?;
        println!("body: {:?}", buf);
        let json_result = String::from_utf8(buf.to_vec()).unwrap();
        let chains: Vec<Chain> = serde_json::from_str::<Vec<Chain>>(json_result.as_str()).unwrap();
        let chain_tvls = chains
            .into_iter()
            .map(|chain| {
                return ChainTvl {
                    symbol: chain.token_symbol.unwrap_or_default(),
                    tvl: chain.tvl.unwrap_or_default(),
                    timestamp: Local::now().timestamp() as u64,
                };
            })
            .collect();
        return Ok(chain_tvls);
    }
}
