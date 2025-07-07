use std::{env, fs, path::Path, thread, time::Duration};
use rusqlite::Connection;
use tempfile::tempdir;

struct Browsers {
    name: String,
    path: String,
    root: String,
}

#[tauri::command]
pub fn delete_account_entry(browser_name: String, profile_name: String, url: String, username: String) -> Result<(), String> {
    let cleaned_profile_name = profile_name.trim_matches('"').to_string();
    let local_appdata = env::var("LOCALAPPDATA").map_err(|e| format!("Failed to get LOCALAPPDATA: {}", e))?;
    let roaming_appdata = env::var("APPDATA").map_err(|e| format!("Failed to get APPDATA: {}", e))?;

    let browsers: Vec<Browsers> = vec![
        Browsers {
            name: "Brave Browser".to_string(),
            path: "BraveSoftware\\Brave-Browser\\User Data".to_string(),
            root: local_appdata.to_owned(),
        },
        Browsers {
            name: "Chromium".to_string(),
            path: "Chromium\\User Data".to_string(),
            root: local_appdata.to_owned(),
        },
        Browsers {
            name: "Google Chrome".to_string(),
            path: "Google\\Chrome\\User Data".to_string(),
            root: local_appdata.to_owned(),
        },
        Browsers {
            name: "Microsoft Edge".to_string(),
            path: "Microsoft\\Edge\\User Data".to_string(),
            root: local_appdata.to_owned(),
        },
        Browsers {
            name: "Opera".to_string(),
            path: "Opera Software\\Opera Stable".to_string(),
            root: roaming_appdata.to_owned(),
        },
        Browsers {
            name: "Opera GX".to_string(),
            path: "Opera Software\\Opera GX Stable".to_string(),
            root: roaming_appdata.to_owned(),
        },
        Browsers {
            name: "Vivaldi".to_string(),
            path: "Vivaldi\\User Data".to_string(),
            root: local_appdata.to_owned(),
        },
    ];

    if let Some(browser) = browsers.iter().find(|b| b.name == browser_name) {
        let browser_path = Path::new(&browser.root).join(&browser.path);

        if !Path::exists(&browser_path) {
            return Err(format!("Browser path does not exist: {:?}", browser_path));
        }

        let local_state_path = Path::new(&browser_path).join("Local State");
        if !Path::exists(&local_state_path) {
            return Err(format!("Local State file does not exist: {:?}", local_state_path));
        }

        let local_state_file = fs::File::open(&local_state_path).map_err(|e| format!("Failed to open Local State file: {}", e))?;
        let local_state_json: serde_json::Value = serde_json::from_reader(local_state_file).map_err(|e| format!("Failed to parse Local State JSON: {}", e))?;

        let mut actual_profile_dir_name: Option<String> = None;
        if let Some(info_cache) = local_state_json["profile"]["info_cache"].as_object() {
            for (dir_name, profile_info) in info_cache {
                if let Some(name) = profile_info["name"].as_str() {
                    if name == cleaned_profile_name {
                        actual_profile_dir_name = Some(dir_name.clone());
                        break;
                    }
                }
            }
        }

        let actual_profile_dir_name = actual_profile_dir_name.ok_or_else(|| format!("Could not find actual directory name for profile: {}", cleaned_profile_name))?;

        let login_data_path = if browser.name == "Opera GX" {
            Path::new(&browser_path).join("Login Data")
        } else {
            Path::new(&browser_path)
                .join(&actual_profile_dir_name)
                .join("Login Data")
        };

        if !Path::exists(&login_data_path) {
            return Err(format!("Login Data file does not exist: {:?}", login_data_path));
        }

        let tmpdir = tempdir().map_err(|e| format!("Failed to create temporary directory: {}", e))?;
        let tmp_login_data_path = tmpdir.path().join("Login Data");

        fs::copy(&login_data_path, &tmp_login_data_path).map_err(|e| format!("Failed to copy Login Data file: {}", e))?;

        {
            let conn = Connection::open(&tmp_login_data_path).map_err(|e| format!("Failed to open SQLite connection: {}", e))?;

            let mut stmt = conn.prepare("DELETE FROM logins WHERE signon_realm = ?1 AND username_value = ?2").map_err(|e| format!("Failed to prepare SQL statement: {}", e))?;
            let rows_affected = stmt.execute(&[&url, &username]).map_err(|e| format!("Failed to execute SQL statement: {}", e))?;

            if rows_affected == 0 {
                println!("No rows deleted for url: {} username: {} in profile: {}", url, username, profile_name);
            } else {
                println!("Successfully deleted {} row(s) for url: {} username: {} in profile: {}", rows_affected, url, username, profile_name);
            }
        } // conn is dropped here, closing the file handle

        let mut attempts = 0;
        const MAX_ATTEMPTS: u8 = 5;
        const RETRY_DELAY_MS: u64 = 100;

        loop {
            match fs::copy(&tmp_login_data_path, &login_data_path) {
                Ok(_) => {
                    println!("Successfully copied modified Login Data back to: {}", login_data_path.display());
                    break;
                },
                Err(e) => {
                    attempts += 1;
                    if attempts >= MAX_ATTEMPTS {
                        return Err(format!("Failed to copy modified Login Data back after {} attempts: {}: {}", MAX_ATTEMPTS, login_data_path.display(), e));
                    }
                    println!("Retrying copy of modified Login Data (attempt {}/{}): {}", attempts, MAX_ATTEMPTS, login_data_path.display());
                    thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
                }
            }
        }

        Ok(())
    } else {
        Err(format!("Browser not found: {}", browser_name))
    }
}