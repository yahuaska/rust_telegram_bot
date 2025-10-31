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

impl Chat {
    pub fn id(&self) -> i64 {
        self.id
    }
}

impl Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chat_name = format!(
            "{}{}",
            self.first_name.clone().unwrap_or_default(),
            self.last_name.clone().unwrap_or_default(),
        );
        let results = vec![
            write!(f, "Chat("),
            write!(f, "{chat_name}"),
            write!(f, "(#{}, ", self.id),
            write!(f, "{})", self.chat_type),
            write!(f, ")"),
        ];
        for r in results {
            r?
        }
        Ok(())
    }
}
