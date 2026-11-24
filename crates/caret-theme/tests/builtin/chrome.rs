use caret_theme::builtin;

// The chrome shares the terminal background, so it has to be legible through
// text colour alone.
#[test]
fn chrome_is_legible_without_a_background_of_its_own() {
    for theme in builtin::all() {
        assert_eq!(theme.ui.chrome_bg, theme.bg, "{}", theme.name);
        assert_eq!(theme.ui.status_bg, theme.bg, "{}", theme.name);

        assert_ne!(
            theme.ui.tab_active_fg, theme.ui.tab_inactive_fg,
            "{}",
            theme.name
        );
        assert_ne!(theme.ui.border, theme.ui.border_focused, "{}", theme.name);
        assert_ne!(theme.ui.tab_active_bg, theme.bg, "{}", theme.name);
        assert_ne!(
            theme.ui.panel_bg, theme.bg,
            "{} dock is its own surface",
            theme.name
        );

        // Chrome text has to be readable.
        for (name, colour) in [
            ("accent", theme.ui.accent),
            ("status text", theme.ui.status_fg),
            ("inactive tab", theme.ui.tab_inactive_fg),
        ] {
            let ratio = theme.bg.contrast_ratio(colour);
            assert!(
                ratio >= 2.0,
                "{} {name} has {ratio:.2}:1 against the background",
                theme.name
            );
        }

        // A divider is a solid rule rather than text, so it reads at a lower
        // ratio than text would, and it should stay quiet.
        // Solved for rather than blended, so every theme lands in the same
        // narrow band whatever its own contrast range.
        let border = theme.bg.contrast_ratio(theme.ui.border);
        assert!(
            (2.2..=2.6).contains(&border),
            "{} border has {border:.2}:1, wanted a consistently quiet rule",
            theme.name
        );
    }
}

// Chrome surfaces lie on the terminal background; if they land too close to
// it they are not surfaces at all, which is what a fixed blend produced on
// the near-black themes.
#[test]
fn every_theme_has_a_visible_active_tab() {
    for theme in builtin::all() {
        let name = &theme.name;
        let ratio = theme.bg.contrast_ratio(theme.ui.tab_active_bg);
        assert!(
            ratio > 1.2,
            "{name}: the active tab is {ratio:.2}:1 against the background"
        );
    }
}

#[test]
fn the_sidebar_reads_as_its_own_surface_but_quieter_than_a_tab() {
    for theme in builtin::all() {
        let name = &theme.name;
        let sidebar = theme.bg.contrast_ratio(theme.ui.panel_bg);
        let tab = theme.bg.contrast_ratio(theme.ui.tab_active_bg);
        assert!(sidebar > 1.1, "{name}: sidebar is flat at {sidebar:.2}:1");
        assert!(
            sidebar < tab,
            "{name}: the sidebar outshouts the active tab"
        );
    }
}

// A focused divider runs the whole height of the window, so it has to be
// unmistakable without being the brightest thing on screen.
#[test]
fn a_focused_edge_is_visible_but_quieter_than_the_accent() {
    for theme in builtin::all() {
        let name = &theme.name;
        let focused = theme.bg.contrast_ratio(theme.ui.border_focused);
        let quiet = theme.bg.contrast_ratio(theme.ui.border);
        assert!(focused >= quiet, "{name}: focus reads weaker than no focus");
        assert!(
            focused <= theme.bg.contrast_ratio(theme.ui.accent),
            "{name}: the edge is louder than the accent it comes from"
        );
    }
}

// A divider is furniture. If it out-shines the text it separates, the eye
// goes to the frame instead of the content.
#[test]
fn no_divider_is_brighter_than_body_text() {
    for theme in builtin::all() {
        let name = &theme.name;
        let text = theme.bg.contrast_ratio(theme.fg);
        for (what, colour) in [
            ("border", theme.ui.border),
            ("border_focused", theme.ui.border_focused),
        ] {
            let ratio = theme.bg.contrast_ratio(colour);
            assert!(ratio < text, "{name}: {what} is louder than the text");
        }
    }
}

// A command list is content. It was being drawn in the same faded slot as the
// status line, which made every unselected row read as disabled.
#[test]
fn list_text_on_chrome_is_properly_readable() {
    for theme in builtin::all() {
        let name = &theme.name;
        let primary = theme.ui.panel_bg.contrast_ratio(theme.ui.chrome_fg);
        let secondary = theme.ui.panel_bg.contrast_ratio(theme.ui.tab_inactive_fg);
        let body = theme.bg.contrast_ratio(theme.fg);

        // Measured against what the theme itself offers: a chrome surface may
        // cost some readability, but not most of it.
        assert!(
            primary >= body * 0.7,
            "{name}: list rows keep only {:.0}% of the theme's contrast",
            primary / body * 100.0
        );
        assert!(
            primary > secondary * 1.4,
            "{name}: primary and secondary text look the same"
        );
    }
}
