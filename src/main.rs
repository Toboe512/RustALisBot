extern crate serde_json;
extern crate reqwest;

use std::collections::HashMap;
use std::env;
use std::process::exit;
use serde::Deserialize;
use clients::telegram::telegram;
use serde_json::json;
use reqwest::{Error, Method, Url};
use clients::telegram::types::{BaseResult, UpdatesResponse};
use telegram::{GET_UPDATES_METHOD, new_base_path, new_query, new_url, TgClient};

mod clients;


const TG_BOT_HOST: &str = "api.telegram.org";
const STORAGE_FILE_PATH: &str = "files_storage";
const STORAGE_SQLITE_PATH: &str = "data/sqlite/storage.db";
const BATCH_SIZE: i32 = 100;


fn help() {
    println!("token is not specified");
    exit(1);
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let c = TgClient::of(TG_BOT_HOST.to_string(), mast_token());

    let result = c.unwrap()
        .updates(0, BATCH_SIZE).await?;

    println!("Response {}", result.unwrap().ok);

    Ok(())
}

fn mast_token() -> String {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let cmd = &args[1];
            let tkn = &args[2];
            if cmd.eq("-tg-bot-token") {
                return String::from(tkn);
            }
            help()
        }
        _ => { help() }
    }
    String::new()
}

