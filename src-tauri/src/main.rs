// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{UNIX_EPOCH, Duration};
use std::{env, fs};
use std::path::{Path};

use chrono::{Date, DateTime, NaiveDateTime, TimeZone, Utc};
use humantime::{format_duration, format_rfc3339_millis};
use rusqlite::{Connection};

fn webkit_to_unix_time(webkit_time: i64) -> i64{
    let seconds_between19701601 = 11644473600; 

    return webkit_time / 1000000 - seconds_between19701601;
}

fn greet() {
    let user_profile = env::var("LOCALAPPDATA").unwrap();
    let chrome_path = Path::new(&user_profile).join("Google/Chrome/User Data");
    let local_state_path = Path::new(&user_profile).join("Google/Chrome/User Data/Local State");
    // let login_data_path = PathBuf::from(&user_profile).join("Google/Chrome/User Data/Default/Login Data");

    let file = fs::File::open(local_state_path).expect("file error");
    let json: serde_json::Value = serde_json::from_reader(file).expect("json error");

    let profiles = json["profile"]["profiles_order"].as_array().unwrap();


    for profile in profiles {
        // println!("{:?}", profile.as_str().unwrap());
        let login_data = Path::new(&chrome_path).join(profile.as_str().unwrap()).join("Login Data");
        // println!("{:?}", login_data);
        
        let conn = Connection::open(login_data).expect("db error");

        let mut logins = conn.prepare("SELECT signon_realm, username_value, date_created, date_password_modified FROM 'logins'").unwrap();
        let mut rows = logins.query([]).unwrap();

        while let Some(row) =  rows.next().unwrap() {
            let url: String = row.get(0).unwrap();
            let username: String = row.get(1).unwrap();
            let date_created: i64 = row.get(2).unwrap();
            let date_password_modified: i64 = row.get(3).unwrap();

            let ts = webkit_to_unix_time(date_created);

            let dt = DateTime::from_timestamp(ts,0);

            let date = dt.unwrap();

            println!("url : {}, name: {}, date: {}", url, username, date);
        }

    }
    // let master_key = chrome_password::get_master_key(&local_state_path);
    // let password = chrome_password::get_password(&login_data_path, &master_key);

    // println!("{:?}", &password);
    //println!("{:?}", profiles);
}
fn main() {
    greet();
    // totlaunloc_lib::run()
}