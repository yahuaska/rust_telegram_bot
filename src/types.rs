pub use crate::types::bot::{Bot, GetMeResponse};
pub use crate::types::chat::Chat;
pub use crate::types::entities::{Entity, EntityType};
pub use crate::types::message::Message;
pub use crate::types::user::{User, UserOption};
use serde::{Deserialize, Serialize};

pub mod bot;
pub mod chat;
pub mod entities;
pub mod message;
pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub update_id: i64,
    /// Optional. New incoming message of any kind - text, photo, sticker, etc.
    pub message: Option<Message>,
    ///Optional. New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,
    business_connection: Option<serde_json::Value>,
    business_message: Option<serde_json::Value>,
    edited_business_message: Option<serde_json::Value>,
    deleted_business_message: Option<serde_json::Value>,
    message_reaction: Option<serde_json::Value>,
    message_reaction_cound: Option<serde_json::Value>,
    inline_query: Option<serde_json::Value>,
    chosen_inline_result: Option<serde_json::Value>,
    callback_query: Option<serde_json::Value>,
    shipping_query: Option<serde_json::Value>,
    pre_checkout_query: Option<serde_json::Value>,
    purchased_paid_media: Option<serde_json::Value>,
    poll: Option<serde_json::Value>,
    poll_answer: Option<serde_json::Value>,
    my_chat_member: Option<serde_json::Value>,
    chat_member: Option<serde_json::Value>,
    chat_join_request: Option<serde_json::Value>,
    chat_boost: Option<serde_json::Value>,
    removed_chat_boost: Option<serde_json::Value>,
}

impl Update {
    pub fn get_update_id(&self) -> i64 {
        self.update_id
    }
}
