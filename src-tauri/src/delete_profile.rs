use std::{env, fs, path::Path};

struct Browsers {
    name: String,
    path: String,
    root: String,
}

#[tauri::command]
pub fn delete_profile(browser_name: String, _profile_name: String) {
    let local_appdata = env::var("LOCALAPPDATA").unwrap();
    let roaming_appdata = env::var("APPDATA").unwrap();
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

        let local_state_path = Path::new(&browser_path).join("Local State");

        let file = fs::File::open(local_state_path).expect("file error");
        let json: serde_json::Value = serde_json::from_reader(file).expect("json error");

        // let profiles = json["profile"]["profiles_order"].as_array().unwrap();
        let profiles = json["profile"]["info_cache"].as_array();

        println!("{:?}", profiles)

    };

}