use serde_json::Value;
use crate::clients::telegram::types::BaseResult;

use crate::clients::telegram::types;
use crate::TG_BOT_HOST;

use reqwest::{Error, Method};
use crate::UpdatesResponse;

pub const GET_UPDATES_METHOD: &str = "getUpdates";
const SENS_MESSAGE_METHOD: &str = "sendMessage";
const SEND_PHOTO: &str = "sendPhoto";
const SET_MY_COMMANDS: &str = "setMyCommands";


pub struct TgClient {
    host: String,
    base_path: String,
    client: reqwest::Client,
}

impl TgClient {
    pub fn of(host: String, token: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(TgClient {
            host,
            base_path: new_base_path(token),
            client: reqwest::Client::new(),
        })
    }

    pub async fn updates(&mut self, offset: i32, limit: i32) -> Result<Option<UpdatesResponse>, Error> {
        let q = new_query(offset, limit);

        self.do_request(Method::GET, GET_UPDATES_METHOD, q, Value::Null)
            .await
    }


    pub async fn do_request(&mut self, http_method: Method, method: &str, query: String, body: Value) -> Result<Option<UpdatesResponse>, Error> {
        let url = new_url(&self.base_path, method, query);

        let response = self.client
            .request(http_method, url)
            .header("Content-Type", "application/json")
            .send().await?
            .json::<UpdatesResponse>().await?;
        Ok(Some(response))
    }
}

pub fn new_query(offset: i32, limit: i32) -> String {
    format!("?offset={}&limit={}", offset, limit)
}

pub fn new_url(base_path: &str, method: &str, query: String) -> String {
    format!("https://{}/{}/{}{}", TG_BOT_HOST, base_path, method, query)
}

pub fn new_base_path(token: String) -> String {
    "bot".to_string() + &token
}


