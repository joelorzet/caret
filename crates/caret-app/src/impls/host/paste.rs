#[allow(unused_imports)]
use crate::prelude::*;

use caret_config::PasteConfirm;
use caret_features::review_paste;

/// Room for the two bracketed-paste markers, so the buffer is sized once.
const PASTE_MARKERS: usize = 12;

impl Session {
    /// Pasting runs what was pasted, so a paste that carries more than one
    /// line is shown before it is sent.
    pub(crate) fn on_paste(&mut self, text: Vec<u8>) {
        let target = match self.focus {
            FocusTarget::Agent => self.agent,
            _ => Some(self.workspace.active_pane()),
        };
        let Some(id) = target else { return };

        let body = String::from_utf8_lossy(&text).into_owned();
        let review = review_paste(&body);
        let ask = match self.config.terminal.confirm_paste {
            PasteConfirm::Always => true,
            PasteConfirm::Notable => review.notable(),
            PasteConfirm::Never => false,
        };

        if ask {
            self.pending_paste = Some(PendingPaste {
                target: id,
                text: body,
                review,
            });
            self.needs_draw = true;
            return;
        }
        self.send_paste(id, &text);
    }

    pub(crate) fn is_confirming_paste(&self) -> bool {
        self.pending_paste.is_some()
    }

    /// Enter sends it, escape drops it, and nothing else reaches the shell
    /// while the question is on screen.
    pub(crate) fn paste_key(&mut self, press: &KeyPress) {
        self.needs_draw = true;
        match press.binding.key {
            Key::Enter => {
                let Some(pending) = self.pending_paste.take() else {
                    return;
                };
                let bytes = pending.text.into_bytes();
                self.send_paste(pending.target, &bytes);
            }
            Key::Escape => {
                self.pending_paste = None;
                self.message = " paste dropped".into();
            }
            _ => {}
        }
    }

    fn send_paste(&mut self, id: PaneId, text: &[u8]) {
        let bracketed = self
            .pane(id)
            .is_some_and(|p| p.terminal.modes().contains(Modes::BRACKETED_PASTE));

        // A lone newline is a carriage return to a terminal, and a pair is
        // one press of enter rather than two.
        let mut body: Vec<u8> = Vec::with_capacity(text.len() + PASTE_MARKERS);
        let mut previous = 0u8;
        for byte in text.iter().copied() {
            match byte {
                b'\n' if previous == b'\r' => {}
                b'\n' => body.push(b'\r'),
                other => body.push(other),
            }
            previous = byte;
        }

        let mut out = Vec::with_capacity(body.len() + PASTE_MARKERS);
        if bracketed {
            out.extend_from_slice(b"\x1b[200~");
        }
        out.extend_from_slice(&body);
        if bracketed {
            out.extend_from_slice(b"\x1b[201~");
        }
        let _ = self.ptys.write(PtyId(id.0), &out);
        self.needs_draw = true;
    }
}
