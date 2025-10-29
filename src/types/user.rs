use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<serde_json::Value>,
    pub is_premium: Option<serde_json::Value>,
    pub added_to_attachment_menu: Option<serde_json::Value>,
    pub can_join_groups: Option<serde_json::Value>,
    pub can_read_all_group_messages: Option<serde_json::Value>,
    pub supports_inline_queries: Option<serde_json::Value>,
    pub can_connect_to_business: Option<serde_json::Value>,
    pub has_main_web_app: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserOption(pub Option<User>);

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!(
            "{}{}{}",
            self.first_name,
            match &self.last_name {
                Some(n) => format!(" {}", n),
                None => String::new(),
            },
            match &self.username.as_ref() {
                Some(n) => format!(" [@{}]", n),
                None => String::new(),
            }
        );
        let bot_label = if self.is_bot { " (bot)" } else { "" };
        write!(f, "User({name}{bot_label})")
    }
}

impl Display for UserOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(x) => write!(f, "{}", x),
            None => write!(f, ""),
        }
    }
}
