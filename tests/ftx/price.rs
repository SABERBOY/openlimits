use ftx::rest::{GetMarket, Rest, Result};
use super::client::init_signed as init;

#[tokio::test]
async fn get_btc_price() -> Result<()> {
    let api = Rest::new(init());
    let price = api.request(GetMarket::new("BTC/USD")).await?.price;
    println!("1 BTC is worth {} USD.", price.unwrap());
    Ok(())
}