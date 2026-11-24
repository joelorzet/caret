#[allow(unused_imports)]
use crate::prelude::*;

use std::fs::File;
use std::io::{BufWriter, Write};

/// A one-frame capture of the window, for working on the chrome.
///
/// Screenshotting the real window depends on it being frontmost and on the
/// recording permission; writing the buffer we just painted does not, and it
/// is exactly the pixels that shipped.
pub struct Snapshot {
    pub path: String,
    /// Key bindings performed before the capture, so a layout with several
    /// tabs or panes can be photographed without driving the window by hand.
    pub keys: Vec<KeyBinding>,
    /// Text typed into the focused pane once the window has settled, so a
    /// command line can be photographed as the shell echoed it.
    pub typing: Option<String>,
    /// How long to let the window run first.
    ///
    /// A program a key just started has not drawn anything yet, so capturing
    /// the next frame photographs an empty panel rather than the agent.
    pub after: std::time::Duration,
}

impl Snapshot {
    /// `CARET_SNAPSHOT=/tmp/shot.bmp CARET_SNAPSHOT_KEYS='ctrl+b c,ctrl+b "'`,
    /// optionally `CARET_SNAPSHOT_DELAY=3000` milliseconds.
    pub fn from_env() -> Option<Snapshot> {
        let path = std::env::var("CARET_SNAPSHOT").ok()?;
        let keys = std::env::var("CARET_SNAPSHOT_KEYS")
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|spec| !spec.is_empty())
            .filter_map(|spec| KeyBinding::parse(spec).ok())
            .collect();
        let after = std::env::var("CARET_SNAPSHOT_DELAY")
            .ok()
            .and_then(|ms| ms.trim().parse().ok())
            .map(std::time::Duration::from_millis)
            .unwrap_or_default();
        let typing = std::env::var("CARET_SNAPSHOT_TYPE")
            .ok()
            .filter(|t| !t.is_empty());
        Some(Snapshot {
            path,
            keys,
            typing,
            after,
        })
    }

    pub fn write(&self, pixels: &[u32], width: usize, height: usize) -> std::io::Result<()> {
        let file = File::create(&self.path)?;
        let mut out = BufWriter::new(file);

        // BMP rather than PNG: no compression to implement, and every image
        // tool on the platform reads it.
        let row_padding = (4 - (width * 3) % 4) % 4;
        let data = (width * 3 + row_padding) * height;
        let size = 54 + data;

        out.write_all(b"BM")?;
        out.write_all(&(size as u32).to_le_bytes())?;
        out.write_all(&[0; 4])?;
        out.write_all(&54u32.to_le_bytes())?;
        out.write_all(&40u32.to_le_bytes())?;
        out.write_all(&(width as i32).to_le_bytes())?;
        // Negative height writes the rows top down, as they are in memory.
        out.write_all(&(-(height as i32)).to_le_bytes())?;
        out.write_all(&1u16.to_le_bytes())?;
        out.write_all(&24u16.to_le_bytes())?;
        out.write_all(&[0; 24])?;

        let padding = vec![0u8; row_padding];
        for row in 0..height {
            for col in 0..width {
                let pixel = pixels[row * width + col];
                out.write_all(&[pixel as u8, (pixel >> 8) as u8, (pixel >> 16) as u8])?;
            }
            out.write_all(&padding)?;
        }
        out.flush()
    }
}
