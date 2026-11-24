#[allow(unused_imports)]
use crate::prelude::*;

/// The control bytes a keyboard sends, by the key that sends them.
mod byte {
    pub const ESCAPE: u8 = 0x1b;
    /// NUL, which is what ctrl and space produce.
    pub const CTRL_SPACE: u8 = 0x00;
    /// Backspace arrives as either, depending on the terminal.
    pub const DELETE: u8 = 0x7f;
    pub const BACKSPACE: u8 = 0x08;
    /// Ctrl and a letter: ctrl+a is 1, through ctrl+z at 26.
    pub const CTRL_LETTER_FIRST: u8 = 0x01;
    pub const CTRL_LETTER_LAST: u8 = 0x1a;
    /// Ctrl and 4 through 7, which land just past the letters.
    pub const CTRL_DIGIT_FIRST: u8 = 0x1c;
    pub const CTRL_DIGIT_LAST: u8 = 0x1f;
    /// The first digit the ctrl+digit range stands for.
    pub const FIRST_CTRL_DIGIT: u8 = b'4';
}

/// One byte consumed.
const ONE: usize = 1;

impl InputDecoder {
    /// Decodes `bytes`, keeping any trailing partial sequence for next time.
    pub fn feed(&mut self, bytes: &[u8], out: &mut Vec<InputEvent>) {
        self.pending.extend_from_slice(bytes);
        let mut at = 0;

        while at < self.pending.len() {
            match self.decode(at) {
                Some((event, used)) => {
                    out.push(event);
                    at += used;
                }
                // An incomplete sequence: wait for the rest.
                None => break,
            }
        }
        self.pending.drain(..at);
    }

    fn decode(&self, at: usize) -> Option<(InputEvent, usize)> {
        let rest = &self.pending[at..];
        let first = *rest.first()?;

        if first == byte::ESCAPE {
            return self.decode_escape(rest);
        }

        let (key, mods, used) = match first {
            b'\r' | b'\n' => (Key::Enter, Modifiers::NONE, ONE),
            b'\t' => (Key::Tab, Modifiers::NONE, ONE),
            byte::DELETE | byte::BACKSPACE => (Key::Backspace, Modifiers::NONE, ONE),
            byte::CTRL_SPACE => (Key::Char(' '), Modifiers::CTRL, ONE),
            // C0 controls are ctrl plus a letter.
            byte::CTRL_LETTER_FIRST..=byte::CTRL_LETTER_LAST => {
                let letter = b'a' + first - byte::CTRL_LETTER_FIRST;
                (Key::Char(letter as char), Modifiers::CTRL, ONE)
            }
            byte::CTRL_DIGIT_FIRST..=byte::CTRL_DIGIT_LAST => {
                let digit = byte::FIRST_CTRL_DIGIT + first - byte::CTRL_DIGIT_FIRST;
                (Key::Char(digit as char), Modifiers::CTRL, ONE)
            }
            _ => return decode_utf8(rest),
        };

        Some((
            InputEvent::Key(KeyPress {
                binding: KeyBinding::new(mods, key),
                bytes: rest[..used].to_vec(),
            }),
            used,
        ))
    }

    fn decode_escape(&self, rest: &[u8]) -> Option<(InputEvent, usize)> {
        // A lone ESC at the end of the buffer could still be the start of a
        // sequence, but waiting forever would swallow the Escape key. The
        // buffer is drained per read, so treating it as Escape is correct
        // here: a real sequence arrives in one read.
        if rest.len() == 1 {
            return Some((press(Modifiers::NONE, Key::Escape, rest, 1), 1));
        }

        // A bracketed paste is one event, not a stream of keystrokes, so the
        // whole payload is gathered before anything is reported.
        if rest.starts_with(PASTE_START) {
            let body = &rest[PASTE_START.len()..];
            let end = find(body, PASTE_END)?;
            let used = PASTE_START.len() + end + PASTE_END.len();
            return Some((InputEvent::Paste(body[..end].to_vec()), used));
        }

        match rest[1] {
            b'[' => self.decode_csi(rest),
            b'O' => {
                let final_byte = *rest.get(2)?;
                let key = ss3_key(final_byte)?;
                Some((press(Modifiers::NONE, key, rest, 3), 3))
            }
            // ESC followed by anything else is Alt plus that key.
            _ => {
                let (inner, used) = decode_utf8(&rest[1..])?;
                let InputEvent::Key(inner) = inner else {
                    return None;
                };
                let mut binding = inner.binding;
                binding.mods.insert(Modifiers::ALT);
                Some((
                    InputEvent::Key(KeyPress {
                        binding,
                        bytes: rest[..used + 1].to_vec(),
                    }),
                    used + 1,
                ))
            }
        }
    }

