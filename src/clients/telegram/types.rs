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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Update {
    #[serde(default)]
    pub update_id: i32,
    #[serde(default)]
    pub message: Option<Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(default)]
    pub message_id: i32,
    #[serde(default)]
    pub message_thread_id: i32,
    #[serde(default)]
    pub from: User,
    #[serde(default)]
    pub sender_chat: Chat,
    #[serde(default)]
    pub sander_business_bot: User,
    #[serde(default)]
    pub date: i32,
    #[serde(default)]
    pub business_connection_id: String,
    #[serde(default)]
    pub chat: Chat,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub photo: Option<Vec<PhotoSize>>,
    #[serde(default)]
    pub caption: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhotoSize {
    #[serde(default)]
    pub file_id: String,
    #[serde(default)]
    pub file_unique_id: String,
    #[serde(default)]
    pub width: i32,
    #[serde(default)]
    pub height: i32,
    #[serde(default)]
    pub file_size: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub is_bot: bool,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Chat {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub type_: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub is_forum: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commands {
    #[serde(default)]
    pub commands: Vec<BotCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotCommand {
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResult {
    pub ok: bool,
    pub result: bool,
}