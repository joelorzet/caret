#[allow(unused_imports)]
use crate::prelude::*;

impl Performer<'_> {
    /// Moves to a 1-based row and column, honouring origin mode (DECOM),
    /// where addressing is relative to the scroll region.
    pub(crate) fn goto(&mut self, row: u16, col: u16) {
        let origin = self.term.modes().contains(Modes::ORIGIN);
        let screen = self.term.screen_mut();
        let top = if origin { screen.region().top } else { 0 };
        let y = top.saturating_add(row.saturating_sub(1));
        let y = if origin {
            y.min(screen.region().bottom)
        } else {
            y
        };
        screen.move_to(col.saturating_sub(1), y);
    }

    pub(crate) fn tab_forward(&mut self, count: u16) {
        for _ in 0..count.max(1) {
            let x = self.term.screen().cursor().x;
            let next = self.term.tabs().next_stop(x);
            self.term.screen_mut().move_x(next);
        }
    }

    pub(crate) fn tab_backward(&mut self, count: u16) {
        for _ in 0..count.max(1) {
            let x = self.term.screen().cursor().x;
            let prev = self.term.tabs().prev_stop(x);
            self.term.screen_mut().move_x(prev);
        }
    }

    pub(crate) fn newline(&mut self) {
        self.term.screen_mut().line_feed();
        if self.term.modes().contains(Modes::LINE_FEED_NL) {
            self.term.screen_mut().carriage_return();
        }
    }
}
