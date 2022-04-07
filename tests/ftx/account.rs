use ftx::rest::{GetAccount, GetFuture, GetMarket, GetPositions, Rest};
use super::client::init_signed as init;

#[tokio::test]
async fn get_account() {
    let api = Rest::new(init());
    println!("Account:");
    println!("{:#?}", api.request(GetAccount {}).await.unwrap());
    println!("Positions:");
    println!("{:#?}", api.request(GetPositions {}).await.unwrap());
    println!("GetMarket:");
    println!("{:#?}", api.request(GetMarket::new("BTC/USD")).await.unwrap());
    println!("GetFutures:");
    println!("{:#?}", api.request(GetFuture::new("BTC/USD")).await.unwrap());
}