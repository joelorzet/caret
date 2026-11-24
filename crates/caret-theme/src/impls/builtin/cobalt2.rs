#[allow(unused_imports)]
use crate::prelude::*;

/// Cobalt2, Wes Bos's blue and yellow.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "cobalt2",
    description: "Cobalt2, Wes Bos's blue and yellow.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x12, 0x26, 0x37),
    fg: Rgb::new(0xff, 0xff, 0xff),
    cursor: Rgb::new(0xf0, 0xcb, 0x09),
    selection_bg: Rgb::new(0x00, 0x50, 0xa4),
    accent: Rgb::new(0xff, 0xc6, 0x00),
    normal: [
        Rgb::new(0x00, 0x00, 0x00), //  0  black          #000000
        Rgb::new(0xff, 0x00, 0x00), //  1  red            #ff0000
        Rgb::new(0x37, 0xdd, 0x21), //  2  green          #37dd21
        Rgb::new(0xfe, 0xe4, 0x09), //  3  yellow         #fee409
        Rgb::new(0x14, 0x60, 0xd2), //  4  blue           #1460d2
        Rgb::new(0xff, 0x00, 0x5d), //  5  magenta        #ff005d
        Rgb::new(0x00, 0xbb, 0xbb), //  6  cyan           #00bbbb
        Rgb::new(0xbb, 0xbb, 0xbb), //  7  white          #bbbbbb
    ],
    bright: [
        Rgb::new(0x54, 0x54, 0x54), //  8  bright black   #545454
        Rgb::new(0xf4, 0x0d, 0x17), //  9  bright red     #f40d17
        Rgb::new(0x3b, 0xcf, 0x1d), // 10  bright green   #3bcf1d
        Rgb::new(0xec, 0xc8, 0x09), // 11  bright yellow  #ecc809
        Rgb::new(0x55, 0x55, 0xff), // 12  bright blue    #5555ff
        Rgb::new(0xff, 0x55, 0xff), // 13  bright magenta #ff55ff
        Rgb::new(0x6a, 0xe3, 0xf9), // 14  bright cyan    #6ae3f9
        Rgb::new(0xff, 0xff, 0xff), // 15  bright white   #ffffff
    ],
};
