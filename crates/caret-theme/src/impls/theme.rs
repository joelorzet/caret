#[allow(unused_imports)]
use crate::prelude::*;

/// Whichever of `a` or `b` reads better against `on`.
fn readable_on(on: Rgb, a: Rgb, b: Rgb) -> Rgb {
    if on.contrast_ratio(a) >= on.contrast_ratio(b) {
        a
    } else {
        b
    }
}

impl Theme {
    pub fn from_spec(spec: &ThemeSpec) -> Theme {
        Theme::new(
            std::borrow::Cow::Borrowed(spec.name),
            std::borrow::Cow::Borrowed(spec.description),
            spec.appearance,
            spec.bg,
            spec.fg,
            spec.cursor,
            spec.selection_bg,
            spec.accent,
            spec.normal,
            spec.bright,
        )
    }

    /// Builds a theme from its colors, deriving cursor text, selection text
    /// and the whole chrome palette. Shared with themes loaded from disk.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: std::borrow::Cow<'static, str>,
        description: std::borrow::Cow<'static, str>,
        appearance: Appearance,
        bg: Rgb,
        fg: Rgb,
        cursor: Rgb,
        selection_bg: Rgb,
        accent: Rgb,
        normal: [Rgb; 8],
        bright: [Rgb; 8],
    ) -> Theme {
        Theme {
            name,
            description,
            appearance,
            palette: Palette { normal, bright },
            bg,
            fg,
            cursor,
            cursor_text: readable_on(cursor, bg, fg),
            selection_bg,
            selection_fg: readable_on(selection_bg, fg, bg),
            ui: UiColors::derive(bg, fg, accent, appearance),
        }
    }

    pub fn is_dark(&self) -> bool {
        self.appearance == Appearance::Dark
    }

    /// Expands a cell color to pixels. Slots 0-15 come from the theme, the
    /// rest from the xterm cube.
    pub fn resolve(&self, color: Color, slot: ColorSlot) -> Rgb {
        match color.kind() {
            ColorKind::Default => match slot {
                ColorSlot::Foreground => self.fg,
                ColorSlot::Background => self.bg,
            },
            ColorKind::Indexed(i) => self.indexed(i),
            ColorKind::Rgb(c) => c,
        }
    }

    pub fn indexed(&self, index: u8) -> Rgb {
        self.palette
            .get(index)
            .or_else(|| xterm_256(index))
            .unwrap_or(self.fg)
    }

    /// Contrast of body text against the background. Anything below 4.5 is
    /// hard to read for long.
    pub fn text_contrast(&self) -> f32 {
        self.bg.contrast_ratio(self.fg)
    }
}
