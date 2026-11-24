#[allow(unused_imports)]
use crate::prelude::*;

/// How far apart two syntax colours must look before they count as different.
///
/// Measured perceptually rather than by contrast: yellow and magenta can
/// share a luminance and still be obviously different colours.
const MIN_SEPARATION: f32 = 45.0;

/// Least contrast any of them may have with the background.
const MIN_READABLE: f32 = 2.6;

/// How many attempts to make a colour readable.
const MAX_NUDGES: usize = 14;

/// Points sampled between background and text when a colour has to be
/// replaced outright.
const RAMP_STEPS: usize = 16;

impl SyntaxPalette {
    /// Builds a palette for `theme`.
    ///
    /// Each kind starts from the slot it conventionally uses, then is pushed
    /// lighter or darker until it is readable on the background and distinct
    /// from the kinds already placed. On a colourful theme nothing moves; on a
    /// monochrome one the kinds spread across lightness instead of hue.
    pub fn derive(theme: &Theme) -> SyntaxPalette {
        let bg = theme.bg;
        let fg = theme.fg;
        // Plain text is fixed, so everything else has to differ from it:
        // Solarized's bright blue is its foreground, which made properties
        // invisible as a distinct kind.
        let mut taken: Vec<Rgb> = vec![fg];

        // Order matters: the kinds that appear most get first choice.
        let comment = place(theme.ui.tab_inactive_fg, bg, fg, &mut taken);
        let keyword = place(theme.indexed(4), bg, fg, &mut taken);
        let string = place(theme.indexed(2), bg, fg, &mut taken);
        let function = place(theme.indexed(3), bg, fg, &mut taken);
        let type_name = place(theme.indexed(6), bg, fg, &mut taken);
        let number = place(theme.indexed(5), bg, fg, &mut taken);
        let property = place(theme.indexed(12), bg, fg, &mut taken);

        SyntaxPalette {
            plain: fg,
            comment,
            string,
            number,
            keyword,
            type_name,
            function,
            property,
        }
    }

    /// Every colour, for checking they stay distinct.
    pub fn all(&self) -> [Rgb; 8] {
        [
            self.plain,
            self.comment,
            self.string,
            self.number,
            self.keyword,
            self.type_name,
            self.function,
            self.property,
        ]
    }
}

/// Makes `preferred` readable, then distinct from everything already chosen.
///
/// The theme's own colour is kept whenever it is already distinct, so a
/// colourful theme looks exactly as its author intended. Only when it clashes
/// is a replacement taken from the range between background and text, picking
/// whichever point sits furthest from the colours already used.
fn place(preferred: Rgb, bg: Rgb, fg: Rgb, taken: &mut Vec<Rgb>) -> Rgb {
    let chosen = readable(preferred, bg, fg);
    if clearance(chosen, taken) >= MIN_SEPARATION {
        taken.push(chosen);
        return chosen;
    }

    let mut best = chosen;
    let mut best_clearance = clearance(chosen, taken);

    // A monochrome theme has only one hue to offer, so neutrals are mixed in
    // as well: on an all-amber palette a grey is the furthest thing from
    // another amber, and still looks like it belongs.
    let neutral = if bg.relative_luminance() < 0.5 {
        Rgb::WHITE
    } else {
        Rgb::BLACK
    };

    for toward in [fg, neutral] {
        for step in 0..RAMP_STEPS {
            let position = (step + 1) as f32 / (RAMP_STEPS + 1) as f32;
            let candidate = readable(bg.blend(toward, position), bg, fg);
            let clear = clearance(candidate, taken);
            if clear > best_clearance {
                best = candidate;
                best_clearance = clear;
            }
        }
    }

    taken.push(best);
    best
}

/// Distance to the nearest colour already in use.
fn clearance(colour: Rgb, taken: &[Rgb]) -> f32 {
    taken
        .iter()
        .map(|other| other.perceptual_distance(colour))
        .fold(f32::MAX, f32::min)
}

/// Pushes a colour away from the background until it can be read.
fn readable(colour: Rgb, bg: Rgb, fg: Rgb) -> Rgb {
    let mut out = colour;
    for _ in 0..MAX_NUDGES {
        if bg.contrast_ratio(out) >= MIN_READABLE {
            return out;
        }
        out = out.blend(fg, 0.80);
    }
    out
}
