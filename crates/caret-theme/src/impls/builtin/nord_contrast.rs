#[allow(unused_imports)]
use crate::prelude::*;

/// Nord taken down to deep water, the frost colours left bright.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "nord-contrast",
    description: "Nord taken down to deep water, the frost colours left bright.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x13, 0x16, 0x1b),
    fg: Rgb::new(0xe2, 0xe6, 0xef),
    cursor: Rgb::new(0xda, 0xe1, 0xee),
    selection_bg: Rgb::new(0x32, 0x39, 0x46),
    accent: Rgb::new(0x8d, 0xcd, 0xdf),
    normal: [
        Rgb::new(0x19, 0x1c, 0x22), //  0  black          #191c22
        Rgb::new(0xd3, 0x68, 0x72), //  1  red            #d36872
        Rgb::new(0xae, 0xcc, 0x93), //  2  green          #aecc93
        Rgb::new(0xfa, 0xd5, 0x8c), //  3  yellow         #fad58c
        Rgb::new(0x88, 0xac, 0xd1), //  4  blue           #88acd1
        Rgb::new(0xc2, 0x97, 0xba), //  5  magenta        #c297ba
        Rgb::new(0x8d, 0xcd, 0xdf), //  6  cyan           #8dcddf
        Rgb::new(0xe7, 0xeb, 0xf3), //  7  white          #e7ebf3
    ],
    bright: [
        Rgb::new(0x2a, 0x2f, 0x3a), //  8  bright black   #2a2f3a
        Rgb::new(0xd3, 0x68, 0x72), //  9  bright red     #d36872
        Rgb::new(0xae, 0xcc, 0x93), // 10  bright green   #aecc93
        Rgb::new(0xfa, 0xd5, 0x8c), // 11  bright yellow  #fad58c
        Rgb::new(0x88, 0xac, 0xd1), // 12  bright blue    #88acd1
        Rgb::new(0xc2, 0x97, 0xba), // 13  bright magenta #c297ba
        Rgb::new(0x96, 0xca, 0xc9), // 14  bright cyan    #96cac9
        Rgb::new(0xed, 0xf1, 0xf6), // 15  bright white   #edf1f6
    ],
};
