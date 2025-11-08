use crate::api_client::types::Payload;
use crate::http_client::HttpClient;
use crate::types::{Bot, BotConfig, Message, Update};
use futures_core::stream::Stream;
use std::collections::HashMap;

type UrlFormatter = fn(&str, &str) -> String;

impl<T: HttpClient + Clone> ApiClient<T> {
    pub fn new(client: T, url_format: UrlFormatter, bot_config: BotConfig) -> Self {
        Self {
            client,
            url_format,
            bot_config,
        }
    }

    pub fn update_offset(&mut self, offset: i64) {
        if offset >= self.bot_config.offset {
            self.bot_config.offset = offset + 1;
        }
    }

    /// Parse a JSON response into a Value.
    fn parse_json(resp: String) -> Option<serde_json::Value> {
        serde_json::from_str::<serde_json::Value>(&resp)
            .map_err(|e| println!("Error parsing response: {:#?}", e))
            .ok()
    }

    /// Parse a JSON response into a Rust type.
    fn parse_response<RT>(resp: serde_json::Value) -> Option<RT>
    where
        RT: serde::de::DeserializeOwned,
    {
        serde_json::from_value::<RT>(resp.clone())
            .map_err(|x| println!("Error parsing response: {:#?}\nfrom: {}", x, resp))
            .ok()
    }

    /// Send a message to the chat.
    pub async fn send_message(&self, chat_id: i64, text: String) -> Option<Message> {
        let url = self.url("sendMessage");
        let payload = Payload::new(chat_id, text, String::from("MarkdownV2"));
        let body = serde_json::to_string(&payload)
            .map_err(|err| println!("Error serializing payload: {:#?}", err))
            .ok()?;
        println!("Sending message: {body} to chat {}", payload.chat_id);
        self.client
            .post(&url, body)
            .await
            .map_err(|err| println!("Err on request: {:#?}", self.client.format_error(err)))
            .ok()
            .and_then(Self::parse_json)?
            .get("result")
            .and_then(|x| Self::parse_response(x.to_owned()))?
    }

    /// Send video to the chat.
    pub async fn send_video(
        &self,
        chat_id: i64,
        attach_name: &str,
        video_path: &str,
    ) -> Option<Message> {
        let url = self.url("sendVideo");
        let body = HashMap::from([
            ("chat_id".to_string(), chat_id.to_string()),
            ("video".to_string(), format!("attach://{}", attach_name)),
        ]);
        println!("Sending message: {body:#?} to chat {}", chat_id);
        self.client
            .post_multipart(&url, body, Some(video_path))
            .await
            .map_err(|err| println!("Err on request: {:#?}", self.client.format_error(err)))
            .ok()
            .and_then(Self::parse_json)?
            .get("result")
            .and_then(|x| Self::parse_response(x.to_owned()))?
    }
    async fn get_updates(client: T, url: String) -> Option<Vec<Update>> {
        println!("Calling: {url}");
        let resp = client
            .get(&url)
            .await
            .map_err(|err| println!("Err on request: {:#?}", client.format_error(err)))
            .ok()
            .and_then(Self::parse_json)?;
        if !resp.get("ok").and_then(|ok| ok.as_bool()).unwrap_or(false) {
            return None;
        }
        resp.clone()
            .get("result")
            .and_then(|result| Self::parse_response::<Vec<Update>>(result.to_owned()))
    }

    /// Yield updates from the API.
    pub async fn yield_updates(&self) -> impl Stream<Item = Update> {
        let url = format!(
            "{}?offset={}&timeout={}",
            self.url("getUpdates"),
            self.bot_config.offset,
            self.bot_config.polling_timeout
        );
        let client = self.client.clone();
        async_stream::stream! {
            match Self::get_updates(client, url).await {
                Some(updates) => {
                    for update in updates {
                        yield update;
                    }
                }
                None => {}
            }

        }
    }

    /// Get information about the bot itself.
    pub async fn get_me(&mut self) -> Option<Bot> {
        let url = self.url("getMe");
        println!("Calling: {url}");
        let payload = self
            .client
            .get(&url)
            .await
            .map_err(|err| println!("Err on request: {:#?}", self.client.format_error(err)))
            .ok()
            .and_then(Self::parse_json)?;
        if payload
            .get("ok")
            .and_then(|ok| ok.as_bool())
            .unwrap_or(false)
        {
            return None;
        }
        payload
            .get("result")
            .and_then(|result| serde_json::from_value(result.clone()).ok())
    }

    fn url(&self, method: &str) -> String {
        (self.url_format)(&self.bot_config.token, method)
    }
}

pub struct ApiClient<T: HttpClient> {
    client: T,
    url_format: UrlFormatter,
    bot_config: BotConfig,
}
