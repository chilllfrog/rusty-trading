pub mod binance;
pub mod exchange;

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn symbol_ticker_test() {
        use crate::binance::Binance;
        use crate::exchange::Exchange;
        let binance = Binance::new();
        let tickers = binance.symbol_ticker(vec!["WAVESUSDT", "BTCUSDT"]).await.unwrap();
        for ticker in tickers {
            println!("token symbol:{}, token price:{}, low price:{}, high price:{}", ticker.symbol, ticker.last_price, ticker.low_price, ticker.high_price);
        }
    }
}
