use log::{debug, error, log};
use crate::errors::errors::log_err;
use crate::utils::consts::SPACE_STR;
use crate::events::messages::MSG_UNKNOWN_COMMAND;
use crate::events::messages::MSG_HELLO;
use crate::events::types::Meta;
use crate::events::types::Event;
use crate::clients::telegram::types::Update;
use crate::events::types::EventType;
use crate::TgClient;


pub const START_CMD: &str = "/start";
pub const HELP_CMD: &str = "/help";


pub struct Processor {
    tg: TgClient,
    offset: i32,
}

impl Processor {
    pub fn of(client: TgClient) -> Self {
        Processor {
            tg: client,
            offset: 0,
        }
    }
    pub async fn fetch(&mut self, limit: i32) -> Result<Vec<Event>, String> {
        let err = String::from("can't get events");

        let response = self.tg.updates(self.offset, limit).await.map_err(|e| {
            log_err(&err, e)
        })?;

        //TODO оставлено для обработки других полей в response в будущем
        let updates = response.result;

        let mut res_evn = Vec::new();

        if updates.is_empty() { return Ok(res_evn); }

        for update in &updates {
            res_evn.push(event(update.clone()));
        }

        self.offset = updates[updates.len() - 1].update_id + 1;

        Ok(res_evn)
    }

    pub async fn process(&self, event: Event) -> Result<(), String> {
        debug!("Process event: {:?}", event);
        let err = String::from("Can't process message");
        match event.event_type {
            EventType::Message | EventType::Image => {
                self.process_message(event).await
            }
            EventType::Unknown => {
                Err(log_err(&err, "Event Type Unknown"))
            }
        }
    }

    pub async fn do_cmd(&self, text: String, chat_id: i32, username: String) -> Result<(), String> {
        let log = String::from("Do command");
        let cmd: Vec<&str> = text.trim().split(SPACE_STR).collect();

        debug!("{}: {:?} in chat id: {} for username: {}", &log, &cmd, &chat_id, &username);

        match cmd[0] {
            START_CMD => {
                debug!("{}: {}", &log, START_CMD);
                self.send_hello(chat_id).await
            }
            HELP_CMD => {
                debug!("{}: {}", &log, HELP_CMD);
                self.send_hello(chat_id).await
            }
            _ => {
                let _ = self.tg.send_message(chat_id, MSG_UNKNOWN_COMMAND).await;
                error!("{}: UNKNOWN COMMAND", &log);
                Err(format!("{}: UNKNOWN COMMAND", &log))
            }
        }
    }

    pub async fn process_message(&self, event: Event) -> Result<(), String> {
        let log = String::from("Process message");

        if event.meta.map(async |m| {
            let _ = self.do_cmd(event.text, m.chat_id, m.user_name).await;
        }).is_none() {
            error!( "{}: can't get meta", log);
            return Err(log_err(&log, "can't get meta"));
        }
        Ok(())
    }

    pub async fn send_hello(&self, chat_id: i32) -> Result<(), String> {
        self.tg.send_message(chat_id, MSG_HELLO).await
    }
}

// event метод в котором по сути происходит мепинг Update в Event с заполнением структуры Meta.
fn event(udp: Update) -> Event {
    let udp_type = fetch_type(udp.clone());

    return match udp_type {
        EventType::Message => {
            Event {
                event_type: udp_type,
                text: fetch_text(udp.clone()),
                meta: Some(Meta {
                    chat_id: udp.clone().message.unwrap().chat.id,
                    user_name: udp.clone().message.unwrap().from.username,
                    image_id: None,
                }),
            }
        }
        EventType::Image => {
            Event {
                event_type: udp_type,
                text: fetch_text(udp.clone()),
                meta: Some(Meta {
                    chat_id: udp.clone().message.unwrap().chat.id,
                    user_name: udp.clone().message.unwrap().from.username,
                    image_id: None,//udp.clone().message.unwrap().photo[0].file_id,
                }),
            }
        }
        EventType::Unknown => {
            Event {
                event_type: udp_type,
                text: fetch_text(udp.clone()),
                meta: None,
            }
        }
    };
}

fn fetch_text(udp: Update) -> String {
    match fetch_type(udp.clone()) {
        EventType::Message => udp.clone().message.unwrap().text,
        EventType::Image => udp.clone().message.unwrap().caption,
        _ => String::new()
    }
}

fn fetch_type(udp: Update) -> EventType {
    if udp.clone().message.is_none() {
        return EventType::Unknown;
    }

    let photo = udp.clone().message.unwrap().photo;

    if photo.is_none() || photo.unwrap().is_empty() {
        return EventType::Message;
    }

    EventType::Image
}
