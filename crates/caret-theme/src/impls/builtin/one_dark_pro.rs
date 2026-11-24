#[allow(unused_imports)]
use crate::prelude::*;

/// Atom's One Dark, in the Pro variant's terminal colors.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "one-dark-pro",
    description: "Atom's One Dark, in the Pro variant's terminal colors.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x28, 0x2c, 0x34),
    fg: Rgb::new(0xab, 0xb2, 0xbf),
    cursor: Rgb::new(0x52, 0x8b, 0xff),
    selection_bg: Rgb::new(0x3e, 0x44, 0x51),
    accent: Rgb::new(0x61, 0xaf, 0xef),
    normal: [
        Rgb::new(0x1e, 0x21, 0x27), //  0  black          #1e2127
        Rgb::new(0xe0, 0x6c, 0x75), //  1  red            #e06c75
        Rgb::new(0x98, 0xc3, 0x79), //  2  green          #98c379
        Rgb::new(0xe5, 0xc0, 0x7b), //  3  yellow         #e5c07b
        Rgb::new(0x61, 0xaf, 0xef), //  4  blue           #61afef
        Rgb::new(0xc6, 0x78, 0xdd), //  5  magenta        #c678dd
        Rgb::new(0x56, 0xb6, 0xc2), //  6  cyan           #56b6c2
        Rgb::new(0xab, 0xb2, 0xbf), //  7  white          #abb2bf
    ],
    bright: [
        Rgb::new(0x5c, 0x63, 0x70), //  8  bright black   #5c6370
        Rgb::new(0xe0, 0x6c, 0x75), //  9  bright red     #e06c75
        Rgb::new(0x98, 0xc3, 0x79), // 10  bright green   #98c379
        Rgb::new(0xe5, 0xc0, 0x7b), // 11  bright yellow  #e5c07b
        Rgb::new(0x61, 0xaf, 0xef), // 12  bright blue    #61afef
        Rgb::new(0xc6, 0x78, 0xdd), // 13  bright magenta #c678dd
        Rgb::new(0x56, 0xb6, 0xc2), // 14  bright cyan    #56b6c2
        Rgb::new(0xff, 0xff, 0xff), // 15  bright white   #ffffff
    ],
};
