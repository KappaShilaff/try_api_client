use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ExchangeName {
    Binance,
    HitBtc,
    Kraken,
    Okex,
    Kucoin,
    Bitfinex,
    Huobi,
    Quoine,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateAccountDto {
    pub uid: String,
    pub exchange: ExchangeName,
    pub api_key: String,
    pub sign_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignAndGetDto {
    pub uid: String,
    pub exchange: ExchangeName,
    pub data_to_sign: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateAccountDto {
    pub uid: String,
    pub exchange: ExchangeName,
    pub api_key: Option<String>,
    pub sign_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetApiKeyDto {
    pub uid: String,
    pub exchange: ExchangeName,
}

