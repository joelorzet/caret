#[allow(unused_imports)]
use crate::prelude::*;

use std::io::Write;
use std::process::{Command, Stdio};

use caret_vt::base64;

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
