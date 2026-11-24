#[allow(unused_imports)]
use crate::prelude::*;

/// Past this, a paste is worth showing whatever else it holds: nobody
/// inspects a screenful by eye, but they should be told how much it is.
const LARGE: usize = 2_000;

/// What the names of the control characters are, where they have one.
fn control_name(ch: char) -> Option<&'static str> {
    Some(match ch {
        '\x1b' => "escape",
        '\x07' => "bell",
        '\x08' => "backspace",
        '\x0c' => "form feed",
        '\r' => "carriage return",
        '\0' => "null",
        c if c.is_control() => "control",
        _ => return None,
    })
}

/// Reads a paste without acting on it.
pub fn review_paste(text: &str) -> PasteReview {
    let mut controls: Vec<&'static str> = Vec::new();
    for ch in text.chars() {
        if ch == '\n' || ch == '\t' {
            continue;
        }
        if let Some(name) = control_name(ch) {
            if !controls.contains(&name) {
                controls.push(name);
            }
        }
    }

    PasteReview {
        // A trailing newline ends the last line rather than starting
        // another, which is why this counts breaks and not pieces.
        lines: text.lines().count().max(1),
        chars: text.chars().count(),
        runs_on_arrival: text.ends_with('\n') || text.ends_with('\r'),
        controls,
    }
}

impl PasteReview {
    /// Whether this is worth stopping for when only multi-line pastes are.
    ///
    /// One line of plain text is the ordinary case and interrupting it
    /// would be noise. Anything that carries a line break, a control
    /// character, or more text than anybody reads is not ordinary.
    pub fn notable(&self) -> bool {
        self.lines > 1 || self.runs_on_arrival || !self.controls.is_empty() || self.chars > LARGE
    }
}
