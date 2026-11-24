#[allow(unused_imports)]
use crate::prelude::*;

use std::path::Path;

impl Config {
    /// Rewrites the top-level `theme` setting in config text.
    ///
    /// A line edit rather than a re-serialise: the file is the user's, and
    /// round-tripping it through a TOML writer would throw away their
    /// comments and ordering.
    pub fn with_theme(text: &str, theme: &str) -> String {
        let line = format!("theme = {theme:?}");
        let mut out = Vec::new();
        let mut replaced = false;
        let mut depth = 0usize;

        for raw in text.lines() {
            let trimmed = raw.trim_start();
            // Only the top level counts; `theme` inside a table is something
            // else entirely.
            if trimmed.starts_with('[') {
                depth += 1;
            }
            if depth == 0 && !replaced && is_theme_assignment(trimmed) {
                out.push(line.clone());
                replaced = true;
                continue;
            }
            out.push(raw.to_string());
        }

        if !replaced {
            out.insert(0, line);
        }

        let mut joined = out.join("\n");
        if text.ends_with('\n') || text.is_empty() {
            joined.push('\n');
        }
        joined
    }

    /// Rewrites a binding in the `[keys.direct]` table.
    ///
    /// A line edit, for the same reason the theme is: the file is the
    /// user's, and a TOML writer would hand it back without their comments
    /// or their ordering. The table is created if it is not there.
    pub fn with_binding(text: &str, keys: &str, action: &str) -> String {
        const TABLE: &str = "[keys.direct]";
        let line = format!("{keys:?} = {action:?}");

        let mut out: Vec<String> = Vec::new();
        let mut inside = false;
        let mut written = false;
        // Where the table ended, for a binding that is not in it yet.
        let mut append_at: Option<usize> = None;

        for raw in text.lines() {
            let trimmed = raw.trim();
            if trimmed.starts_with('[') {
                if inside && !written {
                    append_at = Some(out.len());
                }
                inside = trimmed == TABLE;
            } else if inside && !written && binds(trimmed, keys) {
                out.push(line.clone());
                written = true;
                continue;
            }
            out.push(raw.to_string());
        }

        if !written {
            if inside {
                append_at = Some(out.len());
            }
            match append_at {
                Some(at) => out.insert(at, line),
                None => {
                    out.push(String::new());
                    out.push(TABLE.to_string());
                    out.push(line);
                }
            }
        }

        let mut joined = out.join("\n");
        if text.ends_with('\n') || text.is_empty() {
            joined.push('\n');
        }
        joined
    }

    /// Saves a binding, creating the file if it is not there yet.
    pub fn save_binding(keys: &str, action: &str) -> std::io::Result<std::path::PathBuf> {
        let path = crate::impls::paths::config_path().ok_or_else(|| {
            std::io::Error::other("could not work out where the config should live")
        })?;
        let existing = std::fs::read_to_string(&path).unwrap_or_default();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, Config::with_binding(&existing, keys, action))?;
        Ok(path)
    }

    /// Saves the theme choice, creating the file if it is not there yet.
    pub fn save_theme(theme: &str) -> std::io::Result<std::path::PathBuf> {
        let path = crate::impls::paths::config_path().ok_or_else(|| {
            std::io::Error::other("could not work out where the config should live")
        })?;
        Config::save_theme_to(&path, theme)?;
        Ok(path)
    }

    pub fn save_theme_to(path: &Path, theme: &str) -> std::io::Result<()> {
        let existing = std::fs::read_to_string(path).unwrap_or_default();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, Config::with_theme(&existing, theme))
    }
}

fn is_theme_assignment(line: &str) -> bool {
    let Some(rest) = line.strip_prefix("theme") else {
        return false;
    };
    rest.trim_start().starts_with('=')
}

/// Whether a line assigns the binding for `keys`.
fn binds(line: &str, keys: &str) -> bool {
    let Some(rest) = line.strip_prefix(&format!("{keys:?}")) else {
        return false;
    };
    rest.trim_start().starts_with('=')
}

impl Config {
    /// Makes sure `key` is written under `table`, and says which line it is
    /// on, counting from one.
    ///
    /// A config file is usually a stub: the theme picker writes one line and
    /// nothing else. Opening it to change the window size and finding only
    /// `theme` there is no help at all, so the setting being looked at is
    /// put in with the value it already has, ready to be changed.
    pub fn with_setting(text: &str, table: &str, key: &str, value: &str) -> (String, usize) {
        let header = format!("[{table}]");
        let assignment = format!("{key} = {value}");

        let mut out: Vec<String> = Vec::new();
        // "" is the top level, which starts open and closes at the first table.
        let mut inside = table.is_empty();
        let mut found: Option<usize> = None;
        let mut table_ends: Option<usize> = None;

        for raw in text.lines() {
            let trimmed = raw.trim();
            if trimmed.starts_with('[') {
                if inside && found.is_none() {
                    table_ends = Some(out.len());
                }
                inside = !table.is_empty() && trimmed == header;
            } else if inside && found.is_none() && assigns(trimmed, key) {
                found = Some(out.len());
            }
            out.push(raw.to_string());
        }

        let at = match found {
            Some(at) => at,
            None => {
                if inside {
                    table_ends = Some(out.len());
                }
                match table_ends {
                    Some(end) => {
                        out.insert(end, assignment);
                        end
                    }
                    None => {
                        if !table.is_empty() {
                            out.push(String::new());
                            out.push(header);
                        }
                        out.push(assignment);
                        out.len() - 1
                    }
                }
            }
        };

        let mut joined = out.join("\n");
        if text.ends_with('\n') || text.is_empty() {
            joined.push('\n');
        }
        (joined, at + 1)
    }
}

/// Whether a line assigns `key`.
fn assigns(line: &str, key: &str) -> bool {
    let Some(rest) = line.strip_prefix(key) else {
        return false;
    };
    rest.trim_start().starts_with('=')
}

impl Config {
    /// Fills a stub config out with the commented example, keeping whatever
    /// was already set.
    ///
    /// A file the theme picker wrote is one line long. Opening it to change
    /// something and finding a single `theme =` is no help: the settings are
    /// all optional, so an absent one is not missing, but there is nothing
    /// there to read or edit. A file that already has sections in it is the
    /// user's own and is left exactly as it is.
    pub fn filled_out(text: &str) -> String {
        if text.lines().any(|line| line.trim_start().starts_with('[')) {
            return text.to_string();
        }

        let mut out = crate::EXAMPLE.to_string();
        // `theme` is the only thing a stub ever holds, because the theme
        // picker is the only thing that writes one.
        for line in text.lines() {
            let trimmed = line.trim();
            let Some(rest) = trimmed.strip_prefix("theme") else {
                continue;
            };
            let Some(value) = rest.trim_start().strip_prefix('=') else {
                continue;
            };
            out = Config::with_theme(&out, value.trim().trim_matches('"'));
        }
        out
    }
}
