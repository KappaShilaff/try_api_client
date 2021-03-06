use crate::client::{ClientApiAccount};
use crate::dto::{CreateAccountDto, ExchangeName, SignAndGetDto, UpdateAccountDto, GetApiKeyDto};

mod client;
mod dto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientApiAccount::new();

    println!("body {:#?}", client.create_account(&CreateAccountDto {
        uid: "uid_lol".to_string(),
        exchange: ExchangeName::Binance,
        api_key: "api_string".to_string(),
        sign_key: Option::Some("mem".to_string()),
    }).await);

    println!("body {:#?}", client.sign_get(&SignAndGetDto {
        uid: "uid_lol".to_string(),
        exchange: ExchangeName::Binance,
        data_to_sign: vec![12, 34],
    }).await);

    println!("body {:#?}", client.account_update(&UpdateAccountDto {
        uid: "uid_lol".to_string(),
        exchange: ExchangeName::Binance,
        api_key: Option::Some("api_string".to_string()),
        sign_key: Option::Some("mem".to_string()),
    }).await);

    println!("body {:#?}", client.get_api_key(&GetApiKeyDto {
        uid: "uid lol".to_string(),
        exchange: ExchangeName::Binance,
    }).await);

    println!("body {:#?}", client.remove_key(&"uid_lol".to_string()).await);

    println!("body {:#?}", client.remove_account(&"uid_lol".to_string()).await);

    Ok(())
}
