pub mod defillama;
pub mod base;

#[cfg(test)]
mod defillama_tests {

    #[tokio::test]
    async fn tvl_test() {
        use crate::defillama::DefilLama;
        use crate::base::CoinTvlMetrics;
        let defillama = DefilLama::new();
        let vec = defillama.get_coins_tvl().await.unwrap();
        println!("{}", serde_json::to_string(&vec).unwrap());
    }
}
