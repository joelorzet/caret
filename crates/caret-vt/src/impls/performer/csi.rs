#[allow(unused_imports)]
use crate::prelude::*;

/// The marker byte that makes a CSI sequence a private one.
mod marker {
    /// DEC private, as in `CSI ? 25 h`.
    pub const DEC: u8 = b'?';
    /// Secondary device attributes, as in `CSI > c`.
    pub const SECONDARY: u8 = b'>';
    /// The space that turns `q` into DECSCUSR.
    pub const SPACE: u8 = b' ';
    /// The bang that turns `p` into DECSTR.
    pub const BANG: u8 = b'!';
}

/// Device Status Report requests.
mod report {
    /// "Are you there?"
    pub const STATUS: u16 = 5;
    /// "Where is the cursor?"
    pub const CURSOR_POSITION: u16 = 6;
}

/// What a terminal says about itself.
mod reply {
    /// We are here and in working order.
    pub const OK: &[u8] = b"\x1b[0n";
    /// Secondary DA: terminal type 0, firmware version, cartridge.
    pub const SECONDARY_ATTRIBUTES: &[u8] = b"\x1b[>0;10;1c";
    /// Primary DA: VT220 with 132 columns and ANSI colour.
    pub const PRIMARY_ATTRIBUTES: &[u8] = b"\x1b[?62;22c";
    /// The page number in a private cursor report; we only ever have one.
    pub const PAGE: u16 = 1;
}

/// Tab Clear.
mod tabs {
    pub const AT_CURSOR: u16 = 0;
    pub const ALL: u16 = 3;
}

