#[allow(unused_imports)]
use crate::prelude::*;

/// Solarized light with the base lightened and the accents deepened.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "solarized-light-contrast",
    description: "Solarized light with the base lightened and the accents deepened.",
    appearance: Appearance::Light,
    bg: Rgb::new(0xfd, 0xfb, 0xf7),
    fg: Rgb::new(0x29, 0x35, 0x39),
    cursor: Rgb::new(0x31, 0x41, 0x46),
    selection_bg: Rgb::new(0xf3, 0xef, 0xe2),
    accent: Rgb::new(0x0c, 0x65, 0xa3),
    normal: [
        Rgb::new(0x00, 0x00, 0x00), //  0  black          #000000
        Rgb::new(0xbc, 0x0e, 0x0b), //  1  red            #bc0e0b
        Rgb::new(0x2e, 0x35, 0x00), //  2  green          #2e3500
        Rgb::new(0x59, 0x43, 0x00), //  3  yellow         #594300
        Rgb::new(0x0c, 0x65, 0xa3), //  4  blue           #0c65a3
        Rgb::new(0xb1, 0x14, 0x60), //  5  magenta        #b11460
        Rgb::new(0x10, 0x65, 0x5f), //  6  cyan           #10655f
        Rgb::new(0xed, 0xe4, 0xc5), //  7  white          #ede4c5
    ],
    bright: [
        Rgb::new(0x00, 0x00, 0x00), //  8  bright black   #000000
        Rgb::new(0x91, 0x2b, 0x00), //  9  bright red     #912b00
        Rgb::new(0x31, 0x41, 0x46), // 10  bright green   #314146
        Rgb::new(0x41, 0x53, 0x5a), // 11  bright yellow  #41535a
        Rgb::new(0x61, 0x75, 0x78), // 12  bright blue    #617578
        Rgb::new(0x39, 0x41, 0xbd), // 13  bright magenta #3941bd
        Rgb::new(0x73, 0x89, 0x89), // 14  bright cyan    #738989
        Rgb::new(0xff, 0xf5, 0xd9), // 15  bright white   #fff5d9
    ],
};
