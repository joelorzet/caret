#[allow(unused_imports)]
use crate::prelude::*;

/// Carbonfox, after IBM Carbon.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "carbonfox",
    description: "Carbonfox, after IBM Carbon.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x16, 0x16, 0x16),
    fg: Rgb::new(0xf2, 0xf4, 0xf8),
    cursor: Rgb::new(0xb6, 0xb8, 0xbb),
    selection_bg: Rgb::new(0x2a, 0x2a, 0x2a),
    accent: Rgb::new(0x78, 0xa9, 0xff),
    normal: [
        Rgb::new(0x28, 0x28, 0x28), //  0  black          #282828
        Rgb::new(0xee, 0x53, 0x96), //  1  red            #ee5396
        Rgb::new(0x25, 0xbe, 0x6a), //  2  green          #25be6a
        Rgb::new(0x08, 0xbd, 0xba), //  3  yellow         #08bdba
        Rgb::new(0x78, 0xa9, 0xff), //  4  blue           #78a9ff
        Rgb::new(0xbe, 0x95, 0xff), //  5  magenta        #be95ff
        Rgb::new(0x33, 0xb1, 0xff), //  6  cyan           #33b1ff
        Rgb::new(0xdf, 0xdf, 0xe0), //  7  white          #dfdfe0
    ],
    bright: [
        Rgb::new(0x48, 0x48, 0x48), //  8  bright black   #484848
        Rgb::new(0xf1, 0x6d, 0xa6), //  9  bright red     #f16da6
        Rgb::new(0x46, 0xc8, 0x80), // 10  bright green   #46c880
        Rgb::new(0x2d, 0xc7, 0xc4), // 11  bright yellow  #2dc7c4
        Rgb::new(0x8c, 0xb6, 0xff), // 12  bright blue    #8cb6ff
        Rgb::new(0xc8, 0xa5, 0xff), // 13  bright magenta #c8a5ff
        Rgb::new(0x52, 0xbd, 0xff), // 14  bright cyan    #52bdff
        Rgb::new(0xe4, 0xe4, 0xe5), // 15  bright white   #e4e4e5
    ],
};
