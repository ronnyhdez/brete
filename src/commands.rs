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
    let duration_hours = (duration_minutes as f64 / 60.0 * 100.0).round() / 100.0;

    let session = Session {
        start: current.start,
        end,
        duration_minutes,
        duration_hours,
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

    let header = ["PROJECT", "TAG", "DURATION", "HOURS", "START", "END"]
        .map(String::from);
    let rows: Vec<[String; 6]> = sessions
        .iter()
        .map(|s| {
            [
                s.project.clone(),
                s.tag.clone(),
                format_duration(s.duration_minutes),
                format!("{:.2}", s.duration_hours),
                s.start.format("%Y-%m-%d %H:%M").to_string(),
                s.end.format("%Y-%m-%d %H:%M").to_string(),
            ]
        })
        .collect();

    let mut widths = header.each_ref().map(|h| h.len());
    for row in &rows {
        for (w, cell) in widths.iter_mut().zip(row.iter()) {
            *w = (*w).max(cell.len());
        }
    }

    print_row(&header, &widths);
    for row in &rows {
        print_row(row, &widths);
    }
}

fn print_row(cells: &[String; 6], widths: &[usize; 6]) {
    let line: Vec<String> = cells
        .iter()
        .zip(widths.iter())
        .enumerate()
        .map(|(i, (cell, width))| {
            if i == cells.len() - 1 {
                cell.clone()
            } else {
                format!("{:<width$}", cell, width = width)
            }
        })
        .collect();
    println!("{}", line.join("  "));
}
