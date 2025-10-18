extern crate reqwest;
extern crate serde_json;
extern crate log;

use std::env;
use std::process::exit;
use log::{error, info};
use clients::telegram::telegram;
use consumer::consumer::Consumer;
use events::processor::Processor;
use telegram::TgClient;

mod clients;
mod events;
mod consumer;
mod utils;

const TG_BOT_HOST: &str = "api.telegram.org";
const BATCH_SIZE: i32 = 100;

fn help() {
    error!("token is not specified");
    exit(1);
}


#[tokio::main]
async fn main() -> Result<(), String> {
    // Инициализация логгера.
    env_logger::init();
    info!("Приложение запущено");

    let client = TgClient::of(TG_BOT_HOST.to_string(), mast_token());

    let events_processor = Processor::of(client);

    let mut consumer = Consumer::of(events_processor, BATCH_SIZE);

    consumer.start().await

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

