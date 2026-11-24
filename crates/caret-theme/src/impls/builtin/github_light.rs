#[allow(unused_imports)]
use crate::prelude::*;

/// GitHub light, the default variant.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "github-light",
    description: "GitHub light, the default variant.",
    appearance: Appearance::Light,
    bg: Rgb::new(0xff, 0xff, 0xff),
    fg: Rgb::new(0x0e, 0x11, 0x16),
    cursor: Rgb::new(0x09, 0x69, 0xda),
    selection_bg: Rgb::new(0xbc, 0xd6, 0xf5),
    accent: Rgb::new(0x09, 0x69, 0xda),
    normal: [
        Rgb::new(0x24, 0x29, 0x2f), //  0  black          #24292f
        Rgb::new(0xcf, 0x22, 0x2e), //  1  red            #cf222e
        Rgb::new(0x11, 0x63, 0x29), //  2  green          #116329
        Rgb::new(0x4d, 0x2d, 0x00), //  3  yellow         #4d2d00
        Rgb::new(0x09, 0x69, 0xda), //  4  blue           #0969da
        Rgb::new(0x82, 0x50, 0xdf), //  5  magenta        #8250df
        Rgb::new(0x1b, 0x7c, 0x83), //  6  cyan           #1b7c83
        Rgb::new(0x6e, 0x77, 0x81), //  7  white          #6e7781
    ],
    bright: [
        Rgb::new(0x57, 0x60, 0x6a), //  8  bright black   #57606a
        Rgb::new(0xa4, 0x0e, 0x26), //  9  bright red     #a40e26
        Rgb::new(0x1a, 0x7f, 0x37), // 10  bright green   #1a7f37
        Rgb::new(0x63, 0x3c, 0x01), // 11  bright yellow  #633c01
        Rgb::new(0x21, 0x8b, 0xff), // 12  bright blue    #218bff
        Rgb::new(0xa4, 0x75, 0xf9), // 13  bright magenta #a475f9
        Rgb::new(0x31, 0x92, 0xaa), // 14  bright cyan    #3192aa
        Rgb::new(0x8c, 0x95, 0x9f), // 15  bright white   #8c959f
    ],
};
