#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

/// Honours `CARET_CONFIG_DIR`, then `XDG_CONFIG_HOME`, then `~/.config`.
pub fn config_dir() -> Option<PathBuf> {
    if let Some(dir) = non_empty("CARET_CONFIG_DIR") {
        return Some(PathBuf::from(dir));
    }
    if let Some(dir) = non_empty("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(dir).join("caret"));
    }
    non_empty("HOME").map(|home| PathBuf::from(home).join(".config").join("caret"))
}

pub fn config_path() -> Option<PathBuf> {
    config_dir().map(|d| d.join("config.toml"))
}

pub fn themes_dir() -> Option<PathBuf> {
    config_dir().map(|d| d.join("themes"))
}

fn non_empty(var: &str) -> Option<String> {
    std::env::var(var).ok().filter(|v| !v.is_empty())
}

/// Where the configuration lived before the project was called caret.
fn former_dir() -> Option<PathBuf> {
    if non_empty("CARET_CONFIG_DIR").is_some() {
        return None;
    }
    if let Some(dir) = non_empty("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(dir).join("main-terminal"));
    }
    non_empty("HOME").map(|home| PathBuf::from(home).join(".config").join("main-terminal"))
}

/// Copies a configuration written under the old name across to the new one.
///
/// Only when there is nothing at the new path yet, and the old directory is
/// left exactly where it is: a rename should not be able to lose someone's
/// settings, and it should not delete them either.
pub fn adopt_former_config() {
    let (Some(now), Some(before)) = (config_dir(), former_dir()) else {
        return;
    };
    if now == before || now.exists() || !before.is_dir() {
        return;
    }
    let _ = copy_tree(&before, &now);
}

fn copy_tree(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        if is_leftover(&name.to_string_lossy()) {
            continue;
        }
        let target = to.join(&name);
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

/// Whether a name is an editor's working file rather than a setting.
///
/// A swap file names the path it was made for, so carrying one to a new
/// directory produces a warning about a file that is not there, every time
/// the real one is opened. They belong to whoever is editing, not to us.
fn is_leftover(name: &str) -> bool {
    const ENDINGS: [&str; 5] = [".swp", ".swo", ".swn", ".bak", "~"];
    name == ".DS_Store"
        || name.starts_with(".#")
        || ENDINGS.iter().any(|ending| name.ends_with(ending))
}
