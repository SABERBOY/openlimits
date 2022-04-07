use dotenv::dotenv;
use std::env::var;
use ftx::options::Options;


pub fn init_signed() -> Options {
    dotenv().ok();
    Options::default()
        .authenticate(
            var("API_KEY").expect("API Key is not defined."),
            var("API_SECRET").expect("API Secret is not defined."),
        )
        .subaccount_optional(var("SUBACCOUNT").ok())
}