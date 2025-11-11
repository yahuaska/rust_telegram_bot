use crate::core::bot::Bot;
use crate::types::{EntityType, Message};
use async_trait::async_trait;
use std::sync::Arc;

/// Enum for all possible bot commands
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum BotCommand {
    Video,
    Echo,
    Unknown,
}

/// Wrapper for a command that is known to the bot and its arguments
#[derive(Debug)]
pub struct Command {
    pub command: BotCommand,
    pub args: Vec<String>,
    pub message: Message,
}

/// Decides which command to execute based on the message content and its arguments
/// Consumes the message
pub fn decide_command(message: Message) -> Option<Command> {
    let entities = message.entities.as_ref()?;
    let mut command = BotCommand::Unknown;
    let mut args = Vec::new();
    for entity in entities {
        match entity.entity_type {
            EntityType::BotCommand => {
                let start: usize = entity.offset as usize;
                let end: usize = start + entity.length as usize;
                match &message.text {
                    Some(text) => {
                        let cmd = text[start..end].trim_start_matches("/").to_lowercase();
                        command = match cmd.as_str() {
                            "video" => BotCommand::Video,
                            "echo" => BotCommand::Echo,

                            _ => {
                                println!("Unknown command: <{}>", cmd);
                                BotCommand::Unknown
                            }
                        };
                    }
                    None => {
                        println!("Received command without text");
                    }
                }
            }
            EntityType::Url => {
                let start: usize = entity.offset as usize;
                let end: usize = start + entity.length as usize;
                match &message.text {
                    Some(text) => {
                        let url = text[start..end].to_string();
                        args.push(url);
                    }
                    None => {
                        println!("Received URL without text");
                    }
                }
            }
            _ => {
                println!(
                    "Received message without command: {:#?}",
                    entity.entity_type
                );
            }
        }
    }
    match command {
        BotCommand::Unknown => {
            println!("Received unknown command");
            None
        }
        _ => Some(Command {
            command,
            args,
            message,
        }),
    }
}

#[async_trait]
pub trait CommandHandler: Send + Sync {
    async fn handle(&self, ctx: Arc<Bot>, command: Command);
}

#[async_trait]
pub trait CommandRegistry: Send + Sync {
    async fn register(&self, name: BotCommand, handler: Arc<dyn CommandHandler>);
    async fn dispatch(&self, ctx: Arc<Bot>, command: Command);
}
