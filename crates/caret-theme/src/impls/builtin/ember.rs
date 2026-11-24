#[allow(unused_imports)]
use crate::prelude::*;

/// Orange-led on warm charcoal rather than pure black, so it works as a
/// general-purpose theme and not only as a novelty.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "ember",
    description:
        "Orange-led on warm charcoal rather than pure black, so it works as a daily driver.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x1b, 0x17, 0x14),
    fg: Rgb::new(0xe8, 0xd9, 0xcc),
    cursor: Rgb::new(0xff, 0x77, 0x33),
    selection_bg: Rgb::new(0x3d, 0x31, 0x29),
    accent: Rgb::new(0xff, 0x77, 0x33),
    normal: [
        Rgb::new(0x2a, 0x23, 0x1e), //  0  black          #2a231e
        Rgb::new(0xef, 0x5f, 0x52), //  1  red            #ef5f52
        Rgb::new(0x8f, 0xae, 0x5e), //  2  green          #8fae5e
        Rgb::new(0xff, 0xa6, 0x4d), //  3  yellow         #ffa64d
        Rgb::new(0x6f, 0xa8, 0xc7), //  4  blue           #6fa8c7
        Rgb::new(0xd0, 0x8a, 0xb0), //  5  magenta        #d08ab0
        Rgb::new(0x78, 0xbf, 0xae), //  6  cyan           #78bfae
        Rgb::new(0xd8, 0xc8, 0xba), //  7  white          #d8c8ba
    ],
    bright: [
        Rgb::new(0x57, 0x4a, 0x40), //  8  bright black   #574a40
        Rgb::new(0xff, 0x85, 0x77), //  9  bright red     #ff8577
        Rgb::new(0xb3, 0xcc, 0x85), // 10  bright green   #b3cc85
        Rgb::new(0xff, 0xc0, 0x7a), // 11  bright yellow  #ffc07a
        Rgb::new(0x96, 0xc6, 0xde), // 12  bright blue    #96c6de
        Rgb::new(0xe4, 0xac, 0xc8), // 13  bright magenta #e4acc8
        Rgb::new(0x9b, 0xd8, 0xc9), // 14  bright cyan    #9bd8c9
        Rgb::new(0xf5, 0xec, 0xe3), // 15  bright white   #f5ece3
    ],
};
