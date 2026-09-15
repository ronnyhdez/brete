use std::fs;
use std::path::PathBuf;

use crate::model::{CurrentSession, Session};

fn brete_dir() -> PathBuf {
    let dir = dirs::home_dir()
        .expect("could not determine home directory")
        .join(".brete");
    if !dir.exists() {
        fs::create_dir_all(&dir).expect("could not create ~/.brete");
    }
    dir
}

fn current_path() -> PathBuf {
    brete_dir().join("current.json")
}

fn log_path() -> PathBuf {
    brete_dir().join("log.csv")
}

pub fn read_current() -> Option<CurrentSession> {
    let path = current_path();
    if !path.exists() {
        return None;
    }
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn write_current(session: &CurrentSession) {
    let data = serde_json::to_string_pretty(session).expect("failed to serialize current session");
    fs::write(current_path(), data).expect("failed to write current session");
}

pub fn clear_current() {
    let path = current_path();
    if path.exists() {
        fs::remove_file(path).expect("failed to clear current session");
    }
}

pub fn append_session(session: &Session) {
    let path = log_path();
    let write_header = !path.exists();

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("failed to open log file");

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    if write_header {
        writer
            .write_record([
                "start",
                "end",
                "duration_minutes",
                "duration_hours",
                "week",
                "project",
                "tag",
            ])
            .expect("failed to write header");
    }

    writer.serialize(session).expect("failed to write session");
    writer.flush().expect("failed to flush log file");
}

pub fn read_all_sessions() -> Vec<Session> {
    let path = log_path();
    if !path.exists() {
        return Vec::new();
    }

    let mut reader = csv::Reader::from_path(path).expect("failed to open log file");
    reader
        .deserialize()
        .filter_map(|record: Result<Session, _>| record.ok())
        .collect()
}
