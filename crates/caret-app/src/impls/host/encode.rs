#[allow(unused_imports)]
use crate::prelude::*;

/// The bytes a child expects for a key.
///
/// Arrows and Home/End have two encodings and the application chooses between
/// them with DECCKM, so they cannot simply be forwarded as received: `less`
/// and `vim` set application cursor mode and would otherwise see the wrong
/// sequence.
pub fn encode_for_child(binding: &KeyBinding, app_cursor: bool) -> Option<Vec<u8>> {
    if let Some(bytes) = line_editing(binding, app_cursor) {
        return Some(bytes.into_bytes());
    }

    let modifier = modifier_param(binding.mods);

    let text = match binding.key {
        Key::Up => cursor_key(b'A', app_cursor, modifier),
        Key::Down => cursor_key(b'B', app_cursor, modifier),
        Key::Right => cursor_key(b'C', app_cursor, modifier),
        Key::Left => cursor_key(b'D', app_cursor, modifier),
        Key::Home => cursor_key(b'H', app_cursor, modifier),
        Key::End => cursor_key(b'F', app_cursor, modifier),
        Key::Insert => tilde_key(2, modifier),
        Key::Delete => tilde_key(3, modifier),
        Key::PageUp => tilde_key(5, modifier),
        Key::PageDown => tilde_key(6, modifier),
        Key::Enter => "\r".to_string(),
        Key::Tab if binding.mods.contains(Modifiers::SHIFT) => "\x1b[Z".to_string(),
        Key::Tab => "\t".to_string(),
        Key::Backspace => "\x7f".to_string(),
        Key::Escape => "\x1b".to_string(),
        Key::F(n) => function_key(n, modifier)?,
        // Characters are already correct as typed.
        Key::Char(_) => return None,
    };
    Some(text.into_bytes())
}

/// The line-editing keys every other terminal on this platform has.
///
/// Readline (and zle, and fish) already implement word and line motions; they
/// just listen on sequences that Option and Cmd do not produce by themselves.
/// Translating here means long command lines can be edited the way the rest of
/// the system edits text, and it costs the child nothing.
fn line_editing(binding: &KeyBinding, app_cursor: bool) -> Option<String> {
    let alt = binding.mods == Modifiers::ALT;
    let cmd = binding.mods == Modifiers::SUPER;

    let text = match binding.key {
        // Home and End rather than ^A/^E: an editor understands them too.
        Key::Left if cmd => cursor_key(b'H', app_cursor, None),
        Key::Right if cmd => cursor_key(b'F', app_cursor, None),
        Key::Left if alt => "\x1bb".to_string(),
        Key::Right if alt => "\x1bf".to_string(),
        Key::Backspace if cmd => "\x15".to_string(),
        Key::Backspace if alt => "\x1b\x7f".to_string(),
        Key::Delete if cmd => "\x0b".to_string(),
        Key::Delete if alt => "\x1bd".to_string(),
        _ => return None,
    };
    Some(text)
}

/// Arrows switch between CSI and SS3 with DECCKM, but only when unmodified.
fn cursor_key(final_byte: u8, app_cursor: bool, modifier: Option<u8>) -> String {
    let final_char = final_byte as char;
    match modifier {
        Some(m) => format!("\x1b[1;{m}{final_char}"),
        None if app_cursor => format!("\x1bO{final_char}"),
        None => format!("\x1b[{final_char}"),
    }
}

fn tilde_key(number: u8, modifier: Option<u8>) -> String {
    match modifier {
        Some(m) => format!("\x1b[{number};{m}~"),
        None => format!("\x1b[{number}~"),
    }
}

fn function_key(n: u8, modifier: Option<u8>) -> Option<String> {
    // F1-F4 are SS3; the rest use the numbered tilde form, with the gaps
    // xterm's table has.
    // The tilde numbers are not F-number plus a constant: xterm's table has
    // gaps at 16, 22 and 27, so each run carries its own offset.
    const SS3_LAST: u8 = 4;
    const FIRST_SS3_FINAL: u8 = b'P';
    const RUNS: [(u8, u8, u8); 3] = [(5, 10, 10), (11, 14, 11), (15, 18, 12)];

    if (1..=SS3_LAST).contains(&n) {
        let final_char = (FIRST_SS3_FINAL + (n - 1)) as char;
        return Some(match modifier {
            Some(m) => format!("\x1b[1;{m}{final_char}"),
            None => format!("\x1bO{final_char}"),
        });
    }
    let number = RUNS
        .iter()
        .find(|(first, last, _)| n >= *first && n <= *last)
        .map(|(_, _, offset)| n + offset)?;
    Some(tilde_key(number, modifier))
}

/// xterm's modifier parameter: 1 plus a bitmask.
fn modifier_param(mods: Modifiers) -> Option<u8> {
    let mut bits = 0;
    if mods.contains(Modifiers::SHIFT) {
        bits |= 1;
    }
    if mods.contains(Modifiers::ALT) {
        bits |= 2;
    }
    if mods.contains(Modifiers::CTRL) {
        bits |= 4;
    }
    if bits == 0 {
        None
    } else {
        Some(bits + 1)
    }
}
