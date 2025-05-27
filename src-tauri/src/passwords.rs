use std::path::Path;
use std::{env, fs};

use chrono::{DateTime, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Passwords {
    url: String,
    username: String,
    date_created: DateTime<Utc>,
    date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct ProfileData {
    profile_name: String,
    passwords: Vec<Passwords>,
}

#[derive(Serialize, Deserialize)]
struct BrowserData {
    browser: String,
    profiles: Vec<ProfileData>,
}

struct Browsers {
    name: String,
    path: String, 
    root: String
}

fn webkit_to_unix_time(webkit_time: i64) -> i64 {
    let seconds_between19701601 = 11644473600;

    return webkit_time / 1000000 - seconds_between19701601;
}

#[tauri::command]
pub fn passwords() -> String {
    let local_appdata = env::var("LOCALAPPDATA").unwrap();
    let roaming_appdata = env::var("APPDATA").unwrap();
    let browsers: Vec<Browsers> = vec![
        Browsers {
            name: "Brave Browser".to_string(),
            path: "BraveSoftware\\Brave-Browser\\User Data".to_string(),
            root: local_appdata.to_owned()
        },
        Browsers {
            name: "Chromium".to_string(),
            path: "Chromium\\User Data".to_string(),
            root: local_appdata.to_owned()
        },
        Browsers {
            name: "Google Chrome".to_string(),
            path: "Google\\Chrome\\User Data".to_string(),
            root: local_appdata.to_owned()
        },
        Browsers {
            name: "Microsoft Edge".to_string(),
            path: "Microsoft\\Edge\\User Data".to_string(),
            root: local_appdata.to_owned()
        },
        Browsers {
            name: "Opera".to_string(),
            path: "\\Opera Software\\Opera Stable".to_string(),
            root: roaming_appdata.to_owned()
        },
        Browsers {
            name: "Opera GX".to_string(),
            path: "\\Opera Software\\Opera GX Stable".to_string(),
            root: roaming_appdata.to_owned()
        },
        Browsers {
            name: "Vivaldi".to_string(),
            path: "Vivaldi\\User Data".to_string(),
            root: local_appdata.to_owned()
        },
    ];
    
    let mut browser_data: Vec<BrowserData> = Vec::new();
    for browser in browsers {
        let browser_path = Path::new(&browser.root).join(browser.path);

        if !Path::exists(&browser_path) {
            continue;
        }

        let local_state_path = Path::new(&browser_path).join("Local State");

        let file = fs::File::open(local_state_path).expect("file error");
        let json: serde_json::Value = serde_json::from_reader(file).expect("json error");

        let profiles = json["profile"]["profiles_order"].as_array().unwrap();

        let mut profile_data: Vec<ProfileData> = Vec::new();

        for profile in profiles {
            let profile_name = &json["profile"]["info_cache"][profile.as_str().unwrap()]["name"];

            let login_data = Path::new(&browser_path)
                .join(profile.as_str().unwrap())
                .join("Login Data");

            let conn = Connection::open(login_data).expect("db error");

            let mut logins = conn.prepare("SELECT signon_realm, username_value, date_created, date_password_modified FROM 'logins'").unwrap(); //err app crash browser open

            let mut rows = logins.query([]).unwrap();

            let mut passwords_data: Vec<Passwords> = Vec::new();

            while let Some(row) = rows.next().unwrap() {
                let url: String = row.get(0).unwrap();
                let username: String = row.get(1).unwrap();
                let date_created: i64 = row.get(2).unwrap();
                let date_password_modified: i64 = row.get(3).unwrap();

                let date_created_timestamp =
                    DateTime::from_timestamp(webkit_to_unix_time(date_created), 0).unwrap();
                let date_password_modified_timestamp =
                    DateTime::from_timestamp(webkit_to_unix_time(date_password_modified), 0)
                        .unwrap();

                let data = Passwords {
                    url: url,
                    username: username,
                    date_created: date_created_timestamp,
                    date_modified: date_password_modified_timestamp,
                };

                passwords_data.push(data);
            }

            let profile = ProfileData {
                profile_name: profile_name.to_string(),
                passwords: passwords_data
            };

            profile_data.push(profile);
        }

        let browser = BrowserData {
            browser: browser.name,
            profiles: profile_data
        };

        browser_data.push(browser);

    }

    let json_data = serde_json::to_string(&browser_data).expect("JSON error");

    // println!("{:?}", json_data)

    json_data.into()

}