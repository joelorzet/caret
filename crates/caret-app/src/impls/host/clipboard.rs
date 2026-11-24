#[allow(unused_imports)]
use crate::prelude::*;

use std::io::Write;
use std::process::{Command, Stdio};

/// Puts `text` on the system clipboard.
///
/// Which clipboard that is depends on who owns the screen. Inside another
/// terminal, OSC 52 asks it to set its own, which also works over SSH. In a
/// window of our own there is nobody to ask, so the platform's clipboard is
/// written directly. Without it, copying here could not be pasted anywhere
/// else.
pub fn copy(text: &str, owns_window: bool) -> std::io::Result<()> {
    if owns_window {
        return write_system(text);
    }
    let payload = base64(text.as_bytes());
    let mut out = std::io::stdout();
    write!(out, "\x1b]52;c;{payload}\x07")?;
    out.flush()
}

/// Reads the system clipboard, for a paste we have to perform ourselves.
///
/// Inside another terminal this returns nothing: that terminal's own paste
/// shortcut sends the text as input, and asking the platform would fetch a
/// different clipboard than the one the user sees.
pub fn paste(owns_window: bool) -> Option<String> {
    if !owns_window {
        return None;
    }
    let (program, args) = read_command()?;
    let output = Command::new(program).args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn write_system(text: &str) -> std::io::Result<()> {
    let Some((program, args)) = write_command() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "no clipboard command on this system",
        ));
    };

    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(text.as_bytes())?;
    }
    // Dropping stdin closes it, which is what tells the helper to finish.
    drop(child.stdin.take());
    child.wait()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn write_command() -> Option<(&'static str, &'static [&'static str])> {
    Some(("pbcopy", &[]))
}

#[cfg(target_os = "macos")]
fn read_command() -> Option<(&'static str, &'static [&'static str])> {
    Some(("pbpaste", &[]))
}

/// Wayland first, then X11: a session may have either, and the helpers are
/// not interchangeable.
#[cfg(all(unix, not(target_os = "macos")))]
fn write_command() -> Option<(&'static str, &'static [&'static str])> {
    const CANDIDATES: [(&str, &[&str]); 3] = [
        ("wl-copy", &[]),
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard", "--input"]),
    ];
    CANDIDATES.into_iter().find(|(program, _)| on_path(program))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn read_command() -> Option<(&'static str, &'static [&'static str])> {
    const CANDIDATES: [(&str, &[&str]); 3] = [
        ("wl-paste", &["--no-newline"]),
        ("xclip", &["-selection", "clipboard", "-o"]),
        ("xsel", &["--clipboard", "--output"]),
    ];
    CANDIDATES.into_iter().find(|(program, _)| on_path(program))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn on_path(program: &str) -> bool {
    std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).any(|dir| dir.join(program).is_file()))
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn write_command() -> Option<(&'static str, &'static [&'static str])> {
    Some(("clip", &[]))
}

#[cfg(not(unix))]
fn read_command() -> Option<(&'static str, &'static [&'static str])> {
    None
}

/// Encodes bytes as base64, which is how OSC 52 carries a selection.
///
/// Nothing here is about written language. OSC 52 is a text protocol, so
/// arbitrary bytes have to travel as printable characters, and base64 is the
/// encoding it specifies. The text is already UTF-8 by the time it arrives,
/// so every script encodes the same way; what comes back out is the bytes
/// that went in.
pub fn base64(data: &[u8]) -> String {
    /// The 64 printable characters base64 maps its six-bit groups onto, in
    /// the order RFC 4648 gives them. This is a wire format, not a choice:
    /// a different order would decode to different bytes at the other end.
    const DIGITS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    /// Three bytes in, four characters out.
    const GROUP: usize = 3;
    const ENCODED: usize = 4;
    /// Six bits per character, and the mask that takes them.
    const BITS: u32 = 6;
    const MASK: u32 = (1 << BITS) - 1;
    /// What stands in for a group that was not full.
    const PAD: char = '=';

    let mut out = String::with_capacity(data.len().div_ceil(GROUP) * ENCODED);
    for chunk in data.chunks(GROUP) {
        let bytes = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let packed =
            (bytes[0] as u32) << (BITS * 2 + 4) | (bytes[1] as u32) << (BITS + 2) | bytes[2] as u32;

        // The high six bits first, then down. The last two characters are
        // padding when the group they would describe was not there.
        let digit = |shift: u32| DIGITS[(packed >> shift & MASK) as usize] as char;
        out.push(digit(BITS * 3));
        out.push(digit(BITS * 2));
        out.push(if chunk.len() > 1 { digit(BITS) } else { PAD });
        out.push(if chunk.len() > 2 { digit(0) } else { PAD });
    }
    out
}
