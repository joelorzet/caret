#[allow(unused_imports)]
use crate::prelude::*;

/// SGR parameters, by the attribute each one selects.
mod code {
    pub const RESET: u16 = 0;
    pub const BOLD: u16 = 1;
    pub const DIM: u16 = 2;
    pub const ITALIC: u16 = 3;
    pub const UNDERLINE: u16 = 4;
    /// Slow and rapid blink; we draw them the same.
    pub const BLINK: u16 = 5;
    pub const BLINK_RAPID: u16 = 6;
    pub const REVERSE: u16 = 7;
    pub const HIDDEN: u16 = 8;
    pub const STRIKE: u16 = 9;
    pub const UNDERLINE_DOUBLE: u16 = 21;
    /// Cancels bold and dim together, as the standard has it.
    pub const NORMAL_WEIGHT: u16 = 22;
    pub const NO_ITALIC: u16 = 23;
    pub const NO_UNDERLINE: u16 = 24;
    pub const NO_BLINK: u16 = 25;
    pub const NO_REVERSE: u16 = 27;
    pub const NO_HIDDEN: u16 = 28;
    pub const NO_STRIKE: u16 = 29;

    pub const FG_FIRST: u16 = 30;
    pub const FG_LAST: u16 = 37;
    pub const FG_EXTENDED: u16 = 38;
    pub const FG_DEFAULT: u16 = 39;
    pub const BG_FIRST: u16 = 40;
    pub const BG_LAST: u16 = 47;
    pub const BG_EXTENDED: u16 = 48;
    pub const BG_DEFAULT: u16 = 49;
    pub const FG_BRIGHT_FIRST: u16 = 90;
    pub const FG_BRIGHT_LAST: u16 = 97;
    pub const BG_BRIGHT_FIRST: u16 = 100;
    pub const BG_BRIGHT_LAST: u16 = 107;

    /// Where the bright half of the palette starts.
    pub const BRIGHT_OFFSET: u8 = 8;
}

/// The selector inside `38`/`48`, saying which kind of colour follows.
mod colour_kind {
    pub const TRUE_COLOUR: u16 = 2;
    pub const INDEXED: u16 = 5;
}

/// The sub-parameter of `4:n`, picking an underline style.
mod underline {
    pub const OFF: u16 = 0;
    pub const SINGLE: u16 = 1;
    pub const DOUBLE: u16 = 2;
    /// Curly, dotted and dashed; we draw them all curly.
    pub const CURLY: u16 = 3;
    pub const DOTTED: u16 = 4;
    pub const DASHED: u16 = 5;
}

/// The widest value a true-colour channel can carry.
const CHANNEL_MAX: u16 = 255;

