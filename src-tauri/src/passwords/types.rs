use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Passwords {
    pub url: String,
    pub username: String,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileData {
    pub profile_name: String,
    pub passwords: Vec<Passwords>,
}

#[derive(Serialize, Deserialize)]
pub struct BrowserData {
    pub browser: String,
    pub profiles: Vec<ProfileData>,
}

pub struct Browsers {
    pub name: String,
    pub path: String,
    pub root: String,
}
