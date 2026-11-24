#[allow(unused_imports)]
use crate::prelude::*;

impl Appearance {
    pub const DARK: &'static str = "dark";
    pub const LIGHT: &'static str = "light";

    const NAMES: [(Appearance, &'static str); 2] = [
        (Appearance::Dark, Self::DARK),
        (Appearance::Light, Self::LIGHT),
    ];

    pub fn parse(name: &str) -> Option<Appearance> {
        let lower = name.trim().to_ascii_lowercase();
        Self::NAMES
            .iter()
            .find(|(_, n)| *n == lower)
            .map(|(value, _)| *value)
    }

    pub const fn name(self) -> &'static str {
        match self {
            Appearance::Dark => Self::DARK,
            Appearance::Light => Self::LIGHT,
        }
    }

    pub fn accepted() -> Vec<&'static str> {
        Self::NAMES.iter().map(|(_, n)| *n).collect()
    }
}
