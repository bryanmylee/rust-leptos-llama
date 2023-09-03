use std::cell::RefCell;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Conversation {
    latest_message_id: RefCell<i32>,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            latest_message_id: RefCell::new(0),
            messages: vec![],
        }
    }

    fn next_message_id(&self) -> i32 {
        let mut message_id = self.latest_message_id.borrow_mut();
        *message_id += 1;
        *message_id
    }

    pub fn add_user_message(&mut self, text: &str) {
        let message = Message {
            text: text.to_string(),
            from_user: true,
            id: self.next_message_id(),
        };

        self.messages.push(message);
    }

    pub fn add_assistant_waiting(&mut self) {
        let model_message = Message {
            text: String::from("..."),
            from_user: false,
            id: self.next_message_id(),
        };

        self.messages.push(model_message);
    }

    pub fn resolve_assistant_waiting(&mut self, response: &str) {
        self.messages
            .iter_mut()
            .filter(|m| !m.from_user)
            .last()
            .expect("contains a previous message from the assistant")
            .text = response.to_string();
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Message {
    pub from_user: bool,
    pub text: String,
    pub id: i32,
}
