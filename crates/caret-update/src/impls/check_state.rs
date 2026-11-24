#[allow(unused_imports)]
use crate::prelude::*;

/// Seconds in an hour, for turning the configured interval into a deadline.
const HOUR: u64 = 3_600;

impl CheckState {
    /// Whether a check is due.
    ///
    /// A clock that has gone backwards, which happens, counts as due rather
    /// than as never again.
    pub fn due(&self, now: u64, every_hours: u32) -> bool {
        let next = self.checked_at.saturating_add(every_hours as u64 * HOUR);
        now >= next || now < self.checked_at
    }

    /// The file this is kept in, which is ours to write and nobody's to edit.
    pub fn parse(text: &str) -> CheckState {
        let mut state = CheckState::default();
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            match key.trim() {
                "checked_at" => state.checked_at = value.trim().parse().unwrap_or(0),
                "seen" => state.seen = value.trim().to_string(),
                _ => {}
            }
        }
        state
    }

    pub fn render(&self) -> String {
        format!("checked_at = {}\nseen = {}\n", self.checked_at, self.seen)
    }
}
