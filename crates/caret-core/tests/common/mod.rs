// Each integration test binary compiles this separately, so some helpers are
// unused in some of them.
#![allow(dead_code)]

use caret_core::Screen;

pub fn write(s: &mut Screen, text: &str) {
    for ch in text.chars() {
        match ch {
            '\n' => s.line_feed(),
            '\r' => s.carriage_return(),
            _ => s.print(ch, 1, true, false),
        }
    }
}

pub fn viewport(s: &Screen) -> Vec<String> {
    let base = s.history_len();
    (0..s.rows()).map(|y| s.row_text(base + y as u32)).collect()
}

pub fn display(s: &Screen) -> Vec<String> {
    (0..s.rows())
        .map(|y| {
            s.display_line(y)
                .iter()
                .map(|c| c.ch)
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}
