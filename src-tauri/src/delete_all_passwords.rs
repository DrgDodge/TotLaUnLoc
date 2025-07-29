

use crate::delete_profile::delete_profile;
use std::{env, fs, path::Path};

struct Browsers {
    name: String,
    path: String,
    root: String,
}

#[tauri::command]
pub fn delete_all_passwords() -> Result<(), String> {
    println!("Starting password deletion for all profiles.");
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

    for browser in browsers {
        let browser_path = Path::new(&browser.root).join(&browser.path);
        if !Path::exists(&browser_path) {
            continue;
        }

        let local_state_path = Path::new(&browser_path).join("Local State");
        if !Path::exists(&local_state_path) {
            continue;
        }

        let local_state_file_result = fs::File::open(&local_state_path);
        let local_state_file = match local_state_file_result {
            Ok(file) => file,
            Err(_) => continue,
        };

        let local_state_json: serde_json::Value = match serde_json::from_reader(local_state_file) {
            Ok(json) => json,
            Err(_) => continue,
        };

        if let Some(info_cache) = local_state_json["profile"]["info_cache"].as_object() {
            for (_dir_name, profile_info) in info_cache {
                if let Some(profile_name) = profile_info["name"].as_str() {
                    match delete_profile(browser.name.clone(), profile_name.to_string()) {
                        Ok(_) => println!("Successfully deleted passwords for profile '{}' in {}", profile_name, browser.name),
                        Err(e) => eprintln!("Failed to delete passwords for profile '{}' in {}: {}", profile_name, browser.name, e),
                    }
                }
            }
        }
    }
    println!("Finished password deletion for all profiles.");
    Ok(())
}

