#[allow(unused_imports)]
use crate::prelude::*;

use unicode_normalization::char::{compose, is_combining_mark};
use unicode_width::UnicodeWidthChar;

/// Joins a character to the one before it without being drawn itself.
const ZWJ: char = '\u{200D}';
const ZWNJ: char = '\u{200C}';

/// Chooses between the text and emoji presentations of the character
/// before them. Nothing here draws differently for either, but they belong
/// to the character rather than to the cell after it.
const VARIATION_SELECTORS: [core::ops::RangeInclusive<char>; 2] =
    ['\u{FE00}'..='\u{FE0F}', '\u{E0100}'..='\u{E01EF}'];

/// The skin tone that follows a person, which is part of that person.
const SKIN_TONES: core::ops::RangeInclusive<char> = '\u{1F3FB}'..='\u{1F3FF}';

/// Two of these are one flag.
const REGIONAL_INDICATORS: core::ops::RangeInclusive<char> = '\u{1F1E6}'..='\u{1F1FF}';

impl Performer<'_> {
    pub(crate) fn print_char(&mut self, c: char) {
        if self.combines(c) {
            return;
        }
        let width = c.width().unwrap_or(0) as u16;
        let autowrap = self.term.wrap_enabled();
        let insert = self.term.insert_mode();
        self.term.screen_mut().print(c, width, autowrap, insert);
    }

    /// Whether `c` belongs to the character already printed rather than to a
    /// cell of its own, and joins it if so.
    ///
    /// Which characters combine is decided here, where the Unicode tables
    /// already are, and `caret-core` only has to hold the result. A mark
    /// that has a precomposed form becomes that single character instead of
    /// a cluster, which costs nothing to store, draws with the glyph a font
    /// already has, and is what a search for the word will match.
    fn combines(&mut self, c: char) -> bool {
        let screen = self.term.screen_mut();

        if extends(c) {
            if let Some(Grapheme::One(base)) = screen.last_grapheme() {
                if let Some(composed) = compose(base, c) {
                    return screen.recompose(composed);
                }
            }
            // A mark with nothing to attach to is dropped either way: it has
            // no width, so there is no cell for it to be.
            screen.attach(c);
            return true;
        }

        if c == ZWJ || c == ZWNJ {
            return screen.attach(c);
        }

        // What follows a joiner is the other half of what it joined.
        if screen
            .last_grapheme()
            .is_some_and(|last| last.last_char() == Some(ZWJ))
        {
            return screen.attach(c);
        }

        // A flag is a pair, and only a pair: the next one starts a new flag.
        if REGIONAL_INDICATORS.contains(&c)
            && screen.last_grapheme().is_some_and(
                |last| matches!(last, Grapheme::One(one) if REGIONAL_INDICATORS.contains(&one)),
            )
        {
            return screen.attach(c);
        }

        false
    }
}

/// Whether `c` is written on the character before it rather than beside it.
fn extends(c: char) -> bool {
    is_combining_mark(c)
        || SKIN_TONES.contains(&c)
        || VARIATION_SELECTORS.iter().any(|range| range.contains(&c))
}
