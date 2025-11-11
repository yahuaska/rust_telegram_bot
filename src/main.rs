use crate::api_client::ApiClient;
use crate::commands::EchoCommandHandler;
use crate::core::{bot, decide_command, Command, CommandRegistry, Registry};
use crate::http_client::HttpClient;
use crate::http_clients::ReqwestHttpClient;
use crate::types::Bot;
use std::sync::Arc;
use tokio::sync::mpsc;

use futures_util::StreamExt;
use std::env;

pub mod api_client;
pub mod commands;
pub mod core;
pub mod http_client;
pub mod http_clients;
pub mod types;

fn print_me(resp: Option<Bot>) {
    if let Some(resp) = resp {
        println!("Status: OK");
        println!(
            "My ID: {}\nMy name is: {} ({})",
            resp.id, resp.first_name, resp.username
        );
        if resp.is_bot {
            println!("I am a bot!");
        } else {
            println!("I am not a bot!");
        }
    } else {
        println!("Status: ERROR");
    }
}

async fn updates_loop<T>(api_client: Arc<ApiClient<T>>, tx: mpsc::Sender<Command>)
where
    T: HttpClient,
{
    print_me(api_client.get_me().await);
    loop {
        println!("Running the generator");
        let stream = api_client.yield_updates().await;
        tokio::pin!(stream);
        println!("Querring the generator");
        while let Some(update) = stream.next().await {
            api_client.update_offset(update.update_id);
            println!(
                "Update ID: {}\n{}",
                update.get_update_id(),
                match (update.edited_message.as_ref(), update.message.as_ref()) {
                    (Some(edited_message), _) => format!("Edited message: {}", edited_message),
                    (_, Some(message)) => format!("New message: {}", message),
                    _ => "No message".to_string(),
                }
            );
            let message = match (update.message, update.edited_message) {
                (Some(msg), None) => msg,
                (None, Some(edited_msg)) => edited_msg,
                _ => {
                    println!("No message");
                    continue;
                }
            };
            let cmd = decide_command(message);
            match cmd {
                Some(command) => {
                    tx.send(command).await.unwrap();
                }
                None => {
                    println!("No command");
                }
            };
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Command>(100);
    let registry = Registry::new();
    const TOKEN_KEY: &str = "TOKEN";
    let bot_config = Arc::new(match env::var(TOKEN_KEY) {
        Ok(val) => bot::Bot {
            token: val.to_owned(),
            offset: 0.into(),
            polling_timeout: 30,
            base_url: String::from("https://api.telegram.org"),
            handlers: registry.clone(),
        },
        Err(_) => {
            println!("Warning: {} is not set. Using default token.", TOKEN_KEY);
            return;
        }
    });
    let api_client = Arc::new(ApiClient::new(
        Arc::new(ReqwestHttpClient::new()),
        bot_config.clone(),
    ));
    let command_handler = Arc::new(EchoCommandHandler::new(api_client.clone()));
    registry
        .register(crate::core::BotCommand::Echo, command_handler)
        .await;

    tokio::spawn(updates_loop(api_client.clone(), tx.clone()));
    tokio::spawn(async move {
        while let Some(command) = rx.recv().await {
            bot_config
                .clone()
                .handlers
                .dispatch(bot_config.clone(), command)
                .await;
        }
    });
    // ждём, пока не нажмут Ctrl+C
    tokio::signal::ctrl_c().await.unwrap();
}