impl Performer<'_> {
    pub(crate) fn csi(&mut self, params: &vte::Params, intermediates: &[u8], action: char) {
        let args = Args::new(params);
        let marker = intermediates.first().copied();
        let private = marker == Some(marker::DEC);

        match action {
            '@' => self.term.screen_mut().insert_cells(args.count(0, 1)),
            'A' => self.term.screen_mut().move_up(args.count(0, 1)),
            'B' | 'e' => self.term.screen_mut().move_down(args.count(0, 1)),
            'C' | 'a' => self.term.screen_mut().move_right(args.count(0, 1)),
            'D' => self.term.screen_mut().move_left(args.count(0, 1)),
            'E' => {
                self.term.screen_mut().move_down(args.count(0, 1));
                self.term.screen_mut().carriage_return();
            }
            'F' => {
                self.term.screen_mut().move_up(args.count(0, 1));
                self.term.screen_mut().carriage_return();
            }
            'G' | '`' => self
                .term
                .screen_mut()
                .move_x(args.count(0, 1).saturating_sub(1)),
            'H' | 'f' => self.goto(args.count(0, 1), args.count(1, 1)),
            'I' => self.tab_forward(args.count(0, 1)),
            'J' => self.erase_display(args.mode(0, 0)),
            'K' => self.erase_line(args.mode(0, 0)),
            'L' => self.term.screen_mut().insert_lines(args.count(0, 1)),
            'M' => self.term.screen_mut().delete_lines(args.count(0, 1)),
            'P' => self.term.screen_mut().delete_cells(args.count(0, 1)),
            'S' => self.term.screen_mut().scroll_up(args.count(0, 1)),
            'T' => self.term.screen_mut().scroll_down(args.count(0, 1)),
            'X' => self.term.screen_mut().erase_cells(args.count(0, 1)),
            'Z' => self.tab_backward(args.count(0, 1)),
            'd' => self
                .term
                .screen_mut()
                .move_y(args.count(0, 1).saturating_sub(1)),
            'g' => self.clear_tabs(args.mode(0, 0)),
            'h' => self.each_mode(&args, private, true),
            'l' => self.each_mode(&args, private, false),
            'm' => self.sgr(&args),
            'n' if marker.is_none() => self.report(args.mode(0, 0)),
            'n' if private => self.report_private(args.mode(0, 0)),
            'c' => self.device_attributes(marker),
            'q' if marker == Some(marker::SPACE) => {
                let style = CursorStyle::from_decscusr(args.mode(0, 0));
                self.term.set_cursor_style(style);
            }
            'p' if marker == Some(marker::BANG) => self.soft_reset(),

            // A private marker makes these different sequences entirely, and
            // acting on them as if it were not there moves the cursor on a
            // query that only asked a question: `CSI ? u` asks which keyboard
            // protocol is in use, and running it as `CSI u` restored a cursor
            // that was never saved, which is why a program's first frame left
            // the cursor at the top of the pane until you typed something.
            'r' if marker.is_none() => {
                let bottom = args.count(1, self.term.rows());
                self.term
                    .screen_mut()
                    .set_region(args.count(0, 1).saturating_sub(1), bottom.saturating_sub(1));
                self.goto(1, 1);
            }
            's' if marker.is_none() => self.term.screen_mut().save_cursor(),
            'u' if marker.is_none() => self.term.screen_mut().restore_cursor(),
            // Window manipulation and anything else is not acted on.
            _ => {}
        }
    }

    fn each_mode(&mut self, args: &Args, private: bool, on: bool) {
        for i in 0..args.len().max(1) {
            let code = args.mode(i, 0);
            if code != 0 {
                self.set_mode(private, code, on);
            }
        }
    }

    fn erase_display(&mut self, mode: u16) {
        match mode {
            0 => self.term.screen_mut().erase_display(EraseMode::ToEnd),
            1 => self.term.screen_mut().erase_display(EraseMode::ToStart),
            2 => self.term.screen_mut().erase_display(EraseMode::All),
            3 => self.term.screen_mut().erase_saved_lines(),
            _ => {}
        }
    }

    fn erase_line(&mut self, mode: u16) {
        match mode {
            0 => self.term.screen_mut().erase_line(EraseMode::ToEnd),
            1 => self.term.screen_mut().erase_line(EraseMode::ToStart),
            2 => self.term.screen_mut().erase_line(EraseMode::All),
            _ => {}
        }
    }

    fn clear_tabs(&mut self, mode: u16) {
        match mode {
            tabs::AT_CURSOR => {
                let x = self.term.screen().cursor().x;
                self.term.tabs_mut().unset(x);
            }
            tabs::ALL => self.term.tabs_mut().clear_all(),
            _ => {}
        }
    }

    /// Device Status Report.
    fn report(&mut self, mode: u16) {
        match mode {
            report::STATUS => self.term.reply(reply::OK),
            report::CURSOR_POSITION => {
                let (row, col) = self.cursor_one_based();
                self.term.reply(format!("\x1b[{row};{col}R").as_bytes());
            }
            _ => {}
        }
    }

    /// Where the cursor is, counted from one as the protocol counts, and
    /// relative to the scrolling region when origin mode is on.
    fn cursor_one_based(&self) -> (u16, u16) {
        let origin = self.term.modes().contains(Modes::ORIGIN);
        let screen = self.term.screen();
        let cursor = screen.cursor();
        let top = if origin { screen.region().top } else { 0 };
        (cursor.y.saturating_sub(top) + 1, cursor.x + 1)
    }

    /// `CSI ? Ps n`, which asks the same questions as `CSI Ps n` but wants
    /// the answer in the private form. Replying in the plain form would have
    /// the asker read our answer as something else entirely.
    fn report_private(&mut self, mode: u16) {
        if mode != report::CURSOR_POSITION {
            return;
        }
        let (row, col) = self.cursor_one_based();
        let page = reply::PAGE;
        self.term
            .reply(format!("\x1b[?{row};{col};{page}R").as_bytes());
    }

    fn device_attributes(&mut self, marker: Option<u8>) {
        match marker {
            Some(marker::SECONDARY) => self.term.reply(reply::SECONDARY_ATTRIBUTES),
            _ => self.term.reply(reply::PRIMARY_ATTRIBUTES),
        }
    }

    /// DECSTR: reset the things a soft reset covers, keeping the screen.
    fn soft_reset(&mut self) {
        self.term.set_mode(Modes::INSERT, false);
        self.term.set_mode(Modes::ORIGIN, false);
        self.term.set_mode(Modes::WRAP, true);
        self.term.set_mode(Modes::CURSOR_VISIBLE, true);
        self.term.set_cursor_style(CursorStyle::default());
        let screen = self.term.screen_mut();
        screen.reset_pen();
        screen.reset_region();
        screen.move_to(0, 0);
        screen.save_cursor();
    }
}
