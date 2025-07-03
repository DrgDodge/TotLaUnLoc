use base32;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;
use totp_rs::{Algorithm, TOTP};
use url::Url;

#[derive(Serialize)]
pub struct Entry {
    id: u32,
    icon: String,
    account: String,
    username: String,
    code: String,
}

pub struct Account {
    id: u32,
    icon: String,
    account: String,  // issuer
    username: String, // account_name
    totp: TOTP,
}

pub struct AppState {
    pub accounts: Mutex<Vec<Account>>,
    pub next_id: Mutex<u32>,
}

fn parse_otpauth(url_str: &str) -> Result<(String, String, TOTP), Box<dyn std::error::Error>> {
    let url = Url::parse(url_str)?;
    let path = url.path();
    let label = path.trim_start_matches('/').to_string();
    let query: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
    let issuer = query.get("issuer").cloned().unwrap_or_default();
    let secret_base32 = query.get("secret").ok_or("Missing secret")?;
    let secret = base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret_base32)
        .ok_or("Invalid base32 secret")?;
    let algorithm = query
        .get("algorithm")
        .map(|alg| match alg.as_str() {
            "SHA1" => Algorithm::SHA1,
            "SHA256" => Algorithm::SHA256,
            "SHA512" => Algorithm::SHA512,
            _ => Algorithm::SHA1,
        })
        .unwrap_or(Algorithm::SHA1);
    let digits = query
        .get("digits")
        .and_then(|d| d.parse().ok())
        .unwrap_or(6);
    let period = query
        .get("period")
        .and_then(|p| p.parse().ok())
        .unwrap_or(30);
    let totp = TOTP::new(algorithm, digits, 1, period, secret)?;

    let (account, username) = if let Some((acc, user)) = label.split_once(':') {
        (acc.to_string(), user.to_string())
    } else {
        (issuer.clone(), label)
    };

    if !issuer.is_empty() && account != issuer {
        return Err("Issuer mismatch between label and parameter".into());
    }

    Ok((account, username, totp))
}

#[tauri::command]
pub fn add_account(otpauth: String, state: State<AppState>) -> Result<(), String> {
    let (account, username, totp) = parse_otpauth(&otpauth).map_err(|e| e.to_string())?;
    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;
    let icon = "/icons/default.svg".to_string();
    let mut accounts = state.accounts.lock().unwrap();
    accounts.push(Account {
        id,
        icon,
        account,
        username,
        totp,
    });
    Ok(())
}

#[tauri::command]
pub fn delete_account(id: u32, state: State<AppState>) -> Result<(), String> {
    let mut accounts = state.accounts.lock().unwrap();
    let initial_len = accounts.len();
    accounts.retain(|acc| acc.id != id);
    if accounts.len() < initial_len {
        Ok(())
    } else {
        Err("Account not found".to_string())
    }
}

#[tauri::command]
pub fn get_accounts_with_codes(state: State<AppState>) -> Vec<Entry> {
    let accounts = state.accounts.lock().unwrap();
    accounts
        .iter()
        .map(|acc| Entry {
            id: acc.id,
            icon: acc.icon.clone(),
            account: acc.account.clone(),
            username: acc.username.clone(),
            code: acc.totp.generate_current().unwrap_or_default(),
        })
        .collect()
}
