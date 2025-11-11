use crate::core::bot::Bot;
use crate::core::commands::BotCommand;
use crate::core::commands::Command;
use crate::core::commands::CommandHandler;
use crate::core::commands::CommandRegistry;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Registry {
    handlers: RwLock<HashMap<BotCommand, Arc<dyn CommandHandler>>>,
}

impl Registry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            handlers: RwLock::new(HashMap::new()),
        })
    }
}

#[async_trait]
impl CommandRegistry for Registry {
    async fn register(&self, name: BotCommand, handler: Arc<dyn CommandHandler>) {
        self.handlers.write().await.insert(name, handler);
    }

    async fn dispatch(&self, ctx: Arc<Bot>, command: Command) {
        if let Some(handler) = self.handlers.read().await.get(&command.command) {
            handler.handle(ctx, command).await;
            return;
        }
        // ctx.send_message("Command not found").await;
    }
}
