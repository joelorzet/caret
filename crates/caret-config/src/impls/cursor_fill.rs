#[allow(unused_imports)]
use crate::prelude::*;

impl CursorFill {
    const NAMES: [(CursorFill, &'static [&'static str]); 3] = [
        (CursorFill::Blink, &["blink", "blinking"]),
        (CursorFill::Solid, &["solid", "steady"]),
        (CursorFill::Translucent, &["translucent", "ghost"]),
    ];

    /// How much of the cursor colour to lay over the cell.
    pub const TRANSLUCENT_ALPHA: u8 = 115;

    pub fn parse(name: &str) -> Option<CursorFill> {
        let lower = name.trim().to_ascii_lowercase();
        Self::NAMES
            .iter()
            .find(|(_, names)| names.contains(&lower.as_str()))
            .map(|(value, _)| *value)
    }

    pub const fn name(self) -> &'static str {
        match self {
            CursorFill::Blink => "blink",
            CursorFill::Solid => "solid",
            CursorFill::Translucent => "translucent",
        }
    }

    pub fn accepted() -> Vec<&'static str> {
        Self::NAMES.iter().map(|(_, names)| names[0]).collect()
    }

    pub const fn blinks(self) -> bool {
        matches!(self, CursorFill::Blink)
    }

    pub const fn alpha(self) -> u8 {
        match self {
            CursorFill::Translucent => Self::TRANSLUCENT_ALPHA,
            _ => 255,
        }
    }
}
