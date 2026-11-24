use caret_config::Config;

pub fn parse(text: &str) -> (Config, Vec<String>) {
    Config::parse(text)
}

/// Whether a line assigns `key`, commented out or not.
pub fn assigns_key(line: &str, key: &str) -> bool {
    let trimmed = line.trim_start().trim_start_matches('#').trim_start();
    trimmed
        .strip_prefix(key)
        .is_some_and(|rest| rest.trim_start().starts_with('='))
}
