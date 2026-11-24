#[allow(unused_imports)]
use crate::prelude::*;

/// Catppuccin, Latte flavour, the light one.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "catppuccin-latte",
    description: "Catppuccin, Latte flavour, the light one.",
    appearance: Appearance::Light,
    bg: Rgb::new(0xef, 0xf1, 0xf5),
    fg: Rgb::new(0x4c, 0x4f, 0x69),
    cursor: Rgb::new(0xdc, 0x8a, 0x78),
    selection_bg: Rgb::new(0xdc, 0x8a, 0x78),
    accent: Rgb::new(0x1e, 0x66, 0xf5),
    normal: [
        Rgb::new(0x5c, 0x5f, 0x77), //  0  black          #5c5f77
        Rgb::new(0xd2, 0x0f, 0x39), //  1  red            #d20f39
        Rgb::new(0x40, 0xa0, 0x2b), //  2  green          #40a02b
        Rgb::new(0xdf, 0x8e, 0x1d), //  3  yellow         #df8e1d
        Rgb::new(0x1e, 0x66, 0xf5), //  4  blue           #1e66f5
        Rgb::new(0xea, 0x76, 0xcb), //  5  magenta        #ea76cb
        Rgb::new(0x17, 0x92, 0x99), //  6  cyan           #179299
        Rgb::new(0xac, 0xb0, 0xbe), //  7  white          #acb0be
    ],
    bright: [
        Rgb::new(0x6c, 0x6f, 0x85), //  8  bright black   #6c6f85
        Rgb::new(0xd2, 0x0f, 0x39), //  9  bright red     #d20f39
        Rgb::new(0x40, 0xa0, 0x2b), // 10  bright green   #40a02b
        Rgb::new(0xdf, 0x8e, 0x1d), // 11  bright yellow  #df8e1d
        Rgb::new(0x1e, 0x66, 0xf5), // 12  bright blue    #1e66f5
        Rgb::new(0xea, 0x76, 0xcb), // 13  bright magenta #ea76cb
        Rgb::new(0x17, 0x92, 0x99), // 14  bright cyan    #179299
        Rgb::new(0xbc, 0xc0, 0xcc), // 15  bright white   #bcc0cc
    ],
};
