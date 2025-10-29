use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
    pub can_join_groups: bool,
    pub can_read_all_group_messages: bool,
    pub supports_inline_queries: bool,
    pub can_connect_to_business: bool,
    pub has_main_web_app: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetMeResponse {
    pub ok: bool,
    pub result: Bot,
}
