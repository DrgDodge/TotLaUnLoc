// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod totp;

use totp::{add_account, delete_account, get_accounts_with_codes, AppState};
use std::sync::Mutex;

fn main() {
        totlaunloc_lib::run();
    tauri::Builder::default()
        .manage(AppState {
            accounts: Mutex::new(vec![]),
            next_id: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![add_account, delete_account, get_accounts_with_codes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}