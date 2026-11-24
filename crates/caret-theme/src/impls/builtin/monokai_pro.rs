#[allow(unused_imports)]
use crate::prelude::*;

/// Monokai Pro.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "monokai-pro",
    description: "Monokai Pro.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x2d, 0x2a, 0x2e),
    fg: Rgb::new(0xff, 0xf1, 0xf3),
    cursor: Rgb::new(0xff, 0xf1, 0xf3),
    selection_bg: Rgb::new(0x5b, 0x59, 0x5c),
    accent: Rgb::new(0xff, 0xd8, 0x66),
    normal: [
        Rgb::new(0x2c, 0x25, 0x25), //  0  black          #2c2525
        Rgb::new(0xfd, 0x68, 0x83), //  1  red            #fd6883
        Rgb::new(0xad, 0xda, 0x78), //  2  green          #adda78
        Rgb::new(0xf9, 0xcc, 0x6c), //  3  yellow         #f9cc6c
        Rgb::new(0xf3, 0x8d, 0x70), //  4  blue           #f38d70
        Rgb::new(0xa8, 0xa9, 0xeb), //  5  magenta        #a8a9eb
        Rgb::new(0x85, 0xda, 0xcc), //  6  cyan           #85dacc
        Rgb::new(0xff, 0xf1, 0xf3), //  7  white          #fff1f3
    ],
    bright: [
        Rgb::new(0x72, 0x69, 0x6a), //  8  bright black   #72696a
        Rgb::new(0xfd, 0x68, 0x83), //  9  bright red     #fd6883
        Rgb::new(0xad, 0xda, 0x78), // 10  bright green   #adda78
        Rgb::new(0xf9, 0xcc, 0x6c), // 11  bright yellow  #f9cc6c
        Rgb::new(0xf3, 0x8d, 0x70), // 12  bright blue    #f38d70
        Rgb::new(0xa8, 0xa9, 0xeb), // 13  bright magenta #a8a9eb
        Rgb::new(0x85, 0xda, 0xcc), // 14  bright cyan    #85dacc
        Rgb::new(0xff, 0xf1, 0xf3), // 15  bright white   #fff1f3
    ],
};
