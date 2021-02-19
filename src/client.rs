use crate::dto::{CreateAccountDto, SignAndGetDto, UpdateAccountDto, GetApiKeyDto};
use reqwest::Client;

pub struct ClientApiAccount {
    client: Client
}

impl ClientApiAccount {
    pub fn new() -> ClientApiAccount {
        ClientApiAccount { client: Client::new() }
    }

    pub async fn create_account(&self, create_account_dto: &CreateAccountDto) -> Result<String, reqwest::Error> {
        self.client
            .post("http://localhost:3030/account")
            .json(create_account_dto)
            .send()
            .await?
            .text()
            .await
    }

    pub async fn sign_get(&self, sign_and_get_dto: &SignAndGetDto) -> Result<String, reqwest::Error> {
        self.client
            .get("http://localhost:3030/account")
            .json(sign_and_get_dto)
            .send()
            .await?
            .text()
            .await
    }

    pub async fn remove_account(&self, account_id: &str) -> Result<String, reqwest::Error> {
        self.client
            .delete(&format!("http://localhost:3030/account/{}", account_id))
            .send()
            .await?
            .text()
            .await
    }

    pub async fn remove_key(&self, account_id: &str) -> Result<String, reqwest::Error> {
        self.client
            .delete(&format!("http://localhost:3030/key/account/{}", account_id))
            .send()
            .await?
            .text()
            .await
    }

    pub async fn account_update(&self, update_account_dto: &UpdateAccountDto) -> Result<String, reqwest::Error> {
        self.client
            .patch("http://localhost:3030/account")
            .json(update_account_dto)
            .send()
            .await?
            .text()
            .await
    }

    pub async fn get_api_key(&self, get_api_key_dto: &GetApiKeyDto) -> Result<String, reqwest::Error> {
        self.client
            .get("http://localhost:3030/key/account")
            .json(get_api_key_dto)
            .send()
            .await?
            .text()
            .await
    }
}