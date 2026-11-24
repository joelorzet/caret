#[allow(unused_imports)]
use crate::prelude::*;

/// Gruvbox light, medium contrast.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "gruvbox-light",
    description: "Gruvbox light, medium contrast.",
    appearance: Appearance::Light,
    bg: Rgb::new(0xfb, 0xf1, 0xc7),
    fg: Rgb::new(0x3c, 0x38, 0x36),
    cursor: Rgb::new(0x3c, 0x38, 0x36),
    selection_bg: Rgb::new(0xeb, 0xdb, 0xb2),
    accent: Rgb::new(0x07, 0x66, 0x78),
    normal: [
        Rgb::new(0xfb, 0xf1, 0xc7), //  0  black          #fbf1c7
        Rgb::new(0xcc, 0x24, 0x1d), //  1  red            #cc241d
        Rgb::new(0x98, 0x97, 0x1a), //  2  green          #98971a
        Rgb::new(0xd7, 0x99, 0x21), //  3  yellow         #d79921
        Rgb::new(0x45, 0x85, 0x88), //  4  blue           #458588
        Rgb::new(0xb1, 0x62, 0x86), //  5  magenta        #b16286
        Rgb::new(0x68, 0x9d, 0x6a), //  6  cyan           #689d6a
        Rgb::new(0x7c, 0x6f, 0x64), //  7  white          #7c6f64
    ],
    bright: [
        Rgb::new(0x92, 0x83, 0x74), //  8  bright black   #928374
        Rgb::new(0x9d, 0x00, 0x06), //  9  bright red     #9d0006
        Rgb::new(0x79, 0x74, 0x0e), // 10  bright green   #79740e
        Rgb::new(0xb5, 0x76, 0x14), // 11  bright yellow  #b57614
        Rgb::new(0x07, 0x66, 0x78), // 12  bright blue    #076678
        Rgb::new(0x8f, 0x3f, 0x71), // 13  bright magenta #8f3f71
        Rgb::new(0x42, 0x7b, 0x58), // 14  bright cyan    #427b58
        Rgb::new(0x3c, 0x38, 0x36), // 15  bright white   #3c3836
    ],
};
