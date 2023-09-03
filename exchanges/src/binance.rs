use std::{io::Error, str::FromStr};

use crate::exchange::{Exchange, Ticker24Hr};
use async_trait::async_trait;
use hyper::client::HttpConnector;
pub use hyper::{body::HttpBody as _, Client, Uri};
use hyper_tls::HttpsConnector;

pub struct Binance {
    client: Box<Client<HttpsConnector<HttpConnector>>>,
    host_name: &'static str,
}

impl Default for Binance {
    fn default() -> Self {
        Self {
            client: Box::new(Client::builder().build(HttpsConnector::new())),
            host_name: Default::default(),
        }
    }
}

impl Binance {
    pub fn new() -> Self {
        Self {
            host_name: "https://data-api.binance.vision",
            ..Default::default()
        }
    }
}

#[async_trait]
impl Exchange for Binance {
    async fn ping(&self) -> Result<bool, Error> {
        println!("enter ping...");
        let url = format!("{}{}", self.host_name, "/api/v3/ping");
        let uri = Uri::from_str(&url).unwrap();
        let res = self.client.as_ref().get(uri).await.unwrap();
        println!("status: {}", res.status());
        let buf = hyper::body::to_bytes(res).await.unwrap();
        println!("body: {:?}", buf);
        return Ok(true);
    }

    async fn symbol_ticker(&self, symbols: Vec<&str>) -> Result<Vec<Ticker24Hr>, Error> {
        println!("enter symbol_ticker...");
        let url = format!("{}{}", self.host_name, "/api/v3/ticker/24hr");
        let uri = Uri::from_str(&url).unwrap();
        let res = self.client.as_ref().get(uri).await.unwrap();
        let buf = hyper::body::to_bytes(res).await.unwrap();
        let json_result = String::from_utf8(buf.to_vec()).unwrap();
        let all_tickers: Vec<Ticker24Hr> =
            serde_json::from_str::<Vec<Ticker24Hr>>(json_result.as_str()).unwrap();
        let symbol_tickers: Vec<Ticker24Hr> = all_tickers
            .into_iter()
            .filter(|ticker| symbols.contains(&ticker.symbol.as_str()))
            .collect();
        return Ok(symbol_tickers);
    }
}
