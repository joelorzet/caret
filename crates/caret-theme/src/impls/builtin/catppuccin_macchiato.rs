#[allow(unused_imports)]
use crate::prelude::*;

/// Catppuccin, Macchiato flavour.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "catppuccin-macchiato",
    description: "Catppuccin, Macchiato flavour.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x24, 0x27, 0x3a),
    fg: Rgb::new(0xca, 0xd3, 0xf5),
    cursor: Rgb::new(0xf4, 0xdb, 0xd6),
    selection_bg: Rgb::new(0x5b, 0x60, 0x78),
    accent: Rgb::new(0x8a, 0xad, 0xf4),
    normal: [
        Rgb::new(0x49, 0x4d, 0x64), //  0  black          #494d64
        Rgb::new(0xed, 0x87, 0x96), //  1  red            #ed8796
        Rgb::new(0xa6, 0xda, 0x95), //  2  green          #a6da95
        Rgb::new(0xee, 0xd4, 0x9f), //  3  yellow         #eed49f
        Rgb::new(0x8a, 0xad, 0xf4), //  4  blue           #8aadf4
        Rgb::new(0xf5, 0xbd, 0xe6), //  5  magenta        #f5bde6
        Rgb::new(0x8b, 0xd5, 0xca), //  6  cyan           #8bd5ca
        Rgb::new(0xb8, 0xc0, 0xe0), //  7  white          #b8c0e0
    ],
    bright: [
        Rgb::new(0x5b, 0x60, 0x78), //  8  bright black   #5b6078
        Rgb::new(0xed, 0x87, 0x96), //  9  bright red     #ed8796
        Rgb::new(0xa6, 0xda, 0x95), // 10  bright green   #a6da95
        Rgb::new(0xee, 0xd4, 0x9f), // 11  bright yellow  #eed49f
        Rgb::new(0x8a, 0xad, 0xf4), // 12  bright blue    #8aadf4
        Rgb::new(0xf5, 0xbd, 0xe6), // 13  bright magenta #f5bde6
        Rgb::new(0x8b, 0xd5, 0xca), // 14  bright cyan    #8bd5ca
        Rgb::new(0xa5, 0xad, 0xcb), // 15  bright white   #a5adcb
    ],
};
