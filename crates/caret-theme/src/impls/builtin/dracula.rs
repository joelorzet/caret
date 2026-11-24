#[allow(unused_imports)]
use crate::prelude::*;

/// Dracula, as published by the project.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "dracula",
    description: "Dracula, as published by the project.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x28, 0x2a, 0x36),
    fg: Rgb::new(0xf8, 0xf8, 0xf2),
    cursor: Rgb::new(0xf8, 0xf8, 0xf2),
    selection_bg: Rgb::new(0x44, 0x47, 0x5a),
    accent: Rgb::new(0xbd, 0x93, 0xf9),
    normal: [
        Rgb::new(0x21, 0x22, 0x2c), //  0  black          #21222c
        Rgb::new(0xff, 0x55, 0x55), //  1  red            #ff5555
        Rgb::new(0x50, 0xfa, 0x7b), //  2  green          #50fa7b
        Rgb::new(0xf1, 0xfa, 0x8c), //  3  yellow         #f1fa8c
        Rgb::new(0xbd, 0x93, 0xf9), //  4  blue           #bd93f9
        Rgb::new(0xff, 0x79, 0xc6), //  5  magenta        #ff79c6
        Rgb::new(0x8b, 0xe9, 0xfd), //  6  cyan           #8be9fd
        Rgb::new(0xf8, 0xf8, 0xf2), //  7  white          #f8f8f2
    ],
    bright: [
        Rgb::new(0x62, 0x72, 0xa4), //  8  bright black   #6272a4
        Rgb::new(0xff, 0x6e, 0x6e), //  9  bright red     #ff6e6e
        Rgb::new(0x69, 0xff, 0x94), // 10  bright green   #69ff94
        Rgb::new(0xff, 0xff, 0xa5), // 11  bright yellow  #ffffa5
        Rgb::new(0xd6, 0xac, 0xff), // 12  bright blue    #d6acff
        Rgb::new(0xff, 0x92, 0xdf), // 13  bright magenta #ff92df
        Rgb::new(0xa4, 0xff, 0xff), // 14  bright cyan    #a4ffff
        Rgb::new(0xff, 0xff, 0xff), // 15  bright white   #ffffff
    ],
};
