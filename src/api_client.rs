use crate::types::{BotConfig, GetMeResponse, Update};
use async_stream::stream;
use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;

macro_rules! unwrap_or {
    ($expr:expr, continue) => {
        match $expr {
            Some(val) => val,
            None => continue,
        }
    };
    ($expr:expr, $on_err:expr, continue) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                $on_err(e);
                continue;
            }
        }
    };
    ($expr:expr, return) => {
        unwrap_or!($expr, return ())
    };

    ($expr:expr, $on_err:expr, return) => {
        unwrap_or!($expr, $on_err, return ())
    };
    ($expr:expr, $on_err:expr, return $rt:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                $on_err(e);
                return $rt;
            }
        }
    };
    ($expr:expr, return $rt:expr) => {
        match $expr {
            Some(val) => val,
            None => return $rt,
        }
    };
}

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
        body: String,
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
            let resp = unwrap_or!(client.get(&url).await, |e| {
                println!("Err on request: {:#?}", client.format_error(e));
            }, return);
            let result = unwrap_or!(serde_json::from_str::<serde_json::Value>(&resp), |e| println!("Failed to deserialize: {e}"), return);
            let obj = unwrap_or!(result.get("ok"), return);
            if obj.as_bool().unwrap_or(false) {
                let obj = unwrap_or!(result.get("result"), return);
                let updates = unwrap_or!(obj.as_array(), return);
                for json_update in updates.iter() {
                    let update = unwrap_or!(serde_json::from_value::<Update>(json_update.clone()), |e| println!("Failed to deserialize update: {e}"), continue);
                    println!("Got update: {}", update.get_update_id());
                    yield update;
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
