#[allow(unused_imports)]
use crate::prelude::*;

/// Everforest dark, medium, green and easy on the eyes.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "everforest-dark",
    description: "Everforest dark, medium, green and easy on the eyes.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x2d, 0x35, 0x3b),
    fg: Rgb::new(0xd3, 0xc6, 0xaa),
    cursor: Rgb::new(0xd3, 0xc6, 0xaa),
    selection_bg: Rgb::new(0x54, 0x3a, 0x48),
    accent: Rgb::new(0xa7, 0xc0, 0x80),
    normal: [
        Rgb::new(0x47, 0x52, 0x58), //  0  black          #475258
        Rgb::new(0xe6, 0x7e, 0x80), //  1  red            #e67e80
        Rgb::new(0xa7, 0xc0, 0x80), //  2  green          #a7c080
        Rgb::new(0xdb, 0xbc, 0x7f), //  3  yellow         #dbbc7f
        Rgb::new(0x7f, 0xbb, 0xb3), //  4  blue           #7fbbb3
        Rgb::new(0xd6, 0x99, 0xb6), //  5  magenta        #d699b6
        Rgb::new(0x83, 0xc0, 0x92), //  6  cyan           #83c092
        Rgb::new(0xd3, 0xc6, 0xaa), //  7  white          #d3c6aa
    ],
    bright: [
        Rgb::new(0x5c, 0x6a, 0x72), //  8  bright black   #5c6a72
        Rgb::new(0xe6, 0x7e, 0x80), //  9  bright red     #e67e80
        Rgb::new(0xa7, 0xc0, 0x80), // 10  bright green   #a7c080
        Rgb::new(0xdb, 0xbc, 0x7f), // 11  bright yellow  #dbbc7f
        Rgb::new(0x7f, 0xbb, 0xb3), // 12  bright blue    #7fbbb3
        Rgb::new(0xd6, 0x99, 0xb6), // 13  bright magenta #d699b6
        Rgb::new(0x83, 0xc0, 0x92), // 14  bright cyan    #83c092
        Rgb::new(0xe6, 0xe2, 0xcc), // 15  bright white   #e6e2cc
    ],
};
