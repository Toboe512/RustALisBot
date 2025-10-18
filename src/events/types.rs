#[derive(Clone, Debug)]
pub enum EventType {
    Unknown,
    Message,
    Image,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub(crate) event_type: EventType,
    pub(crate) text: String,
    pub meta: Option<Meta>,
}

// Meta Структура с данными которые получаем от пользователя.
#[derive(Clone, Debug)]
pub struct Meta {
    pub chat_id: i32,
    pub user_name: String,
    pub image_id: Option<String>,
}
