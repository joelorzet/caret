#[allow(unused_imports)]
use crate::prelude::*;

/// OSC command numbers, as they arrive on the wire.
mod code {
    pub const ICON_AND_TITLE: &[u8] = b"0";
    pub const ICON: &[u8] = b"1";
    pub const TITLE: &[u8] = b"2";
    pub const WORKING_DIRECTORY: &[u8] = b"7";
    pub const SHELL_INTEGRATION: &[u8] = b"133";
}

/// Shell-integration marks within OSC 133.
mod mark {
    pub const PROMPT_START: &[u8] = b"A";
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
            code::SHELL_INTEGRATION if params.get(1) == Some(&mark::PROMPT_START) => {
                self.term.push_event(TerminalEvent::PromptMarked);
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
    let path = match rest.find('/') {
        Some(i) => &rest[i..],
        None => return None,
    };
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
