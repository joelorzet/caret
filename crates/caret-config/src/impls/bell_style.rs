#[allow(unused_imports)]
use crate::prelude::*;

impl BellStyle {
    pub const NONE: &'static str = "none";
    pub const VISUAL: &'static str = "visual";
    pub const AUDIBLE: &'static str = "audible";

    /// Canonical name first, then the accepted aliases.
    const NAMES: [(BellStyle, &'static [&'static str]); 3] = [
        (BellStyle::None, &[Self::NONE, "off"]),
        (BellStyle::Visual, &[Self::VISUAL, "flash"]),
        (BellStyle::Audible, &[Self::AUDIBLE, "sound"]),
    ];

    pub fn parse(name: &str) -> Option<BellStyle> {
        let lower = name.trim().to_ascii_lowercase();
        Self::NAMES
            .iter()
            .find(|(_, names)| names.contains(&lower.as_str()))
            .map(|(value, _)| *value)
    }

    pub const fn name(self) -> &'static str {
        match self {
            BellStyle::None => Self::NONE,
            BellStyle::Visual => Self::VISUAL,
            BellStyle::Audible => Self::AUDIBLE,
        }
    }

    /// Canonical names, for error messages and the example config.
    pub fn accepted() -> Vec<&'static str> {
        Self::NAMES.iter().map(|(_, names)| names[0]).collect()
    }
}
