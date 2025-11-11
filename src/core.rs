pub use crate::core::bot::Bot;
pub use crate::core::command_registry::Registry;
pub use crate::core::commands::{
    decide_command, BotCommand, Command, CommandHandler, CommandRegistry,
};
pub mod bot;
mod command_registry;
mod commands;
