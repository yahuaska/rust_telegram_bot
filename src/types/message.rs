use crate::types::{Chat, Entity, UserOption};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic: Option<serde_json::Value>,
    pub from: UserOption,
    pub sender_chat: Option<serde_json::Value>,
    pub sender_boost_count: Option<i64>,
    pub sender_business_bot: Option<serde_json::Value>,
    pub date: i64,
    pub business_connection_id: Option<serde_json::Value>,
    pub chat: Chat,
    forward_origin: Option<serde_json::Value>,
    is_topic_message: Option<bool>,
    is_automatic_forward: Option<bool>,
    reply_to_message: Option<serde_json::Value>,
    external_reply: Option<serde_json::Value>,
    quote: Option<serde_json::Value>,
    reply_to_story: Option<serde_json::Value>,
    reply_to_checklist_task_id: Option<i64>,
    via_bot: Option<serde_json::Value>,
    edit_date: Option<i64>,
    has_protected_content: Option<bool>,
    is_from_offline: Option<bool>,
    is_paid_post: Option<bool>,
    media_group_id: Option<i64>,
    author_signature: Option<String>,
    paid_star_count: Option<i64>,
    pub text: Option<String>,
    pub entities: Option<Vec<Entity>>,
    link_preview_options: Option<serde_json::Value>,
    suggested_post_info: Option<serde_json::Value>,
    effect_id: Option<String>,
    animation: Option<serde_json::Value>,
    audio: Option<serde_json::Value>,
    document: Option<serde_json::Value>,
    paid_media: Option<serde_json::Value>,
    photo: Option<serde_json::Value>,
    sticker: Option<serde_json::Value>,
    story: Option<serde_json::Value>,
    video: Option<serde_json::Value>,
    video_note: Option<serde_json::Value>,
    voice: Option<serde_json::Value>,
    caption: Option<String>,
    caption_entities: Option<serde_json::Value>,
    show_caption_above_media: Option<bool>,
    has_media_spoiler: Option<bool>,
    checklist: Option<serde_json::Value>,
    dice: Option<serde_json::Value>,
    game: Option<serde_json::Value>,
    poll: Option<serde_json::Value>,
    venue: Option<serde_json::Value>,
    location: Option<serde_json::Value>,
    new_chat_members: Option<serde_json::Value>,
    left_chat_member: Option<serde_json::Value>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<serde_json::Value>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    channel_chat_created: Option<bool>,
    message_auto_delete_timer_changed: Option<serde_json::Value>,
    migrate_to_chat_id: Option<i64>,
    migrate_from_chat_id: Option<i64>,
    pinned_message: Option<serde_json::Value>,
    invoice: Option<serde_json::Value>,
    successful_payment: Option<serde_json::Value>,
    refunded_payment: Option<serde_json::Value>,
    users_shared: Option<serde_json::Value>,
    chat_shared: Option<serde_json::Value>,
    gift: Option<serde_json::Value>,
    unique_gift: Option<serde_json::Value>,
    connected_website: Option<String>,
    write_access_allowed: Option<bool>,
    passport_data: Option<serde_json::Value>,
    proximity_alert_triggered: Option<serde_json::Value>,
    boost_added: Option<serde_json::Value>,
    chat_background_set: Option<serde_json::Value>,

    checklist_tasks_added: Option<serde_json::Value>,
    checklist_tasks_done: Option<serde_json::Value>,

    directed_message_price_changed: Option<serde_json::Value>,

    forum_topic_created: Option<serde_json::Value>,
    forum_topic_edited: Option<serde_json::Value>,
    forum_topic_closed: Option<serde_json::Value>,
    forum_topic_reopened: Option<serde_json::Value>,

    general_forum_topic_hidden: Option<serde_json::Value>,
    general_forum_topic_unhidden: Option<serde_json::Value>,

    giveaway_created: Option<serde_json::Value>,
    giveaway: Option<serde_json::Value>,
    giveaway_winners: Option<serde_json::Value>,
    giveaway_completed: Option<serde_json::Value>,

    paid_message_price_changed: Option<serde_json::Value>,

    suggested_post_approved: Option<serde_json::Value>,
    suggested_post_approval_failed: Option<serde_json::Value>,
    suggested_post_declined: Option<serde_json::Value>,
    suggested_post_paid: Option<serde_json::Value>,
    suggested_post_refunded: Option<serde_json::Value>,

    video_chat_scheduled: Option<serde_json::Value>,
    video_chat_started: Option<serde_json::Value>,
    video_chat_ended: Option<serde_json::Value>,
    video_chat_participants_invited: Option<serde_json::Value>,

    web_app_data: Option<serde_json::Value>,
    reply_markup: Option<serde_json::Value>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entitites = match &self.entities {
            Some(ent) => {
                let mut txt = String::from("\nEntities:\n");
                for (index, entity) in ent.iter().enumerate() {
                    if (index + 1) < ent.len() {
                        txt = format!("{}{}, ", txt, entity);
                    } else {
                        txt = format!("{}{}", txt, entity);
                    }
                }
                txt
            }
            None => String::new(),
        };
        write!(
            f,
            "Message ID: {}\nFrom: {}\nChat: {:#}\nText: {}{entitites}",
            self.message_id,
            self.from,
            self.chat,
            match &self.text {
                Some(txt) => txt,
                None => "",
            }
        )
    }
}
