use crate::types::{EntityType, Message};

/// Enum for all possible bot commands
#[derive(Debug)]
pub enum BotCommand {
    Video,
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
pub fn decide_command(msg: &Message) -> Option<Command> {
    msg.entities.as_ref()?;
    let entities = msg.entities.as_ref().unwrap();
    let mut command = BotCommand::Unknown;
    let mut args = Vec::new();
    for entity in entities {
        match entity.entity_type {
            EntityType::BotCommand => {
                let start: usize = entity.offset as usize;
                let end: usize = start + entity.length as usize;
                match &msg.text {
                    Some(text) => {
                        let cmd = text[start..end].trim_start_matches("/").to_lowercase();
                        command = match cmd.as_str() {
                            "video" => BotCommand::Video,
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
                match &msg.text {
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
        _ => Some(Command { command, args }),
    }
}
