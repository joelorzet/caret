#[allow(unused_imports)]
use crate::prelude::*;

/// Ayu with the lights off and the accents left burning.
pub const SPEC: ThemeSpec = ThemeSpec {
    name: "ayu-dark-contrast",
    description: "Ayu with the lights off and the accents left burning.",
    appearance: Appearance::Dark,
    bg: Rgb::new(0x04, 0x06, 0x08),
    fg: Rgb::new(0xc7, 0xc5, 0xc2),
    cursor: Rgb::new(0xff, 0xd2, 0x78),
    selection_bg: Rgb::new(0x17, 0x1b, 0x24),
    accent: Rgb::new(0x6d, 0xc9, 0xff),
    normal: [
        Rgb::new(0x00, 0x03, 0x06), //  0  black          #000306
        Rgb::new(0xfd, 0x6d, 0x75), //  1  red            #fd6d75
        Rgb::new(0xa0, 0xc7, 0x6a), //  2  green          #a0c76a
        Rgb::new(0xff, 0xb9, 0x5f), //  3  yellow         #ffb95f
        Rgb::new(0x63, 0xc6, 0xff), //  4  blue           #63c6ff
        Rgb::new(0xff, 0xef, 0x9c), //  5  magenta        #ffef9c
        Rgb::new(0x93, 0xef, 0xd0), //  6  cyan           #93efd0
        Rgb::new(0xce, 0xce, 0xce), //  7  white          #cecece
    ],
    bright: [
        Rgb::new(0x39, 0x39, 0x39), //  8  bright black   #393939
        Rgb::new(0xff, 0x75, 0x7c), //  9  bright red     #ff757c
        Rgb::new(0xd6, 0xf0, 0x4f), // 10  bright green   #d6f04f
        Rgb::new(0xff, 0xbd, 0x69), // 11  bright yellow  #ffbd69
        Rgb::new(0x6d, 0xc9, 0xff), // 12  bright blue    #6dc9ff
        Rgb::new(0xff, 0xf0, 0xa5), // 13  bright magenta #fff0a5
        Rgb::new(0x97, 0xf4, 0xd5), // 14  bright cyan    #97f4d5
        Rgb::new(0xff, 0xff, 0xff), // 15  bright white   #ffffff
    ],
};
