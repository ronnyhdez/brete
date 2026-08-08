use chrono::Local;

use crate::model::{CurrentSession, Session};
use crate::storage;

fn format_duration(minutes: i64) -> String {
    let hours = minutes / 60;
    let mins = minutes % 60;
    if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

pub fn start(project: String, tag: String) {
    if let Some(current) = storage::read_current() {
        eprintln!(
            "Timer already running for {}/{} since {} — run `brete stop` first.",
            current.project,
            current.tag,
            current.start.format("%H:%M")
        );
        std::process::exit(1);
    }

    let session = CurrentSession {
        start: Local::now(),
        project,
        tag,
    };
    storage::write_current(&session);
    println!(
        "Started {}/{} at {}",
        session.project,
        session.tag,
        session.start.format("%H:%M")
    );
}

pub fn stop() {
    let current = match storage::read_current() {
        Some(c) => c,
        None => {
            eprintln!("No timer running.");
            std::process::exit(1);
        }
    };

    let end = Local::now();
    let duration_minutes = (end - current.start).num_minutes();

    let session = Session {
        start: current.start,
        end,
        duration_minutes,
        project: current.project,
        tag: current.tag,
    };
    storage::append_session(&session);
    storage::clear_current();

    println!(
        "Stopped {}/{} — {}",
        session.project,
        session.tag,
        format_duration(session.duration_minutes)
    );
}

pub fn status() {
    match storage::read_current() {
        Some(current) => {
            let elapsed = (Local::now() - current.start).num_minutes();
            println!(
                "Running: {}/{} since {} ({} elapsed)",
                current.project,
                current.tag,
                current.start.format("%H:%M"),
                format_duration(elapsed)
            );
        }
        None => println!("No timer running."),
    }
}

pub fn log() {
    let mut sessions = storage::read_all_sessions();
    if sessions.is_empty() {
        println!("No sessions recorded yet.");
        return;
    }

    sessions.sort_by(|a, b| b.start.cmp(&a.start));

    println!(
        "{:<12} {:<8} {:<8} {:<20} {:<20}",
        "PROJECT", "TAG", "DURATION", "START", "END"
    );
    for s in sessions {
        println!(
            "{:<12} {:<8} {:<8} {:<20} {:<20}",
            s.project,
            s.tag,
            format_duration(s.duration_minutes),
            s.start.format("%Y-%m-%d %H:%M"),
            s.end.format("%Y-%m-%d %H:%M"),
        );
    }
}
