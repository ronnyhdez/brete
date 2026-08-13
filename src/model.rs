use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentSession {
    pub start: DateTime<Local>,
    pub project: String,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub duration_minutes: i64,
    pub duration_hours: f64,
    pub project: String,
    pub tag: String,
}
