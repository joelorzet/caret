#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

/// Where the last check is remembered, beside the configuration.
pub fn state_path(config_dir: &std::path::Path) -> PathBuf {
    config_dir.join("update-check")
}

impl CheckState {
    pub fn read(config_dir: &std::path::Path) -> CheckState {
        std::fs::read_to_string(state_path(config_dir))
            .map(|text| CheckState::parse(&text))
            .unwrap_or_default()
    }

    /// Writes the state, and says nothing if it cannot: failing to remember
    /// a check is not worth a word to somebody opening a terminal.
    pub fn write(&self, config_dir: &std::path::Path) {
        let _ = std::fs::create_dir_all(config_dir);
        let _ = std::fs::write(state_path(config_dir), self.render());
    }
}

/// Now, in seconds since the epoch.
pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
