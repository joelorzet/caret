#[allow(unused_imports)]
use crate::prelude::*;

/// Everforest light, medium.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "everforest-light",
    description: "Everforest light, medium.",
    appearance: Appearance::Light,
    bg: Rgb::new(0xfd, 0xf6, 0xe3),
    fg: Rgb::new(0x5c, 0x6a, 0x72),
    cursor: Rgb::new(0x5c, 0x6a, 0x72),
    selection_bg: Rgb::new(0xea, 0xed, 0xc8),
    accent: Rgb::new(0x8d, 0xa1, 0x01),
    normal: [
        Rgb::new(0x5c, 0x6a, 0x72), //  0  black          #5c6a72
        Rgb::new(0xf8, 0x55, 0x52), //  1  red            #f85552
        Rgb::new(0x8d, 0xa1, 0x01), //  2  green          #8da101
        Rgb::new(0xdf, 0xa0, 0x00), //  3  yellow         #dfa000
        Rgb::new(0x3a, 0x94, 0xc5), //  4  blue           #3a94c5
        Rgb::new(0xdf, 0x69, 0xba), //  5  magenta        #df69ba
        Rgb::new(0x35, 0xa7, 0x7c), //  6  cyan           #35a77c
        Rgb::new(0xe0, 0xdc, 0xc7), //  7  white          #e0dcc7
    ],
    bright: [
        Rgb::new(0x82, 0x91, 0x81), //  8  bright black   #829181
        Rgb::new(0xf8, 0x55, 0x52), //  9  bright red     #f85552
        Rgb::new(0x8d, 0xa1, 0x01), // 10  bright green   #8da101
        Rgb::new(0xdf, 0xa0, 0x00), // 11  bright yellow  #dfa000
        Rgb::new(0x3a, 0x94, 0xc5), // 12  bright blue    #3a94c5
        Rgb::new(0xdf, 0x69, 0xba), // 13  bright magenta #df69ba
        Rgb::new(0x35, 0xa7, 0x7c), // 14  bright cyan    #35a77c
        Rgb::new(0xef, 0xeb, 0xd4), // 15  bright white   #efebd4
    ],
};
