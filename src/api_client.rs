use crate::types::{BotConfig, GetMeResponse, Update};
use async_stream::stream;
use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;

type UrlFormatter = fn(&str, &str) -> String;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUpdatesResponse {
    ok: bool,
    pub result: Vec<Update>,
}

pub trait HttpClient {
    type Error;

    fn get<'a>(
        &'a self,
        url: &'a str,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send + 'a;
    fn post<'a>(
        &'a self,
        url: &'a str,
        body: &'a str,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send + 'a;
    fn format_error(&self, error: Self::Error) -> String;
}

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

    pub async fn get_updates(&self) -> Option<GetUpdatesResponse> {
        let url = format!(
            "{}?offset={}&timeout={}",
            self.url("getUpdates"),
            self.bot_config.offset,
            self.bot_config.polling_timeout
        );
        println!("Calling: {url}");
        let resp = self.client.get(&url).await;

        match resp {
            Ok(resp) => {
                println!("resp: {resp:#}");
                let result = serde_json::from_str(&resp);
                match result {
                    Ok(updates_response) => Some(updates_response),
                    Err(err) => {
                        println!("{:#?}\nfrom: {}\n", err, resp);
                        None
                    }
                }
            }
            Err(err) => {
                println!("Err on request: {:#?}", self.client.format_error(err));
                None
            }
        }
    }

    pub async fn yield_updates(&self) -> impl Stream<Item = Update> {
        let url = format!(
            "{}?offset={}&timeout={}",
            self.url("getUpdates"),
            self.bot_config.offset,
            self.bot_config.polling_timeout
        );
        let client = self.client.clone();
        stream! {
            println!("Calling: {url}");
            if let Ok(resp) = client.get(&url).await {
                let result: Result<serde_json::Value, _> = serde_json::from_str(&resp);
                if let Ok(result) = result {
                    if let Some(obj) = result.get("ok") {
                        if obj.as_bool().unwrap_or(false) {
                            if let Some(obj) = result.get("result") {
                                if let Some(updates) = obj.as_array() {
                                    for json_update in updates.iter() {
                                        if let Ok(update) = serde_json::from_value::<Update>(json_update.clone())
                                        {
                                            println!("Got update: {}", update.get_update_id());
                                            yield update;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn get_me(&mut self) -> Option<GetMeResponse> {
        let url = self.url("getMe");
        println!("Calling: {url}");
        let resp = self.client.get(&url).await;

        match resp {
            Ok(resp) => {
                let result = serde_json::from_str(&resp);
                match result {
                    Ok(me_response) => Some(me_response),
                    Err(err) => {
                        println!("{:#?}\nfrom: {}\n", err, resp);
                        None
                    }
                }
            }
            Err(err) => {
                println!("Err on request: {:#?}", self.client.format_error(err));
                None
            }
        }
    }

    fn url(&self, method: &str) -> String {
        (self.url_format)(&self.bot_config.token, method)
    }
}

pub struct ApiClient<T: HttpClient> {
    // Implementation details
    client: T,
    url_format: UrlFormatter,
    bot_config: BotConfig,
}
