use reqwest::Client;
use serde_json::json;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, Mutex};
use crate::models::{Button, ReplyKeyboard, ApiResponse, Message};

pub struct TelegramBot {
    token: String,
    client: Client,
    last_update_id: Arc<Mutex<i64>>,
    processed_users: Arc<Mutex<HashSet<i64>>>,
    shutdown_tx: Option<broadcast::Sender<()>>,
}

impl TelegramBot {
    pub fn new(token: String) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        TelegramBot {
            token,
            client: Client::new(),
            last_update_id: Arc::new(Mutex::new(0)),
            processed_users: Arc::new(Mutex::new(HashSet::new())),
            shutdown_tx: Some(shutdown_tx),
        }
    }

    pub fn shutdown(&self) {
        if let Some(ref tx) = self.shutdown_tx {
            let _ = tx.send(());
            println!("Shutdown signal sent.");
        }
    }

    pub async fn run(&self, greeting: String, buttons: Vec<Button>) {
        let url = format!("https://api.telegram.org/bot{}/getUpdates", self.token);
        let last_update_id = self.last_update_id.clone();
        let mut shutdown_rx = self.shutdown_tx.as_ref().unwrap().subscribe();
    
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    println!("Shutting down bot...");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    let current_last_update_id = {
                        let guard = last_update_id.lock().await;
                        *guard
                    };
    
                    match self.client.get(&url)
                        .query(&[("offset", (current_last_update_id + 1).to_string())])
                        .send()
                        .await
                    {
                        Ok(response) if response.status().is_success() => {
                            match response.text().await {
                                Ok(response_text) => {
                                    let updates: ApiResponse = serde_json::from_str(&response_text)
                                        .unwrap_or_else(|e| {
                                            eprintln!("Failed to parse API response: {}", e);
                                            ApiResponse { result: vec![] }
                                        });
    
                                    for update in updates.result {
                                        let mut last_update_id = last_update_id.lock().await;
    
                                        if update.update_id > *last_update_id {
                                            *last_update_id = update.update_id;
    
                                            if let Some(message) = update.message {
                                                if let Some(text) = message.text {
                                                    if text == "/start" {
                                                        let keyboard = create_default_keyboard(buttons.clone());
                                                        if let Err(e) = self.send_message_with_keyboard(&message.chat.id, &greeting, keyboard).await {
                                                            eprintln!("Failed to send message: {}", e);
                                                        }
                                                    } else {
                                                        for button in buttons.iter() {
                                                            if text == button.text {
                                                                if let Err(e) = self.send_message_with_keyboard(&message.chat.id, &button.handler, create_default_keyboard(buttons.clone())).await {
                                                                    eprintln!("Failed to send message: {}", e);
                                                                }
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => eprintln!("Failed to read response text: {}", e),
                            }
                        }
                        Ok(response) => eprintln!("API Error: {}", response.status()),
                        Err(e) => eprintln!("Request failed: {}", e),
                    }
                }
            }
        }
        println!("Bot has stopped.");
    }

    async fn send_message_with_keyboard(&self, chat_id: &i64, text: &str, keyboard: ReplyKeyboard) -> Result<(), reqwest::Error> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        let body = json!({
            "chat_id": chat_id,
            "text": text,
            "reply_markup": keyboard,
        });

        self.client.post(&url)
            .json(&body)
            .send()
            .await?;

        Ok(())
    }
}

fn create_default_keyboard(buttons: Vec<Button>) -> ReplyKeyboard {
    ReplyKeyboard {
        keyboard: buttons.into_iter().map(|b| vec![b.text]).collect(),
        resize_keyboard: true,
        one_time_keyboard: true,
        selective: false,
    }
}
