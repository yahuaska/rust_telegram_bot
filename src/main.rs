use crate::api_client::ApiClient;
use crate::commands::{decide_command, Command};
use crate::http_clients::ReqwestHttpClient;
use crate::types::BotConfig;
use crate::types::GetMeResponse;
use tokio::sync::mpsc;

use futures_util::StreamExt;
use std::env;

pub mod api_client;
pub mod commands;
pub mod http_clients;
pub mod types;

fn print_me(resp: GetMeResponse) {
    if resp.ok {
        println!("Status: OK");
        println!(
            "My ID: {}\nMy name is: {} ({})",
            resp.result.id, resp.result.first_name, resp.result.username
        );
        if resp.result.is_bot {
            println!("I am a bot!");
        } else {
            println!("I am not a bot!");
        }
    }
}

async fn updates_loop(bot_config: BotConfig, tx: mpsc::Sender<Command>) {
    let mut api_client = ApiClient::new(
        ReqwestHttpClient::new(),
        |token: &str, method: &str| format!("https://api.telegram.org/bot{token}/{method}"),
        bot_config,
    );
    print_me(api_client.get_me().await.unwrap());
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

async fn worker(mut rx: mpsc::Receiver<Command>) {
    while let Some(command) = rx.recv().await {
        println!(
            "Command: {:#?}, args: {:#?} #{}",
            command.command, command.args, command.message.message_id
        );
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let _: &mpsc::Sender<Command> = &tx;
    const TOKEN_KEY: &str = "TOKEN";
    let bot_config = match env::var(TOKEN_KEY) {
        Ok(val) => BotConfig {
            token: val.to_owned(),
            offset: 0,
            polling_timeout: 30,
        },
        Err(_) => {
            println!("Warning: {} is not set. Using default token.", TOKEN_KEY);
            return;
        }
    };

    tokio::spawn(updates_loop(bot_config.clone(), tx.clone()));
    tokio::spawn(worker(rx));
    // ждём, пока не нажмут Ctrl+C
    tokio::signal::ctrl_c().await.unwrap();
}
