#[allow(unused_imports)]
use crate::prelude::*;

impl Performer<'_> {
    /// SM / RM, and their DEC private variants when `private` is set.
    pub(crate) fn set_mode(&mut self, private: bool, code: u16, on: bool) {
        if private {
            if let Some(mode) = DecMode::from_code(code) {
                self.set_dec_mode(mode, on);
            }
            return;
        }
        match AnsiMode::from_code(code) {
            Some(AnsiMode::Insert) => self.term.set_mode(Modes::INSERT, on),
            Some(AnsiMode::LineFeedNewLine) => self.term.set_mode(Modes::LINE_FEED_NL, on),
            None => {}
        }
    }

    fn set_dec_mode(&mut self, mode: DecMode, on: bool) {
        match mode {
            DecMode::ApplicationCursor => self.term.set_mode(Modes::APP_CURSOR, on),
            DecMode::ReverseVideo => {
                self.term.set_mode(Modes::REVERSE_VIDEO, on);
                self.term.screen_mut().damage_all();
            }
            DecMode::Origin => {
                self.term.set_mode(Modes::ORIGIN, on);
                // DECOM homes the cursor inside the new addressing space.
                self.goto(1, 1);
            }
            DecMode::AutoWrap => self.term.set_mode(Modes::WRAP, on),
            DecMode::CursorBlink => {
                let mut style = self.term.cursor_style();
                style.blink = on;
                self.term.set_cursor_style(style);
            }
            DecMode::CursorVisible => self.term.set_mode(Modes::CURSOR_VISIBLE, on),
            DecMode::MouseClick => self.term.set_mode(Modes::MOUSE_CLICK, on),
            DecMode::MouseDrag => self.term.set_mode(Modes::MOUSE_DRAG, on),
            DecMode::MouseMotion => self.term.set_mode(Modes::MOUSE_MOTION, on),
            DecMode::FocusEvents => self.term.set_mode(Modes::FOCUS_EVENTS, on),
            DecMode::MouseSgr => self.term.set_mode(Modes::MOUSE_SGR, on),
            DecMode::AltScroll => self.term.set_mode(Modes::ALT_SCROLL, on),
            DecMode::SaveCursor => {
                if on {
                    self.term.screen_mut().save_cursor()
                } else {
                    self.term.screen_mut().restore_cursor()
                }
            }
            DecMode::AltScreen | DecMode::AltScreenClear | DecMode::AltScreenSaveClear => {
                if on {
                    self.term.enter_alt_screen()
                } else {
                    self.term.exit_alt_screen()
                }
            }
            DecMode::BracketedPaste => self.term.set_mode(Modes::BRACKETED_PASTE, on),
            DecMode::SynchronisedUpdate => self.term.set_mode(Modes::SYNC_UPDATE, on),
        }
    }
}
