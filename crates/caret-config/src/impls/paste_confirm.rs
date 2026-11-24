#[allow(unused_imports)]
use crate::prelude::*;

impl PasteConfirm {
    pub const ALWAYS: &'static str = "always";
    pub const NOTABLE: &'static str = "notable";
    pub const NEVER: &'static str = "never";

    /// Canonical name first, then the accepted aliases.
    const NAMES: [(PasteConfirm, &'static [&'static str]); 3] = [
        (PasteConfirm::Always, &[Self::ALWAYS, "all"]),
        (
            PasteConfirm::Notable,
            &[Self::NOTABLE, "lines", "multiline"],
        ),
        (PasteConfirm::Never, &[Self::NEVER, "off"]),
    ];

    pub fn parse(name: &str) -> Option<PasteConfirm> {
        let lower = name.trim().to_ascii_lowercase();
        Self::NAMES
            .iter()
            .find(|(_, names)| names.contains(&lower.as_str()))
            .map(|(value, _)| *value)
    }

    pub const fn name(self) -> &'static str {
        match self {
            PasteConfirm::Always => Self::ALWAYS,
            PasteConfirm::Notable => Self::NOTABLE,
            PasteConfirm::Never => Self::NEVER,
        }
    }

    pub fn accepted() -> Vec<&'static str> {
        Self::NAMES.iter().map(|(_, names)| names[0]).collect()
    }
}
