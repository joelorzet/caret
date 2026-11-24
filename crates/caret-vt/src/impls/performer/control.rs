#[allow(unused_imports)]
use crate::prelude::*;

/// C0 and C1 control bytes, by the names the standard gives them.
mod code {
    /// BEL
    pub const BELL: u8 = 0x07;
    /// BS
    pub const BACKSPACE: u8 = 0x08;
    /// HT
    pub const TAB: u8 = 0x09;
    /// LF, VT and FF all move to the next line.
    pub const LINE_FEED: u8 = 0x0a;
    pub const FORM_FEED: u8 = 0x0c;
    /// CR
    pub const CARRIAGE_RETURN: u8 = 0x0d;
    /// SO and SI select a character set; only the default set is supported.
    pub const SHIFT_OUT: u8 = 0x0e;
    pub const SHIFT_IN: u8 = 0x0f;
    /// IND
    pub const INDEX: u8 = 0x84;
    /// NEL
    pub const NEXT_LINE: u8 = 0x85;
    /// HTS
    pub const SET_TAB: u8 = 0x88;
    /// RI
    pub const REVERSE_INDEX: u8 = 0x8d;
}

impl Performer<'_> {
    /// C0 and C1 control functions.
    pub(crate) fn control(&mut self, byte: u8) {
        match byte {
            code::BELL => self.term.push_event(TerminalEvent::Bell),
            code::BACKSPACE => self.term.screen_mut().move_left(1),
            code::TAB => self.tab_forward(1),
            code::LINE_FEED..=code::FORM_FEED => self.newline(),
            code::CARRIAGE_RETURN => self.term.screen_mut().carriage_return(),
            code::SHIFT_OUT | code::SHIFT_IN => {}
            code::INDEX => self.term.screen_mut().line_feed(),
            code::NEXT_LINE => {
                self.term.screen_mut().line_feed();
                self.term.screen_mut().carriage_return();
            }
            code::SET_TAB => {
                let x = self.term.screen().cursor().x;
                self.term.tabs_mut().set(x);
            }
            code::REVERSE_INDEX => self.term.screen_mut().reverse_line_feed(),
            _ => {}
        }
    }
}
