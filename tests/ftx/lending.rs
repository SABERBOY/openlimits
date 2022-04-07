use {
    ftx::{
        rest::{
            GetLendingInfo, GetLendingRates, GetMyLendingHistory, LendingInfo, Rest,
            SubmitLendingOffer,
        },
    },
    std::env,
};
use super::client::init_signed as init;

#[tokio::test]
async fn lending() {
    let api = Rest::new(init());

    let lending_info = api.request(GetLendingInfo {}).await.unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{:#?}", lending_info);

        let lending_rates = api.request(GetLendingRates {}).await.unwrap();
        println!("{:#?}", lending_rates);

        let lending_history = api.request(GetMyLendingHistory::default()).await.unwrap();
        println!("{:#?}", lending_history);
    } else {
        let coin = &args[1];

        for LendingInfo {
            lendable, min_rate, ..
        } in lending_info
            .iter()
            .filter(|lending_info| lending_info.coin == *coin)
        {
            let size = lendable.floor();
            println!("Submitting lending offer for {}: {}", coin, size);
            api.request(SubmitLendingOffer {
                coin: coin.clone(),
                size,
                rate: min_rate.unwrap_or_default(),
            })
                .await
                .unwrap();
            println!("Offer :{}", 123)
        }
    }
}