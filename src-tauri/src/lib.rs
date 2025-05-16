use std::{env, fs};
use std::path::{Path};

use chrono::{DateTime, Utc};
use rusqlite::{Connection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Passwords {
    url: String,
    username: String,
    date_created: DateTime<Utc>,
    date_modified: DateTime<Utc>
}

#[tauri::command]
fn greet() -> String {
    fn webkit_to_unix_time(webkit_time: i64) -> i64 {
        let seconds_between19701601 = 11644473600; 

        return webkit_time / 1000000 - seconds_between19701601;
    }

    let user_profile = env::var("LOCALAPPDATA").unwrap();
    let chrome_path = Path::new(&user_profile).join("Google/Chrome/User Data");
    let local_state_path = Path::new(&user_profile).join("Google/Chrome/User Data/Local State");
    // let login_data_path = PathBuf::from(&user_profile).join("Google/Chrome/User Data/Default/Login Data");

    let file = fs::File::open(local_state_path).expect("file error");
    let json: serde_json::Value = serde_json::from_reader(file).expect("json error");

    let profiles = json["profile"]["profiles_order"].as_array().unwrap();

    let mut passwords_data: Vec<Passwords> = Vec::new();

    for profile in profiles {
        let login_data = Path::new(&chrome_path).join(profile.as_str().unwrap()).join("Login Data");
        
        let conn = Connection::open(login_data).expect("db error");

        let mut logins = conn.prepare("SELECT signon_realm, username_value, date_created, date_password_modified FROM 'logins'").unwrap();
        let mut rows = logins.query([]).unwrap();


        while let Some(row) =  rows.next().unwrap() {
            let url: String = row.get(0).unwrap();
            let username: String = row.get(1).unwrap();
            let date_created: i64 = row.get(2).unwrap();
            let date_password_modified: i64 = row.get(3).unwrap();

            let date_created_timestamp = DateTime::from_timestamp(webkit_to_unix_time(date_created), 0).unwrap();
            let date_password_modified_timestamp = DateTime::from_timestamp(webkit_to_unix_time(date_password_modified), 0).unwrap();

            let data = Passwords {
                url: url,
                username: username,
                date_created: date_created_timestamp,
                date_modified: date_password_modified_timestamp
            };

            passwords_data.push(data);
            
        }
    }
    
    let json_data = serde_json::to_string(&passwords_data).expect("json error");

    json_data.into()

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
