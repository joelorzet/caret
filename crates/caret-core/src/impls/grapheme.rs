#[allow(unused_imports)]
use crate::prelude::*;

impl<'a> Grapheme<'a> {
    /// The character the cluster is built on, which is the one a font has a
    /// glyph for and the one everything else falls back to.
    pub fn base(self) -> char {
        match self {
            Grapheme::One(ch) => ch,
            Grapheme::Many(text) => text.chars().next().unwrap_or(' '),
        }
    }

    pub fn chars(self) -> impl Iterator<Item = char> + 'a {
        let (one, many) = match self {
            Grapheme::One(ch) => (Some(ch), None),
            Grapheme::Many(text) => (None, Some(text.chars())),
        };
        one.into_iter().chain(many.into_iter().flatten())
    }

    /// The character the cluster ends with, which says whether it is
    /// waiting for another one.
    pub fn last_char(self) -> Option<char> {
        match self {
            Grapheme::One(ch) => Some(ch),
            Grapheme::Many(text) => text.chars().next_back(),
        }
    }

    pub fn is_cluster(self) -> bool {
        matches!(self, Grapheme::Many(_))
    }
}

impl core::fmt::Display for Grapheme<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Grapheme::One(ch) => f.write_str(ch.encode_utf8(&mut [0u8; 4])),
            Grapheme::Many(text) => f.write_str(text),
        }
    }
}
