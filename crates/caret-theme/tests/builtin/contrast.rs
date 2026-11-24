use caret_theme::builtin;

// The point of shipping themes is that they are comfortable to read. 4.5:1 is
// the WCAG AA bar for body text; a theme below it does not ship.
#[test]
fn body_text_clears_the_contrast_bar() {
    for theme in builtin::all() {
        let ratio = theme.text_contrast();
        assert!(
            ratio >= 4.5,
            "{} has {ratio:.2}:1 text contrast, needs 4.5",
            theme.name
        );
    }
}

#[test]
fn nothing_is_invisible_against_the_background() {
    for theme in builtin::all() {
        assert_ne!(theme.fg, theme.bg, "{}", theme.name);
        assert_ne!(theme.cursor, theme.bg, "{}", theme.name);
        assert_ne!(theme.selection_bg, theme.bg, "{}", theme.name);
        assert!(
            theme.selection_bg.contrast_ratio(theme.selection_fg) >= 2.0,
            "{} selection is unreadable",
            theme.name
        );
        assert!(
            theme.cursor.contrast_ratio(theme.cursor_text) >= 2.0,
            "{} cursor text is unreadable",
            theme.name
        );
    }
}

// Colour-coded output is useless if the primaries collide.
#[test]
fn the_primaries_are_distinct_within_each_theme() {
    for theme in builtin::all() {
        let primaries: Vec<_> = (1..=6).map(|i| theme.indexed(i)).collect();
        for (i, a) in primaries.iter().enumerate() {
            for b in &primaries[i + 1..] {
                assert_ne!(a, b, "{} reuses a primary colour", theme.name);
            }
        }
    }
}

// Themes we authored ourselves are held to a stricter bar than the ports,
// where we keep the original author's choices.
#[test]
fn our_own_themes_keep_every_primary_legible() {
    let ours = [
        "phosphor-green",
        "phosphor-green-soft",
        "phosphor-amber",
        "phosphor-amber-soft",
        "phosphor-yellow",
        "phosphor-orange",
        "phosphor-cyan",
        "phosphor-blue",
        "phosphor-magenta",
        "phosphor-white",
        "ember",
    ];
    for name in ours {
        let theme = builtin::by_name(name).expect(name);
        for i in 1..=7u8 {
            let ratio = theme.bg.contrast_ratio(theme.indexed(i));
            assert!(
                ratio >= 2.0,
                "{name} colour {i} has {ratio:.2}:1 against the background"
            );
        }
    }
}
