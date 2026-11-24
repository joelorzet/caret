#[allow(unused_imports)]
use crate::prelude::*;

/// Dracula pushed to near-black with its colours brought forward.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "dracula-contrast",
    description: "Dracula pushed to near-black with its colours brought forward.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x11, 0x12, 0x17),
    fg: Rgb::new(0xfa, 0xfa, 0xf5),
    cursor: Rgb::new(0xfa, 0xfa, 0xf3),
    selection_bg: Rgb::new(0x33, 0x35, 0x44),
    accent: Rgb::new(0xc4, 0x9b, 0xff),
    normal: [
        Rgb::new(0x0e, 0x0e, 0x12), //  0  black          #0e0e12
        Rgb::new(0xff, 0x69, 0x69), //  1  red            #ff6969
        Rgb::new(0x61, 0xff, 0x89), //  2  green          #61ff89
        Rgb::new(0xf6, 0xff, 0x95), //  3  yellow         #f6ff95
        Rgb::new(0xc4, 0x9b, 0xff), //  4  blue           #c49bff
        Rgb::new(0xff, 0x89, 0xcd), //  5  magenta        #ff89cd
        Rgb::new(0x97, 0xed, 0xff), //  6  cyan           #97edff
        Rgb::new(0xfa, 0xfa, 0xf3), //  7  white          #fafaf3
    ],
    bright: [
        Rgb::new(0x35, 0x3e, 0x5b), //  8  bright black   #353e5b
        Rgb::new(0xff, 0x7f, 0x7f), //  9  bright red     #ff7f7f
        Rgb::new(0x7b, 0xff, 0xa1), // 10  bright green   #7bffa1
        Rgb::new(0xff, 0xff, 0xb0), // 11  bright yellow  #ffffb0
        Rgb::new(0xdb, 0xb6, 0xff), // 12  bright blue    #dbb6ff
        Rgb::new(0xff, 0x9f, 0xe3), // 13  bright magenta #ff9fe3
        Rgb::new(0xaf, 0xff, 0xff), // 14  bright cyan    #afffff
        Rgb::new(0xff, 0xff, 0xff), // 15  bright white   #ffffff
    ],
};
