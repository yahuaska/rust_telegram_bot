use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    id: i64,
    #[serde(rename = "type")]
    chat_type: String,
    first_name: Option<String>,
    last_name: Option<String>,
    username: Option<String>,
}

impl Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chat_name = format!(
            "{}{}",
            self.first_name.clone().unwrap_or(String::new()),
            self.last_name.clone().unwrap_or(String::new()),
        );
        let results = vec![
            write!(f, "Chat("),
            write!(f, "{chat_name}"),
            write!(f, "(#{}, ", self.id),
            write!(f, "{})", self.chat_type),
            write!(f, ")"),
        ];
        for r in results {
            if r.is_err() {
                return r;
            }
        }
        return Ok(());
        //     "Chat(\nid: {}\nchat_type:{}\n{chat_name}\n)",
        //     self.id, self.chat_type
        // );
    }
}
