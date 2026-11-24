#[allow(unused_imports)]
use crate::prelude::*;

/// One Dark with a darker room and sharper syntax.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "one-dark-pro-contrast",
    description: "One Dark with a darker room and sharper syntax.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x11, 0x12, 0x16),
    fg: Rgb::new(0xc0, 0xc6, 0xd0),
    cursor: Rgb::new(0x67, 0x99, 0xff),
    selection_bg: Rgb::new(0x2e, 0x33, 0x3d),
    accent: Rgb::new(0x66, 0xba, 0xff),
    normal: [
        Rgb::new(0x0d, 0x0e, 0x10), //  0  black          #0d0e10
        Rgb::new(0xf3, 0x6e, 0x79), //  1  red            #f36e79
        Rgb::new(0xa3, 0xd4, 0x7f), //  2  green          #a3d47f
        Rgb::new(0xf6, 0xcc, 0x7d), //  3  yellow         #f6cc7d
        Rgb::new(0x66, 0xba, 0xff), //  4  blue           #66baff
        Rgb::new(0xd4, 0x7b, 0xee), //  5  magenta        #d47bee
        Rgb::new(0x5c, 0xca, 0xd8), //  6  cyan           #5ccad8
        Rgb::new(0xb2, 0xba, 0xc9), //  7  white          #b2bac9
    ],
    bright: [
        Rgb::new(0x33, 0x36, 0x3e), //  8  bright black   #33363e
        Rgb::new(0xf3, 0x6e, 0x79), //  9  bright red     #f36e79
        Rgb::new(0xa3, 0xd4, 0x7f), // 10  bright green   #a3d47f
        Rgb::new(0xf6, 0xcc, 0x7d), // 11  bright yellow  #f6cc7d
        Rgb::new(0x66, 0xba, 0xff), // 12  bright blue    #66baff
        Rgb::new(0xd4, 0x7b, 0xee), // 13  bright magenta #d47bee
        Rgb::new(0x5c, 0xca, 0xd8), // 14  bright cyan    #5ccad8
        Rgb::new(0xff, 0xff, 0xff), // 15  bright white   #ffffff
    ],
};
