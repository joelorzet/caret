#[allow(unused_imports)]
use crate::prelude::*;

/// Ayu dark, very dark, high contrast.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "ayu-dark",
    description: "Ayu dark, very dark, high contrast.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x0a, 0x0e, 0x14),
    fg: Rgb::new(0xb3, 0xb1, 0xad),
    cursor: Rgb::new(0xff, 0xcc, 0x66),
    selection_bg: Rgb::new(0x1f, 0x24, 0x30),
    accent: Rgb::new(0x59, 0xc2, 0xff),
    normal: [
        Rgb::new(0x01, 0x06, 0x0e), //  0  black          #01060e
        Rgb::new(0xea, 0x6c, 0x73), //  1  red            #ea6c73
        Rgb::new(0x91, 0xb3, 0x62), //  2  green          #91b362
        Rgb::new(0xf9, 0xaf, 0x4f), //  3  yellow         #f9af4f
        Rgb::new(0x53, 0xbd, 0xfa), //  4  blue           #53bdfa
        Rgb::new(0xfa, 0xe9, 0x94), //  5  magenta        #fae994
        Rgb::new(0x90, 0xe1, 0xc6), //  6  cyan           #90e1c6
        Rgb::new(0xc7, 0xc7, 0xc7), //  7  white          #c7c7c7
    ],
    bright: [
        Rgb::new(0x68, 0x68, 0x68), //  8  bright black   #686868
        Rgb::new(0xf0, 0x71, 0x78), //  9  bright red     #f07178
        Rgb::new(0xc2, 0xd9, 0x4c), // 10  bright green   #c2d94c
        Rgb::new(0xff, 0xb4, 0x54), // 11  bright yellow  #ffb454
        Rgb::new(0x59, 0xc2, 0xff), // 12  bright blue    #59c2ff
        Rgb::new(0xff, 0xee, 0x99), // 13  bright magenta #ffee99
        Rgb::new(0x95, 0xe6, 0xcb), // 14  bright cyan    #95e6cb
        Rgb::new(0xff, 0xff, 0xff), // 15  bright white   #ffffff
    ],
};
