#[allow(unused_imports)]
use crate::prelude::*;

/// Horizon dark, warm coral and teal.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "horizon-dark",
    description: "Horizon dark, warm coral and teal.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x1c, 0x1e, 0x26),
    fg: Rgb::new(0xe0, 0xe0, 0xe0),
    cursor: Rgb::new(0xe9, 0x56, 0x78),
    selection_bg: Rgb::new(0x2e, 0x30, 0x3e),
    accent: Rgb::new(0xe9, 0x56, 0x78),
    normal: [
        Rgb::new(0x16, 0x16, 0x1c), //  0  black          #16161c
        Rgb::new(0xe9, 0x56, 0x78), //  1  red            #e95678
        Rgb::new(0x29, 0xd3, 0x98), //  2  green          #29d398
        Rgb::new(0xfa, 0xb7, 0x95), //  3  yellow         #fab795
        Rgb::new(0x26, 0xbb, 0xd9), //  4  blue           #26bbd9
        Rgb::new(0xee, 0x64, 0xac), //  5  magenta        #ee64ac
        Rgb::new(0x59, 0xe1, 0xe3), //  6  cyan           #59e1e3
        Rgb::new(0xd5, 0xd8, 0xda), //  7  white          #d5d8da
    ],
    bright: [
        Rgb::new(0x5b, 0x58, 0x58), //  8  bright black   #5b5858
        Rgb::new(0xec, 0x6a, 0x88), //  9  bright red     #ec6a88
        Rgb::new(0x3f, 0xda, 0xa4), // 10  bright green   #3fdaa4
        Rgb::new(0xfb, 0xc3, 0xa7), // 11  bright yellow  #fbc3a7
        Rgb::new(0x3f, 0xc4, 0xde), // 12  bright blue    #3fc4de
        Rgb::new(0xf0, 0x75, 0xb5), // 13  bright magenta #f075b5
        Rgb::new(0x6b, 0xe4, 0xe6), // 14  bright cyan    #6be4e6
        Rgb::new(0xd5, 0xd8, 0xda), // 15  bright white   #d5d8da
    ],
};
