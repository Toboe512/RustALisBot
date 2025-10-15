use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatesResponse {
    pub ok: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub error_code: i32,
    #[serde(default)]
    pub result: Vec<Update>,
    #[serde(default)]
    pub parameters: Option<ResponseParameters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseParameters {
    #[serde(default)]
    pub migrate_to_chat_id: i32,
    #[serde(default)]
    pub retry_after: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub migrate_to_chat_id: i32,
    pub retry_after: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i32,
    pub message_thread_id: i32,
    pub from: User,
    pub sender_chat: Chat,
    pub sander_business_bot: User,
    pub date: i32,
    pub business_connection_id: String,
    pub chat: Chat,
    pub text: String,
    pub photo: Vec<PhotoSize>,
    pub caption: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub title: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_forum: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commands {
    pub commands: Vec<BotCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResult {
    pub ok: bool,
    pub result: bool,
}