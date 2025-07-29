mod delete_all_passwords;
use delete_all_passwords::delete_all_passwords;


mod delete_profile;
use delete_profile::delete_profile;

mod passwords;
use passwords::passwords;

mod totp;
use totp::{add_account, delete_account, get_accounts_with_codes, AppState};

mod delete_account_entry;
use delete_account_entry::delete_account_entry;

use std::sync::Mutex;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            accounts: Mutex::new(vec![]),
            next_id: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            delete_all_passwords,
            delete_profile,
            passwords,
            add_account,
            delete_account,
            get_accounts_with_codes,
            delete_account_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


