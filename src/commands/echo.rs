use crate::api_client::ApiClient;
use crate::core::Bot;
use crate::core::Command;
use crate::core::CommandHandler;
use crate::http_client::HttpClient;
use async_trait::async_trait;
use std::sync::Arc;

pub struct EchoCommandHandler<T>
where
    T: HttpClient,
{
    api_client: Arc<ApiClient<T>>,
}

impl<T> EchoCommandHandler<T>
where
    T: HttpClient,
{
    pub fn new(api_client: Arc<ApiClient<T>>) -> Self {
        EchoCommandHandler { api_client }
    }
}

#[async_trait]
impl<T> CommandHandler for EchoCommandHandler<T>
where
    T: HttpClient + Send + Sync,
{
    async fn handle(&self, ctx: Arc<Bot>, command: Command) {
        let _ = ctx;
        self.api_client
            .send_message(
                command.message.chat.id(),
                command
                    .message
                    .text
                    .as_ref()
                    .map(|text| text.clone())
                    .unwrap_or_default(),
            )
            .await;
    }
}
