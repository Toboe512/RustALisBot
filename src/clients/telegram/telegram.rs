use std::thread;
use std::time::Duration;
use log::{debug, error, info};
use serde_json::{Value};
use reqwest::{Error, Method, Response, StatusCode};
use crate::errors::errors::log_err;
use crate::clients::telegram::types::UpdatesResponse;

pub const GET_UPDATES_METHOD: &str = "getUpdates";
const SENS_MESSAGE_METHOD: &str = "sendMessage";

const RETRY_PERIOD: Duration = Duration::from_secs(3);

pub struct TgClient {
    host: String,
    base_path: String,
    client: reqwest::Client,
}

impl TgClient {
    pub fn of(host: &str, token: &str) -> Self {
        TgClient {
            host: String::from(host),
            base_path: new_base_path(token),
            client: reqwest::Client::new(),
        }
    }

    pub async fn updates(&self, offset: i32, limit: i32) -> Result<UpdatesResponse, String> {
        let err = "Can't do updates";
        let query = new_update_query(offset, limit);

        let response = self
            .do_request(Method::GET, GET_UPDATES_METHOD, &query, &Value::Null)
            .await
            .map_err(|e| log_err(err, e))?;

        if response.status() != StatusCode::OK {
            error!("{}: Status: {}", err,  response.status());
            // Приостановить поток на RETRY_PERIOD сек (период ретрая)
            thread::sleep(RETRY_PERIOD);
            return Err(format!("{}: Status: {}", err, response.status()));
        }
        response.json::<UpdatesResponse>().await.map_err(|e| log_err(&err, e))
    }

    pub async fn send_message(&self, chat_id: i32, text: &str) -> Result<(), String> {
        let err = "Can't sand message";
        let query = new_message_query(chat_id, text);

        self.do_request(Method::POST, SENS_MESSAGE_METHOD, &query, &Value::Null)
            .await
            .map_err(|e| log_err(err, e))
            .map(|_| info!("Sand message: \"\"\" {} \"\"\" in chat id: {}", text, chat_id))?;
        Ok(())
    }

    async fn do_request(&self, http_method: Method, method: &str, query: &str, body: &Value) -> Result<Response, Error> {
        let url = new_url(&*self.host, &self.base_path, method, query);

        // debug!("Do request URL: {} Body: {}", &url, &body);

        let response = self.client
            .request(http_method, &url)
            .json(body)
            .header("Content-Type", "application/json")
            .send().await?;

        // debug!("Do response URL: {} Response: {:?}", &url,  &response);
        Ok(response)
    }
}

fn new_message_query(chat_id: i32, text: &str) -> String {
    format!("?chat_id={}&text={}", chat_id, text)
}

fn new_update_query(offset: i32, limit: i32) -> String {
    format!("?offset={}&limit={}", offset, limit)
}

fn new_url(host: &str, base_path: &str, method: &str, query: &str) -> String {
    format!("https://{}/{}/{}{}", host, base_path, method, query)
}

fn new_base_path(token: &str) -> String {
    "bot".to_string() + &token
}


