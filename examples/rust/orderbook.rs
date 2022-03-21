use dotenv::dotenv;
use openlimits_binance::{BinanceCredentials, BinanceParameters, BinanceWebsocket};
use openlimits::prelude::*;
// use openlimits::exchange::coinbase::client::stream::CoinbaseWebsocket;
// use openlimits::exchange::coinbase::CoinbaseParameters;
use openlimits::model::websocket::OpenLimitsWebSocketMessage::OrderBook;
use openlimits::model::websocket::OpenLimitsWebSocketMessage::Trades as TradesMessage;
use openlimits::model::websocket::Subscription::OrderBookUpdates;
use openlimits::model::websocket::WebSocketResponse::Generic;
use openlimits_exchange::model::market_pair::MarketPair;
use openlimits_exchange::model::currency::Currency;
use openlimits_exchange::exchange::Environment;
use openlimits_exchange::model::websocket::Subscription::Trades;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // let coinbase_websocket = CoinbaseWebsocket::new(CoinbaseParameters::production()).await.unwrap();
    let parameters = BinanceParameters {
        credentials: Some(BinanceCredentials {
            api_key:    std::env::var("BINANCE_API_KEY").expect("Couldn't get environment variable."),
            api_secret: std::env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
    };
    let binance_websocket = BinanceWebsocket::new(parameters).await.unwrap();
    let market = MarketPair(Currency::BTC, Currency::USDT);

    binance_websocket.subscribe(OrderBookUpdates(market), move |m| {
        let r = m.as_ref();

        if let Ok(Generic(OrderBook(order_book))) = r {
            println!("{:?}", order_book)
        } else if let Err(err) = r {
            println!("{:#?}", err);
        }
    }).await.expect("Failed to subscribe to orderbook on Binance");

    let market = MarketPair(Currency::BTC, Currency::USDT);
    binance_websocket.subscribe(Trades(market), move |m| {
        let r = m.as_ref();

        if let Ok(Generic(TradesMessage(order_book))) = r {
            println!("{:?}", order_book)
        } else if let Err(err) = r {
            println!("{:#?}", err);
        }
    }).await.expect("Failed to subscribe to orderbook on Binance");

    std::thread::sleep(std::time::Duration::from_millis(50000));
}