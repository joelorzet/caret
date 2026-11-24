#[allow(unused_imports)]
use crate::prelude::*;

/// Tokyo Night.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "tokyo-night",
    description: "Tokyo Night.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x1a, 0x1b, 0x26),
    fg: Rgb::new(0xc0, 0xca, 0xf5),
    cursor: Rgb::new(0xc0, 0xca, 0xf5),
    selection_bg: Rgb::new(0x28, 0x34, 0x57),
    accent: Rgb::new(0x7a, 0xa2, 0xf7),
    normal: [
        Rgb::new(0x15, 0x16, 0x1e), //  0  black          #15161e
        Rgb::new(0xf7, 0x76, 0x8e), //  1  red            #f7768e
        Rgb::new(0x9e, 0xce, 0x6a), //  2  green          #9ece6a
        Rgb::new(0xe0, 0xaf, 0x68), //  3  yellow         #e0af68
        Rgb::new(0x7a, 0xa2, 0xf7), //  4  blue           #7aa2f7
        Rgb::new(0xbb, 0x9a, 0xf7), //  5  magenta        #bb9af7
        Rgb::new(0x7d, 0xcf, 0xff), //  6  cyan           #7dcfff
        Rgb::new(0xa9, 0xb1, 0xd6), //  7  white          #a9b1d6
    ],
    bright: [
        Rgb::new(0x41, 0x48, 0x68), //  8  bright black   #414868
        Rgb::new(0xff, 0x7a, 0x93), //  9  bright red     #ff7a93
        Rgb::new(0xb9, 0xf2, 0x7c), // 10  bright green   #b9f27c
        Rgb::new(0xff, 0x9e, 0x64), // 11  bright yellow  #ff9e64
        Rgb::new(0x7d, 0xa6, 0xff), // 12  bright blue    #7da6ff
        Rgb::new(0xbb, 0x9a, 0xf7), // 13  bright magenta #bb9af7
        Rgb::new(0x0d, 0xb9, 0xd7), // 14  bright cyan    #0db9d7
        Rgb::new(0xc0, 0xca, 0xf5), // 15  bright white   #c0caf5
    ],
};
