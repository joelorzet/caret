#[allow(unused_imports)]
use crate::prelude::*;

/// Kanagawa Dragon, the darker, muted sibling.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "kanagawa-dragon",
    description: "Kanagawa Dragon, the darker, muted sibling.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x18, 0x16, 0x16),
    fg: Rgb::new(0xc5, 0xc9, 0xc5),
    cursor: Rgb::new(0xc5, 0xc9, 0xc5),
    selection_bg: Rgb::new(0x2d, 0x4f, 0x67),
    accent: Rgb::new(0x8b, 0xa4, 0xb0),
    normal: [
        Rgb::new(0x0d, 0x0c, 0x0c), //  0  black          #0d0c0c
        Rgb::new(0xc4, 0x74, 0x6e), //  1  red            #c4746e
        Rgb::new(0x8a, 0x9a, 0x7b), //  2  green          #8a9a7b
        Rgb::new(0xc4, 0xb2, 0x8a), //  3  yellow         #c4b28a
        Rgb::new(0x8b, 0xa4, 0xb0), //  4  blue           #8ba4b0
        Rgb::new(0xa2, 0x92, 0xa3), //  5  magenta        #a292a3
        Rgb::new(0x8e, 0xa4, 0xa2), //  6  cyan           #8ea4a2
        Rgb::new(0xc8, 0xc0, 0x93), //  7  white          #c8c093
    ],
    bright: [
        Rgb::new(0xa6, 0xa6, 0x9c), //  8  bright black   #a6a69c
        Rgb::new(0xe4, 0x68, 0x76), //  9  bright red     #e46876
        Rgb::new(0x87, 0xa9, 0x87), // 10  bright green   #87a987
        Rgb::new(0xe6, 0xc3, 0x84), // 11  bright yellow  #e6c384
        Rgb::new(0x7f, 0xb4, 0xca), // 12  bright blue    #7fb4ca
        Rgb::new(0x93, 0x8a, 0xa9), // 13  bright magenta #938aa9
        Rgb::new(0x7a, 0xa8, 0x9f), // 14  bright cyan    #7aa89f
        Rgb::new(0xc5, 0xc9, 0xc5), // 15  bright white   #c5c9c5
    ],
};
