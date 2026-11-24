use caret_theme::{builtin, Appearance};

#[test]
fn every_theme_builds() {
    let all = builtin::all();
    assert!(all.len() >= 10, "got {} themes", all.len());
    assert_eq!(all.len(), builtin::names().len());
}

#[test]
fn names_are_unique() {
    let mut names = builtin::names();
    names.sort();
    let before = names.len();
    names.dedup();
    assert_eq!(names.len(), before);
}

#[test]
fn lookup_ignores_case_and_separators() {
    for spelling in ["tokyo-night", "tokyo_night", "Tokyo Night", "TOKYONIGHT"] {
        assert_eq!(
            builtin::by_name(spelling).map(|t| t.name.to_string()),
            Some("tokyo-night".to_string()),
            "{spelling} should resolve"
        );
    }
    assert!(builtin::by_name("no-such-theme").is_none());
}

#[test]
fn the_default_theme_exists() {
    assert_eq!(builtin::default_theme().name, builtin::DEFAULT);
}

#[test]
fn both_appearances_are_represented() {
    let all = builtin::all();
    assert!(all.iter().any(|t| t.appearance == Appearance::Dark));
    assert!(all.iter().any(|t| t.appearance == Appearance::Light));
}

#[test]
fn the_requested_families_are_present() {
    for name in [
        "dracula",
        "one-dark-pro",
        "phosphor-green",
        "phosphor-amber",
        "ember",
    ] {
        assert!(builtin::by_name(name).is_some(), "{name} is missing");
    }
    // Each lead-colour family ships a variant.
    assert!(builtin::by_name("phosphor-green-soft").is_some());
    assert!(builtin::by_name("phosphor-amber-soft").is_some());

    // The phosphors cover the wheel, not just the two colours CRTs came in.
    for lead in ["yellow", "orange", "cyan", "blue", "magenta", "white"] {
        assert!(
            builtin::by_name(&format!("phosphor-{lead}")).is_some(),
            "phosphor-{lead} is missing"
        );
    }
}

#[test]
fn the_sixteen_ansi_colors_are_all_present() {
    for theme in builtin::all() {
        assert_eq!(theme.palette.iter().count(), 16, "{}", theme.name);
        for i in 0..16u8 {
            assert!(theme.palette.get(i).is_some(), "{} slot {i}", theme.name);
        }
        assert!(theme.palette.get(16).is_none());
    }
}
