use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::models::LogEntry;

// STREAMING LOG READER
// Reads logs line-by-line instead of loading entire file
pub fn stream_logs(file_path: &str) -> impl Iterator<Item = String> {

    let file = File::open(file_path)
        .expect("Failed to open log file");

    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(Result::ok)
}


// STRUCTURED LOG PARSER
// Converts a raw log line into a structured LogEntry object
pub fn parse_structured_log(line: &str) -> Option<LogEntry> {

    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 5 {
        return None;
    }

    Some(LogEntry {
        timestamp: format!("{} {}", parts[0], parts[1]),
        level: parts[2].to_string(),
        ip: parts[3].to_string(),
        message: parts[4..].join(" "),
    })
}