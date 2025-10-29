use serde::de;
use std::fmt::Display;

use crate::types::User;
use serde::{Deserialize, Serialize};

/// Type of the entity.
/// Currently, can be “mention” (@username),
/// “hashtag” (#hashtag or #hashtag@chatusername),
/// “cashtag” ($USD or $USD@chatusername),
/// “bot_command” (/start@jobs_bot),
/// “url” (https://telegram.org),
/// “email” (do-not-reply@telegram.org),
/// “phone_number” (+1-212-555-0123),
/// “bold” (bold text),
/// “italic” (italic text),
/// “underline” (underlined text),
/// “strikethrough” (strikethrough text),
/// “spoiler” (spoiler message),
/// “blockquote” (block quotation),
/// “expandable_blockquote” (collapsed-by-default block quotation),
/// “code” (monowidth string),
/// “pre” (monowidth block),
/// “text_link” (for clickable text URLs),
/// “text_mention” (for users without usernames),
/// “custom_emoji” (for inline custom emoji stickers)
#[derive(Debug, Serialize)]
pub enum EntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Blockquote,
    ExpandableBlockquote,
    Code,
    Pre,
    TextLink,
    TextMention,
    CustomEmoji,
    Other(String),
}

impl<'de> Deserialize<'de> for EntityType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EntityTypeVisitor;

        impl<'de> de::Visitor<'de> for EntityTypeVisitor {
            type Value = EntityType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("mention, hashtag, cashtag, botcommand, url, email, phone_number, bold, italic, code, pre, text_link, text_mention, custom_emoji, or any other string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "mention" => Ok(EntityType::Mention),
                    "hashtag" => Ok(EntityType::Hashtag),
                    "cashtag" => Ok(EntityType::Cashtag),
                    "bot_command" => Ok(EntityType::BotCommand),
                    "url" => Ok(EntityType::Url),
                    "email" => Ok(EntityType::Email),
                    "phone_number" => Ok(EntityType::PhoneNumber),
                    "bold" => Ok(EntityType::Bold),
                    "italic" => Ok(EntityType::Italic),
                    "underline" => Ok(EntityType::Underline),
                    "strikethrough" => Ok(EntityType::Strikethrough),
                    "spoiler" => Ok(EntityType::Spoiler),
                    "code" => Ok(EntityType::Code),
                    "pre" => Ok(EntityType::Pre),
                    "text_link" => Ok(EntityType::TextLink),
                    "text_mention" => Ok(EntityType::TextMention),
                    "custom_emoji" => Ok(EntityType::CustomEmoji),
                    _ => Ok(EntityType::Other(v.to_string())),
                }
            }
        }
        deserializer.deserialize_identifier(EntityTypeVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Entity {{ type: {:?}, offset: {}, length: {}",
            self.entity_type, self.offset, self.length
        )?;

        if let Some(url) = &self.url {
            write!(f, ", url: {url}")?;
        }
        if let Some(user) = &self.user {
            write!(f, ", user: {user}")?;
        }
        if let Some(lang) = &self.language {
            write!(f, ", language: {lang}")?;
        }
        if let Some(cei) = &self.custom_emoji_id {
            write!(f, ", custom_emoji_id: {cei}")?;
        }

        write!(f, " }}")
    }
}
