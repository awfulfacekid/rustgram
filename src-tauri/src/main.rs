#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bot;
mod commands;
mod models;

use bot::TelegramBot;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    static ref GLOBAL_BOT: Arc<Mutex<Option<TelegramBot>>> = Arc::new(Mutex::new(None));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::start_bot,
            commands::stop_bot
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
