use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation { messages: vec![] }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub from_user: bool,
    pub text: String,
}
