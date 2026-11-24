#[allow(unused_imports)]
use crate::prelude::*;

use std::io::Write;
use std::path::Path;

/// Reads a command history file, oldest first.
///
/// A missing or unreadable file is an empty history rather than an error:
/// suggestions are a convenience and must never stop the terminal starting.
pub fn load(path: &Path, limit: usize) -> Vec<String> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    // The file is appended to, so a command run repeatedly appears many
    // times. Only the most recent occurrence matters for suggestions.
    let mut entries: Vec<String> = Vec::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        entries.retain(|entry| entry != line);
        entries.push(line.to_string());
    }

    if entries.len() > limit {
        entries.drain(..entries.len() - limit);
    }
    entries
}

/// Appends one command, creating the file if needed.
pub fn append(path: &Path, command: &str) -> std::io::Result<()> {
    let command = command.trim();
    if command.is_empty() || command.contains('\n') {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(file, "{command}")
}
