use serde_json::{Value};
use reqwest::{Error, Method, Response};
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
        let err = String::from("can't do updates");
        let query = new_update_query(offset, limit);

        let response = self.do_request(Method::GET, GET_UPDATES_METHOD, query, Value::Null)
            .await;

        match response {
            Err(e) => { return Err(format!("{}: {}", err, e)); }
            Ok(..) => {}
        }

        let response = response.unwrap();

        match response {
            None => { return Err(format!("{}: NoneUpdatesResponse", err)); }
            Some(..) => {}
        }

        let response = response.unwrap().json::<UpdatesResponse>().await;

        match response {
            Ok(r) => { Ok(Some(r)) }
            Err(e) => { Err(format!("{}: {}", err, e)) }
        }
    }

    pub async fn send_message(&mut self, chat_id: i32, text: &str) -> Result<(), String> {
        let err = String::from("can't sand message");

        let query = new_message_query(chat_id, text);

        let res = self.do_request(Method::POST, SENS_MESSAGE_METHOD, query, Value::Null)
            .await;

        match res {
            Ok(..) => Ok(()),
            Err(e) => {
                Err(format!("{}: {}", err, e))
            }
        }
    }

    async fn do_request(&mut self, http_method: Method, method: &str, query: String, body: Value) -> Result<Option<Response>, Error> {
        let url = new_url(&*self.host, &self.base_path, method, query);

        //println!("Запрос на do request: {}", url); //TODO добавить дебаг

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


