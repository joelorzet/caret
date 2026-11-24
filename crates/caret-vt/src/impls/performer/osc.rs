#[allow(unused_imports)]
use crate::prelude::*;

/// OSC command numbers, as they arrive on the wire.
mod code {
    pub const ICON_AND_TITLE: &[u8] = b"0";
    pub const ICON: &[u8] = b"1";
    pub const TITLE: &[u8] = b"2";
    pub const WORKING_DIRECTORY: &[u8] = b"7";
    pub const SHELL_INTEGRATION: &[u8] = b"133";
    pub const CLIPBOARD: &[u8] = b"52";
}

/// How much encoded clipboard text to accept at once.
///
/// A megabyte of base64 is three quarters of a megabyte of text, which is
/// more than anybody means to copy and enough to notice if a program is
/// filling the clipboard on purpose.
const MAX_CLIPBOARD: usize = 1 << 20;

/// Shell-integration marks within OSC 133: where the prompt starts, where
/// what you typed starts, where the output starts, and what it exited
/// with.
mod mark {
    pub const PROMPT_START: &[u8] = b"A";
    pub const INPUT_START: &[u8] = b"B";
    pub const OUTPUT_START: &[u8] = b"C";
    pub const FINISHED: &[u8] = b"D";
}

impl Performer<'_> {
    pub(crate) fn osc(&mut self, params: &[&[u8]]) {
        let Some(code) = params.first() else { return };

        match *code {
            code::ICON_AND_TITLE | code::TITLE => {
                if let Some(title) = params.get(1) {
                    self.term.set_title(String::from_utf8_lossy(title));
                }
            }
            // OSC 7 reports the shell's directory as a file:// URL. The
            // explorer follows it, so the host part is dropped and the path
            // percent-decoded.
            code::WORKING_DIRECTORY => {
                if let Some(url) = params.get(1) {
                    if let Some(path) = path_from_url(&String::from_utf8_lossy(url)) {
                        self.term.set_cwd(path);
                    }
                }
            }
            // Shell integration marks; the prompt boundary is the useful one.
            code::SHELL_INTEGRATION => match params.get(1).copied() {
                Some(mark::PROMPT_START) => {
                    self.term.screen_mut().mark_prompt();
                    self.term.push_event(TerminalEvent::PromptMarked);
                }
                Some(mark::OUTPUT_START) => {
                    self.term.push_event(TerminalEvent::CommandStarted);
                }
                Some(mark::FINISHED) => {
                    // `D` alone means finished without saying how, which
                    // is what a shell sends when there was nothing to run.
                    let status = params
                        .get(2)
                        .and_then(|text| String::from_utf8_lossy(text).trim().parse().ok());
                    self.term.push_event(TerminalEvent::CommandFinished(status));
                }
                // Where input begins: part of the vocabulary, and
                // nothing here needs it.
                Some(mark::INPUT_START) => {}
                _ => {}
            },

            // A program asking for text to go on the clipboard. Reading
            // it back is never answered: a program that can ask what is on
            // the clipboard can read what was copied somewhere else
            // entirely, and no amount of convenience is worth that.
            code::CLIPBOARD => {
                let Some(data) = params.get(2) else { return };
                if data.first() == Some(&b'?') {
                    return;
                }
                if data.len() > MAX_CLIPBOARD {
                    return;
                }
                let Some(bytes) = crate::unbase64(&String::from_utf8_lossy(data)) else {
                    return;
                };
                let Ok(text) = String::from_utf8(bytes) else {
                    return;
                };
                self.term.push_event(TerminalEvent::ClipboardWrite(text));
            }

            // The icon name, palette writes and colour queries are parsed
            // but not acted on.
            code::ICON => {}
            _ => {}
        }
    }
}

fn path_from_url(url: &str) -> Option<String> {
    let rest = url.strip_prefix("file://")?;
    let path = &rest[rest.find('/')?..];
    Some(percent_decode(path))
}

fn percent_decode(path: &str) -> String {
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = core::str::from_utf8(&bytes[i + 1..i + 3]).ok();
            if let Some(byte) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
