#[allow(unused_imports)]
use crate::prelude::*;

/// Mixes `amount` of `to` into `from`.
fn toward(from: Rgb, to: Rgb, amount: f32) -> Rgb {
    to.blend(from, amount)
}

impl UiColors {
    /// Builds the chrome colors from a theme's background, text and accent.
    ///
    /// The chrome shares the terminal's own background rather than sitting on
    /// a lighter band. Blending toward white muddies every theme and makes the
    /// bars read as a different, worse application; separation comes from text
    /// weight and colour instead, which is what a modern editor does.
    pub fn derive(bg: Rgb, fg: Rgb, accent: Rgb, appearance: Appearance) -> UiColors {
        let lift = match appearance {
            Appearance::Dark => Rgb::WHITE,
            Appearance::Light => Rgb::BLACK,
        };
        UiColors {
            chrome_bg: bg,
            // Chrome's primary text. The rows of a command list are content,
            // not furniture: faded, every unselected row read as disabled.
            chrome_fg: fg,
            // Just enough to read as a chip, not as a slab.
            tab_active_bg: surface(bg, lift, TAB),
            tab_active_fg: fg,
            tab_inactive_fg: toward(fg, bg, 0.45),
            border: rule(bg, fg),
            border_focused: focused_edge(bg, accent, rule(bg, fg)),
            status_bg: bg,
            status_fg: toward(fg, bg, 0.45),
            // A dock is a panel, not terminal output; a hint of a different
            // surface is what makes that legible at a glance.
            panel_bg: surface(bg, lift, PANEL),
            panel_fg: toward(fg, bg, 0.12),
            selection_ui: toward(bg, accent, 0.28),
            accent,
        }
    }
}

/// How far each surface sits from the background it lies on, as contrast.
const TAB: f32 = 1.7;
const PANEL: f32 = 1.22;
const RULE: f32 = 2.4;

/// A light theme can only make a surface by going darker, and past this it
/// stops looking like the same application.
const MAX_SHIFT: f32 = 0.16;

/// A focused edge is a rule the width of one cell running the height of the
/// window; at full accent that is a glowing bar rather than a hint of where
/// focus is, so it is pulled back toward the background.
const FOCUS: f32 = 0.5;

/// A surface that reads the same distance from its background in every theme.
///
/// A fixed blend cannot work: a theme whose background is near-black and text
/// near-white lands far hotter than one with a narrow range, so a 5% lift was
/// a clear chip on one theme and literally invisible on another with a
/// near-black background. Solving for a target contrast instead makes every
/// surface equally present everywhere.
fn surface(bg: Rgb, toward_colour: Rgb, target: f32) -> Rgb {
    // Contrast rises monotonically as the surface moves away from the
    // background, so the blend that hits the target can simply be bisected
    // for.
    let (mut low, mut high) = (0.0f32, 1.0f32);
    for _ in 0..16 {
        let mid = (low + high) / 2.0;
        if bg.contrast_ratio(toward(bg, toward_colour, mid)) < target {
            low = mid;
        } else {
            high = mid;
        }
    }
    toward(bg, toward_colour, high.min(MAX_SHIFT))
}

/// The edge marking the focused pane.
///
/// Pulled back from the accent, because the edge runs the whole height of the
/// window. A theme whose accent is already soft against its background is left
/// alone: muting it further would make a focused edge read as quieter than an
/// unfocused one, which is backwards.
fn focused_edge(bg: Rgb, accent: Rgb, quiet: Rgb) -> Rgb {
    let floor = bg.contrast_ratio(quiet) * 1.25;
    let pulled = toward(bg, accent, FOCUS);
    if bg.contrast_ratio(pulled) >= floor {
        return pulled;
    }

    // Pulling back left the focused edge quieter than the unfocused rule,
    // which is backwards. Raise it to just clear that floor rather than
    // falling back to the accent itself: on a theme whose accent is its text
    // colour, that made the frame as loud as the content it frames.
    let (mut low, mut high) = (FOCUS, 1.0f32);
    let mut found = accent;
    for _ in 0..16 {
        let mid = (low + high) / 2.0;
        found = toward(bg, accent, mid);
        if bg.contrast_ratio(found) < floor {
            low = mid;
        } else {
            high = mid;
        }
    }
    found
}

/// A divider is a line rather than a surface, and is allowed to go as far as
/// it needs to be seen.
fn rule(bg: Rgb, fg: Rgb) -> Rgb {
    let (mut low, mut high) = (0.0f32, 1.0f32);
    let mut found = toward(bg, fg, high);
    for _ in 0..16 {
        let mid = (low + high) / 2.0;
        found = toward(bg, fg, mid);
        if bg.contrast_ratio(found) < RULE {
            low = mid;
        } else {
            high = mid;
        }
    }
    found
}
