use caret_theme::{builtin, SyntaxPalette, Theme};

const KINDS: [&str; 8] = [
    "plain", "comment", "string", "number", "keyword", "type", "function", "property",
];

fn palette(theme: &Theme) -> SyntaxPalette {
    SyntaxPalette::derive(theme)
}

// The failure this exists to prevent: on a deliberately monochrome theme every
// palette slot is the same hue, so code came out in one colour.
#[test]
fn every_theme_gives_code_distinguishable_colours() {
    for theme in builtin::all() {
        let colours = palette(&theme).all();

        for (i, a) in colours.iter().enumerate() {
            for (j, b) in colours.iter().enumerate().skip(i + 1) {
                let distance = a.perceptual_distance(*b);
                assert!(
                    distance >= 40.0,
                    "{}: {} and {} are {distance:.0} apart, too close to tell apart",
                    theme.name,
                    KINDS[i],
                    KINDS[j]
                );
            }
        }
    }
}

#[test]
fn every_syntax_colour_is_readable_on_the_background() {
    for theme in builtin::all() {
        for (name, colour) in KINDS.iter().zip(palette(&theme).all()) {
            let ratio = theme.bg.contrast_ratio(colour);
            assert!(
                ratio >= 2.4,
                "{}: {name} has {ratio:.2}:1 against the background",
                theme.name
            );
        }
    }
}

// The monochrome themes are the hard case, so they get checked by name.
#[test]
fn the_monochrome_themes_still_separate_their_tokens() {
    for name in [
        "phosphor-green",
        "phosphor-green-soft",
        "phosphor-amber",
        "phosphor-amber-soft",
    ] {
        let theme = builtin::by_name(name).expect(name);
        let colours = palette(&theme).all();

        let mut seen: Vec<_> = colours.to_vec();
        seen.sort_by_key(|c| (c.r, c.g, c.b));
        seen.dedup();
        assert_eq!(seen.len(), colours.len(), "{name} repeats a colour");

        // Comments should stay the quietest thing on screen.
        let syntax = palette(&theme);
        let comment = theme.bg.contrast_ratio(syntax.comment);
        let keyword = theme.bg.contrast_ratio(syntax.keyword);
        assert!(
            comment < keyword,
            "{name}: comments should be quieter than keywords"
        );
    }
}

// A theme with plenty of hue should keep the colours it was designed with.
#[test]
fn a_colourful_theme_keeps_its_own_colours() {
    let theme = builtin::by_name("dracula").unwrap();
    let syntax = palette(&theme);
    assert_eq!(syntax.keyword, theme.indexed(4), "blue stayed blue");
    assert_eq!(syntax.string, theme.indexed(2), "green stayed green");
    assert_eq!(syntax.plain, theme.fg);
}

#[test]
fn light_themes_work_too() {
    for name in [
        "github-light",
        "catppuccin-latte",
        "gruvbox-light",
        "ayu-light",
    ] {
        let theme = builtin::by_name(name).expect(name);
        let colours = palette(&theme).all();
        for (kind, colour) in KINDS.iter().zip(colours) {
            assert!(
                theme.bg.contrast_ratio(colour) >= 2.4,
                "{name}: {kind} is washed out"
            );
        }
    }
}

#[test]
fn deriving_twice_gives_the_same_answer() {
    let theme = builtin::by_name("tokyo-night").unwrap();
    assert_eq!(palette(&theme).all(), palette(&theme).all());
}
