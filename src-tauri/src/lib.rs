// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg_attr(mobile, tauri::mobile_entry_point)]

mod passwords;
mod totp;

use totp::{add_account, delete_account, get_accounts_with_codes, AppState};
use std::sync::Mutex;

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            accounts: Mutex::new(vec![]),
            next_id: Mutex::new(0),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            passwords::passwords,
            add_account,
            delete_account,
            get_accounts_with_codes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}