impl Performer<'_> {
    /// Select Graphic Rendition: text attributes and colours.
    pub(crate) fn sgr(&mut self, args: &Args) {
        if args.len() == 0 {
            self.term.screen_mut().reset_pen();
            return;
        }

        let mut i = 0;
        while i < args.len() {
            let sub = args.subparams(i);
            let code = sub.first().copied().unwrap_or(0);

            match code {
                code::RESET => self.term.screen_mut().reset_pen(),
                code::BOLD => self.set_flag(Flags::BOLD, true),
                code::DIM => self.set_flag(Flags::DIM, true),
                code::ITALIC => self.set_flag(Flags::ITALIC, true),
                code::UNDERLINE => self.underline(sub),
                code::BLINK | code::BLINK_RAPID => self.set_flag(Flags::BLINK, true),
                code::REVERSE => self.set_flag(Flags::REVERSE, true),
                code::HIDDEN => self.set_flag(Flags::HIDDEN, true),
                code::STRIKE => self.set_flag(Flags::STRIKE, true),
                code::UNDERLINE_DOUBLE => self.underline_style(Flags::UNDERLINE_DOUBLE),
                code::NORMAL_WEIGHT => {
                    self.set_flag(Flags::BOLD, false);
                    self.set_flag(Flags::DIM, false);
                }
                code::NO_ITALIC => self.set_flag(Flags::ITALIC, false),
                code::NO_UNDERLINE => self.clear_underline(),
                code::NO_BLINK => self.set_flag(Flags::BLINK, false),
                code::NO_REVERSE => self.set_flag(Flags::REVERSE, false),
                code::NO_HIDDEN => self.set_flag(Flags::HIDDEN, false),
                code::NO_STRIKE => self.set_flag(Flags::STRIKE, false),
                code::FG_FIRST..=code::FG_LAST => self
                    .term
                    .screen_mut()
                    .set_pen_fg(Color::indexed((code - code::FG_FIRST) as u8)),
                code::FG_EXTENDED => i += self.extended_colour(args, i, true),
                code::FG_DEFAULT => self.term.screen_mut().set_pen_fg(Color::DEFAULT),
                code::BG_FIRST..=code::BG_LAST => self
                    .term
                    .screen_mut()
                    .set_pen_bg(Color::indexed((code - code::BG_FIRST) as u8)),
                code::BG_EXTENDED => i += self.extended_colour(args, i, false),
                code::BG_DEFAULT => self.term.screen_mut().set_pen_bg(Color::DEFAULT),
                code::FG_BRIGHT_FIRST..=code::FG_BRIGHT_LAST => {
                    let index = (code - code::FG_BRIGHT_FIRST) as u8 + code::BRIGHT_OFFSET;
                    self.term.screen_mut().set_pen_fg(Color::indexed(index));
                }
                code::BG_BRIGHT_FIRST..=code::BG_BRIGHT_LAST => {
                    let index = (code - code::BG_BRIGHT_FIRST) as u8 + code::BRIGHT_OFFSET;
                    self.term.screen_mut().set_pen_bg(Color::indexed(index));
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn set_flag(&mut self, flag: Flags, on: bool) {
        self.term.screen_mut().pen_flags_mut().set(flag, on);
    }

    /// `4` alone underlines; `4:n` picks a style, where `4:0` turns it off.
    fn underline(&mut self, sub: &[u16]) {
        match sub.get(1).copied() {
            None | Some(underline::SINGLE) => self.underline_style(Flags::UNDERLINE),
            Some(underline::OFF) => self.clear_underline(),
            Some(underline::DOUBLE) => self.underline_style(Flags::UNDERLINE_DOUBLE),
            Some(underline::CURLY | underline::DOTTED | underline::DASHED) => {
                self.underline_style(Flags::UNDERLINE_CURLY)
            }
            Some(_) => self.underline_style(Flags::UNDERLINE),
        }
    }

    fn underline_style(&mut self, style: Flags) {
        self.clear_underline();
        self.set_flag(style, true);
    }

    fn clear_underline(&mut self) {
        let flags = self.term.screen_mut().pen_flags_mut();
        flags.remove(Flags::UNDERLINE);
        flags.remove(Flags::UNDERLINE_DOUBLE);
        flags.remove(Flags::UNDERLINE_CURLY);
    }

    /// Handles `38`/`48` in both the colon form (`38:2:r:g:b`, all in one
    /// parameter) and the legacy semicolon form (`38;2;r;g;b`, spread across
    /// parameters). Returns how many extra parameters were consumed.
    fn extended_colour(&mut self, args: &Args, index: usize, foreground: bool) -> usize {
        let sub = args.subparams(index);

        // Colon form: everything is in this one parameter.
        if sub.len() > 1 {
            let colour = match sub[1] {
                colour_kind::TRUE_COLOUR => {
                    // Some emitters include a colour-space id: 38:2::r:g:b
                    let offset = if sub.len() >= 6 { 3 } else { 2 };
                    rgb(sub.get(offset), sub.get(offset + 1), sub.get(offset + 2))
                }
                colour_kind::INDEXED => sub.get(2).map(|i| Color::indexed(*i as u8)),
                _ => None,
            };
            if let Some(colour) = colour {
                self.apply_colour(colour, foreground);
            }
            return 0;
        }

        // Semicolon form.
        match args.subparams(index + 1).first().copied() {
            Some(colour_kind::TRUE_COLOUR) => {
                let colour = rgb(
                    args.subparams(index + 2).first(),
                    args.subparams(index + 3).first(),
                    args.subparams(index + 4).first(),
                );
                if let Some(colour) = colour {
                    self.apply_colour(colour, foreground);
                }
                4
            }
            Some(colour_kind::INDEXED) => {
                if let Some(i) = args.subparams(index + 2).first() {
                    self.apply_colour(Color::indexed(*i as u8), foreground);
                }
                2
            }
            _ => 1,
        }
    }

    fn apply_colour(&mut self, colour: Color, foreground: bool) {
        if foreground {
            self.term.screen_mut().set_pen_fg(colour);
        } else {
            self.term.screen_mut().set_pen_bg(colour);
        }
    }
}

fn rgb(r: Option<&u16>, g: Option<&u16>, b: Option<&u16>) -> Option<Color> {
    Some(Color::rgb(
        (*r?).min(CHANNEL_MAX) as u8,
        (*g?).min(CHANNEL_MAX) as u8,
        (*b?).min(CHANNEL_MAX) as u8,
    ))
}
