#[allow(unused_imports)]
use crate::prelude::*;

/// Latte with more milk, and the pastels taken down to read on it.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "catppuccin-latte-contrast",
    description: "Latte with more milk, and the pastels taken down to read on it.",
    appearance: Appearance::Light,
    bg: Rgb::new(0xfa, 0xfa, 0xfb),
    fg: Rgb::new(0x1b, 0x1c, 0x27),
    cursor: Rgb::new(0xe2, 0x5f, 0x42),
    selection_bg: Rgb::new(0xe6, 0xad, 0xa1),
    accent: Rgb::new(0x00, 0x46, 0xd1),
    normal: [
        Rgb::new(0x35, 0x38, 0x4a), //  0  black          #35384a
        Rgb::new(0x91, 0x00, 0x1f), //  1  red            #91001f
        Rgb::new(0x1f, 0x65, 0x10), //  2  green          #1f6510
        Rgb::new(0xb0, 0x68, 0x03), //  3  yellow         #b06803
        Rgb::new(0x00, 0x46, 0xd1), //  4  blue           #0046d1
        Rgb::new(0xf7, 0x3d, 0xc5), //  5  magenta        #f73dc5
        Rgb::new(0x03, 0x4b, 0x4f), //  6  cyan           #034b4f
        Rgb::new(0x92, 0x98, 0xaf), //  7  white          #9298af
    ],
    bright: [
        Rgb::new(0x48, 0x4b, 0x5e), //  8  bright black   #484b5e
        Rgb::new(0x91, 0x00, 0x1f), //  9  bright red     #91001f
        Rgb::new(0x1f, 0x65, 0x10), // 10  bright green   #1f6510
        Rgb::new(0xb0, 0x68, 0x03), // 11  bright yellow  #b06803
        Rgb::new(0x00, 0x46, 0xd1), // 12  bright blue    #0046d1
        Rgb::new(0xf7, 0x3d, 0xc5), // 13  bright magenta #f73dc5
        Rgb::new(0x03, 0x4b, 0x4f), // 14  bright cyan    #034b4f
        Rgb::new(0xa7, 0xad, 0xc0), // 15  bright white   #a7adc0
    ],
};
