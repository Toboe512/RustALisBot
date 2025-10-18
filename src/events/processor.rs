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
        let response = self.tg.updates(self.offset, limit)
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

        let updates = response.unwrap().result;

        let mut result = Vec::new();

        if updates.is_empty() { return Ok(result); }

        for update in updates.iter() {
            result.push(event(update.clone()));
        }

        self.offset = updates[updates.len() - 1].update_id + 1;

        Ok(result)
    }

    pub async fn process(&mut self, event: Event) -> Result<(), String> {
        //println!("process: {}", event.text); //TODO добавить дебаг
        let err = String::from("can't process message unknown event type");
        match event.event_type {
            EventType::Message | EventType::Image => {
                self.process_message(event).await
            }
            _ => {
                Err(err)
            }
        }
    }

    pub async fn do_cmd(&mut self, text: String, chat_id: i32, username: String) -> Result<(), String> {
        let text1 = text.trim();
        let cmd: Vec<&str> = text1.split(" ").collect();

        //TODO добавить дебаг

        match cmd[0] {
            START_CMD => self.send_hello(chat_id).await,
            HELP_CMD => self.send_hello(chat_id).await,
            _ => {
                self.tg.send_message(chat_id, MSG_UNKNOWN_COMMAND).await;
                Err(String::from(MSG_UNKNOWN_COMMAND))
            }
        }
    }

    pub async fn process_message(&mut self, event: Event) -> Result<(), String> {
        self.do_cmd(event.text, event.meta.chat_id, event.meta.user_name).await
    }

    pub async fn send_hello(&mut self, chat_id: i32) -> Result<(), String> {
        self.tg.send_message(chat_id, MSG_HELLO).await
    }
}

fn event(udp: Update) -> Event {
    let udp_type = fetch_type(udp.clone());
    Event {
        event_type: udp_type,
        text: fetch_text(udp.clone()),
        meta: Meta {
            chat_id: udp.clone().message.unwrap().chat.id,
            user_name: udp.clone().message.unwrap().from.username,
        },
    }
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
