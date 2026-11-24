#[allow(unused_imports)]
use crate::prelude::*;

impl CursorShape {
    pub const BLOCK: &'static str = "block";
    pub const UNDERLINE: &'static str = "underline";
    pub const BAR: &'static str = "bar";
    pub const HOLLOW: &'static str = "hollow";

    const NAMES: [(CursorShape, &'static [&'static str]); 4] = [
        (CursorShape::Block, &[Self::BLOCK]),
        (CursorShape::Underline, &[Self::UNDERLINE, "under"]),
        (CursorShape::Bar, &[Self::BAR, "beam"]),
        (CursorShape::Hollow, &[Self::HOLLOW, "outline"]),
    ];

    pub fn parse(name: &str) -> Option<CursorShape> {
        let lower = name.trim().to_ascii_lowercase();
        Self::NAMES
            .iter()
            .find(|(_, names)| names.contains(&lower.as_str()))
            .map(|(value, _)| *value)
    }

    pub const fn name(self) -> &'static str {
        match self {
            CursorShape::Block => Self::BLOCK,
            CursorShape::Underline => Self::UNDERLINE,
            CursorShape::Bar => Self::BAR,
            CursorShape::Hollow => Self::HOLLOW,
        }
    }

    pub fn accepted() -> Vec<&'static str> {
        Self::NAMES.iter().map(|(_, names)| names[0]).collect()
    }
}
