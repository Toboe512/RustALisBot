use log::{debug, error, info};
use serde_json::{Value};
use reqwest::{Error, Method, Response, StatusCode};
use crate::clients::telegram::types::UpdatesResponse;

pub const GET_UPDATES_METHOD: &str = "getUpdates";
const SENS_MESSAGE_METHOD: &str = "sendMessage";

pub struct TgClient {
    host: String,
    base_path: String,
    client: reqwest::Client,
}

impl TgClient {
    pub fn of(host: String, token: String) -> Self {
        TgClient {
            host,
            base_path: new_base_path(token),
            client: reqwest::Client::new(),
        }
    }

    pub async fn updates(&mut self, offset: i32, limit: i32) -> Result<Option<UpdatesResponse>, String> {
        let err = String::from("Can't do updates");
        let query = new_update_query(offset, limit);

        let response = self.do_request(Method::GET, GET_UPDATES_METHOD, query, Value::Null)
            .await;

        match response {
            Err(e) => {
                error!("{}: {}", err, e);
                return Err(format!("{}: {}", err, e));
            }
            Ok(..) => {}
        }

        let response = response.unwrap();

        match response {
            None => {
                error!("{}: NoneUpdatesResponse", err);
                return Err(format!("{}: NoneUpdatesResponse", err));
            }
            Some(ref r) => {
                if r.status() != StatusCode::OK {
                    error!("{}: Status: {}", err,  r.status());
                   // TODO Нужен??? return Err(format!("{}: Status: {}", err,  r.status()));
                }
            }
        }

        let response = response.unwrap().json::<UpdatesResponse>().await;

        match response {
            Ok(r) => { Ok(Some(r)) }
            Err(e) => {
                error!("{}: {}", err, e);
                Err(format!("{}: {}", err, e))
            }
        }
    }

    pub async fn send_message(&mut self, chat_id: i32, text: &str) -> Result<(), String> {
        let err = String::from("Can't sand message");

        let query = new_message_query(chat_id, text);

        let res = self.do_request(Method::POST, SENS_MESSAGE_METHOD, query, Value::Null)
            .await;

        match res {
            Ok(..) => {
                info!("Sand message: \"\"\" {} \"\"\" in chat id: {}", text, chat_id);
                Ok(())
            },
            Err(e) => {
                error!("{}: {}", err, e);
                Err(format!("{}: {}", err, e))
            }
        }
    }

    async fn do_request(&mut self, http_method: Method, method: &str, query: String, body: Value) -> Result<Option<Response>, Error> {
        let url = new_url(&*self.host, &self.base_path, method, query);

        //debug!("Do request URL: {} Body: {}", url, body);

        let response = self.client
            .request(http_method, url)
            .json(&body)
            .header("Content-Type", "application/json")
            .send().await?;
        Ok(Some(response))
    }
}

fn new_message_query(chat_id: i32, text: &str) -> String {
    format!("?chat_id={}&text={}", chat_id, text)
}

fn new_update_query(offset: i32, limit: i32) -> String {
    format!("?offset={}&limit={}", offset, limit)
}

fn new_url(host: &str, base_path: &str, method: &str, query: String) -> String {
    format!("https://{}/{}/{}{}", host, base_path, method, query)
}

fn new_base_path(token: String) -> String {
    "bot".to_string() + &token
}


