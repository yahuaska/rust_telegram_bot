use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: String,
}

impl Payload {
    pub fn new(chat_id: i64, text: String, parse_mode: String) -> Self {
        Self {
            chat_id,
            text,
            parse_mode,
        }
    }
}
