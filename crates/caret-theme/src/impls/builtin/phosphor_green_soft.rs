#[allow(unused_imports)]
use crate::prelude::*;

/// Desaturated phosphor green, for long sessions without the glare.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "phosphor-green-soft",
    description: "Desaturated phosphor green, for long sessions without the glare.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x0b, 0x0f, 0x0c),
    fg: Rgb::new(0x9a, 0xd4, 0xa8),
    cursor: Rgb::new(0x7f, 0xc7, 0x95),
    selection_bg: Rgb::new(0x1e, 0x33, 0x2a),
    accent: Rgb::new(0x6f, 0xbf, 0x85),
    normal: [
        Rgb::new(0x16, 0x1d, 0x18), //  0  black          #161d18
        Rgb::new(0xd9, 0x73, 0x6a), //  1  red            #d9736a
        Rgb::new(0x6f, 0xbf, 0x85), //  2  green          #6fbf85
        Rgb::new(0xa8, 0xbf, 0x6f), //  3  yellow         #a8bf6f
        Rgb::new(0x5a, 0xa8, 0x8a), //  4  blue           #5aa88a
        Rgb::new(0x8f, 0xbf, 0xa3), //  5  magenta        #8fbfa3
        Rgb::new(0x74, 0xb8, 0xa8), //  6  cyan           #74b8a8
        Rgb::new(0xb6, 0xc9, 0xbb), //  7  white          #b6c9bb
    ],
    bright: [
        Rgb::new(0x33, 0x45, 0x3a), //  8  bright black   #33453a
        Rgb::new(0xe0, 0x9a, 0x92), //  9  bright red     #e09a92
        Rgb::new(0x8f, 0xd4, 0xa3), // 10  bright green   #8fd4a3
        Rgb::new(0xc2, 0xd4, 0x9a), // 11  bright yellow  #c2d49a
        Rgb::new(0x7f, 0xc2, 0xa8), // 12  bright blue    #7fc2a8
        Rgb::new(0xaa, 0xd1, 0xbb), // 13  bright magenta #aad1bb
        Rgb::new(0x97, 0xcf, 0xc2), // 14  bright cyan    #97cfc2
        Rgb::new(0xd6, 0xe3, 0xd9), // 15  bright white   #d6e3d9
    ],
};
