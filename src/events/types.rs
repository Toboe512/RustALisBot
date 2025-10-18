#[derive(Clone)]
pub enum EventType {
    Unknown,
    Message,
    Image,
}

#[derive(Clone)]
pub struct Event {
    pub(crate) event_type: EventType,
    pub(crate) text: String,
    pub meta: Meta,
}

// Meta Структура с данными которые получаем от пользователя.
#[derive(Clone)]
pub struct Meta {
    pub chat_id: i32,
    pub user_name: String,
}
