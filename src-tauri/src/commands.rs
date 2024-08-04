use crate::bot::TelegramBot;
use crate::models::Button;
use crate::GLOBAL_BOT;
use tauri::command;
use std::sync::Arc;
use tokio::sync::Mutex;

#[command]
pub async fn start_bot(token: String, greeting: String, buttons: Vec<Button>) -> Result<(), String> {
    let bot = TelegramBot::new(token);
    *GLOBAL_BOT.lock().await = Some(bot);
    tokio::spawn(async move {
        if let Some(bot) = GLOBAL_BOT.lock().await.as_ref() {
            bot.run(greeting, buttons).await;
        }
    });
    Ok(())
}

#[command]
pub async fn stop_bot() -> Result<(), String> {
    if let Some(bot) = GLOBAL_BOT.lock().await.take() {
        bot.shutdown();
    } else {
        println!("No bot instance found to stop.");
    }
    Ok(())
}
