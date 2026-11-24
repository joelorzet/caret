#[allow(unused_imports)]
use crate::prelude::*;

/// Solarized dark, keeping Ethan Schoonover's unusual ANSI mapping.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "solarized-dark",
    description: "Solarized dark, keeping Ethan Schoonover's unusual ANSI mapping.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x00, 0x2b, 0x36),
    fg: Rgb::new(0x83, 0x94, 0x96),
    cursor: Rgb::new(0x93, 0xa1, 0xa1),
    selection_bg: Rgb::new(0x07, 0x36, 0x42),
    accent: Rgb::new(0x26, 0x8b, 0xd2),
    normal: [
        Rgb::new(0x07, 0x36, 0x42), //  0  black          #073642
        Rgb::new(0xdc, 0x32, 0x2f), //  1  red            #dc322f
        Rgb::new(0x85, 0x99, 0x00), //  2  green          #859900
        Rgb::new(0xb5, 0x89, 0x00), //  3  yellow         #b58900
        Rgb::new(0x26, 0x8b, 0xd2), //  4  blue           #268bd2
        Rgb::new(0xd3, 0x36, 0x82), //  5  magenta        #d33682
        Rgb::new(0x2a, 0xa1, 0x98), //  6  cyan           #2aa198
        Rgb::new(0xee, 0xe8, 0xd5), //  7  white          #eee8d5
    ],
    bright: [
        Rgb::new(0x00, 0x2b, 0x36), //  8  bright black   #002b36
        Rgb::new(0xcb, 0x4b, 0x16), //  9  bright red     #cb4b16
        Rgb::new(0x58, 0x6e, 0x75), // 10  bright green   #586e75
        Rgb::new(0x65, 0x7b, 0x83), // 11  bright yellow  #657b83
        Rgb::new(0x83, 0x94, 0x96), // 12  bright blue    #839496
        Rgb::new(0x6c, 0x71, 0xc4), // 13  bright magenta #6c71c4
        Rgb::new(0x93, 0xa1, 0xa1), // 14  bright cyan    #93a1a1
        Rgb::new(0xfd, 0xf6, 0xe3), // 15  bright white   #fdf6e3
    ],
};