    fn decode_csi(&self, rest: &[u8]) -> Option<(InputEvent, usize)> {
        let mut end = 2;
        while end < rest.len() && !matches!(rest[end], b'@'..=b'~') {
            end += 1;
        }
        if end >= rest.len() {
            return None;
        }
        let final_byte = rest[end];
        let params = core::str::from_utf8(&rest[2..end]).ok()?;
        let used = end + 1;

        // `CSI <` introduces an SGR mouse report, whose parameters are not
        // the plain numbers the key decoding below expects.
        if let Some(report) = params.strip_prefix('<') {
            return match mouse::decode_sgr(report, final_byte) {
                Some(event) => Some((InputEvent::Mouse(event), used)),
                // A malformed report is consumed rather than leaked as text.
                None => Some((press(Modifiers::NONE, Key::Escape, rest, used), used)),
            };
        }

        let numbers: Vec<u16> = params
            .split(';')
            .map(|p| p.parse::<u16>().unwrap_or(0))
            .collect();
        let mods = numbers
            .get(1)
            .copied()
            .map(xterm_modifiers)
            .unwrap_or(Modifiers::NONE);

        let key = match final_byte {
            b'A' => Key::Up,
            b'B' => Key::Down,
            b'C' => Key::Right,
            b'D' => Key::Left,
            b'H' => Key::Home,
            b'F' => Key::End,
            b'Z' => return Some((press(Modifiers::SHIFT, Key::Tab, rest, used), used)),
            b'~' => match numbers.first().copied().unwrap_or(0) {
                1 | 7 => Key::Home,
                2 => Key::Insert,
                3 => Key::Delete,
                4 | 8 => Key::End,
                5 => Key::PageUp,
                6 => Key::PageDown,
                n @ 11..=15 => Key::F((n - 10) as u8),
                n @ 17..=21 => Key::F((n - 11) as u8),
                n @ 23..=26 => Key::F((n - 12) as u8),
                _ => return Some((press(Modifiers::NONE, Key::Escape, rest, used), used)),
            },
            _ => return Some((press(Modifiers::NONE, Key::Escape, rest, used), used)),
        };

        Some((press(mods, key, rest, used), used))
    }
}

const PASTE_START: &[u8] = b"\x1b[200~";
const PASTE_END: &[u8] = b"\x1b[201~";

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn press(mods: Modifiers, key: Key, bytes: &[u8], used: usize) -> InputEvent {
    InputEvent::Key(KeyPress {
        binding: KeyBinding::new(mods, key),
        bytes: bytes[..used].to_vec(),
    })
}

/// xterm encodes modifiers as 1 plus a bitmask.
fn xterm_modifiers(value: u16) -> Modifiers {
    let bits = value.saturating_sub(1);
    let mut mods = Modifiers::NONE;
    if bits & 1 != 0 {
        mods.insert(Modifiers::SHIFT);
    }
    if bits & 2 != 0 {
        mods.insert(Modifiers::ALT);
    }
    if bits & 4 != 0 {
        mods.insert(Modifiers::CTRL);
    }
    if bits & 8 != 0 {
        mods.insert(Modifiers::SUPER);
    }
    mods
}

fn ss3_key(byte: u8) -> Option<Key> {
    Some(match byte {
        b'A' => Key::Up,
        b'B' => Key::Down,
        b'C' => Key::Right,
        b'D' => Key::Left,
        b'H' => Key::Home,
        b'F' => Key::End,
        b'P' => Key::F(1),
        b'Q' => Key::F(2),
        b'R' => Key::F(3),
        b'S' => Key::F(4),
        _ => return None,
    })
}

/// Decodes one UTF-8 character, or `None` if it is still incomplete.
fn decode_utf8(bytes: &[u8]) -> Option<(InputEvent, usize)> {
    let first = *bytes.first()?;
    let len = match first {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf7 => 4,
        // Not a valid leading byte; drop it rather than stalling.
        _ => return Some((press(Modifiers::NONE, Key::Char('\u{fffd}'), bytes, 1), 1)),
    };
    if bytes.len() < len {
        return None;
    }
    let text = core::str::from_utf8(&bytes[..len]).ok()?;
    let ch = text.chars().next()?;
    Some((press(Modifiers::NONE, Key::Char(ch), bytes, len), len))
}
