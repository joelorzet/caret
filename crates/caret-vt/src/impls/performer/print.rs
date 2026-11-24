#[allow(unused_imports)]
use crate::prelude::*;

use unicode_width::UnicodeWidthChar;

impl Performer<'_> {
    pub(crate) fn print_char(&mut self, c: char) {
        let width = c.width().unwrap_or(0) as u16;
        let autowrap = self.term.wrap_enabled();
        let insert = self.term.insert_mode();
        self.term.screen_mut().print(c, width, autowrap, insert);
    }
}
