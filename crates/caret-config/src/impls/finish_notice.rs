#[allow(unused_imports)]
use crate::prelude::*;

impl FinishNotice {
    pub const OFF: &'static str = "off";
    pub const STATUS: &'static str = "status";
    pub const SYSTEM: &'static str = "system";

    /// Canonical name first, then the accepted aliases.
    const NAMES: [(FinishNotice, &'static [&'static str]); 3] = [
        (FinishNotice::Off, &[Self::OFF, "none"]),
        (FinishNotice::Status, &[Self::STATUS, "line"]),
        (FinishNotice::System, &[Self::SYSTEM, "notification"]),
    ];

    pub fn parse(name: &str) -> Option<FinishNotice> {
        let lower = name.trim().to_ascii_lowercase();
        Self::NAMES
            .iter()
            .find(|(_, names)| names.contains(&lower.as_str()))
            .map(|(value, _)| *value)
    }

    pub const fn name(self) -> &'static str {
        match self {
            FinishNotice::Off => Self::OFF,
            FinishNotice::Status => Self::STATUS,
            FinishNotice::System => Self::SYSTEM,
        }
    }

    pub fn accepted() -> Vec<&'static str> {
        Self::NAMES.iter().map(|(_, names)| names[0]).collect()
    }
}
