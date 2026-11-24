#[allow(unused_imports)]
use crate::prelude::*;

/// The final byte of an escape sequence, by the name the standard gives it.
mod code {
    /// DECSC
    pub const SAVE_CURSOR: u8 = b'7';
    /// DECRC, and with the `#` intermediate, DECALN.
    pub const RESTORE_CURSOR: u8 = b'8';
    /// IND
    pub const INDEX: u8 = b'D';
    /// NEL
    pub const NEXT_LINE: u8 = b'E';
    /// HTS
    pub const SET_TAB: u8 = b'H';
    /// RI
    pub const REVERSE_INDEX: u8 = b'M';
    /// RIS
    pub const RESET: u8 = b'c';
    /// DECKPAM
    pub const APPLICATION_KEYPAD: u8 = b'=';
    /// DECKPNM
    pub const NORMAL_KEYPAD: u8 = b'>';
    /// The intermediate that turns `8` into the alignment test.
    pub const DEC_PRIVATE: u8 = b'#';
}

/// What DECALN fills the screen with.
const ALIGNMENT_GLYPH: char = 'E';

impl Performer<'_> {
    pub(crate) fn esc(&mut self, intermediates: &[u8], byte: u8) {
        match (byte, intermediates.first().copied()) {
            (code::SAVE_CURSOR, None) => self.term.screen_mut().save_cursor(),
            (code::RESTORE_CURSOR, None) => self.term.screen_mut().restore_cursor(),
            // DECALN fills the screen with E, for alignment tests.
            (code::RESTORE_CURSOR, Some(code::DEC_PRIVATE)) => self.align_test(),
            (code::INDEX, None) => self.term.screen_mut().line_feed(),
            (code::NEXT_LINE, None) => {
                self.term.screen_mut().line_feed();
                self.term.screen_mut().carriage_return();
            }
            (code::SET_TAB, None) => {
                let x = self.term.screen().cursor().x;
                self.term.tabs_mut().set(x);
            }
            (code::REVERSE_INDEX, None) => self.term.screen_mut().reverse_line_feed(),
            (code::RESET, None) => self.term.reset(),
            (code::APPLICATION_KEYPAD, None) => self.term.set_mode(Modes::APP_KEYPAD, true),
            (code::NORMAL_KEYPAD, None) => self.term.set_mode(Modes::APP_KEYPAD, false),
            // Character set selection; only the default set is supported.
            _ => {}
        }
    }

    fn align_test(&mut self) {
        let (cols, rows) = (self.term.cols(), self.term.rows());
        let screen = self.term.screen_mut();
        for y in 0..rows {
            for cell in screen.line_mut(y).iter_mut().take(cols as usize) {
                cell.ch = ALIGNMENT_GLYPH;
            }
        }
        screen.move_to(0, 0);
        screen.damage_all();
    }
}
