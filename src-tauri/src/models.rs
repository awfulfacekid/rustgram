use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub text: Option<String>,
    pub chat: Chat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Message,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub result: Vec<Update>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboard {
    pub keyboard: Vec<Vec<String>>,
    pub resize_keyboard: bool,
    pub one_time_keyboard: bool,
    pub selective: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Button {
    pub text: String,
    pub handler: String,
}
