use caret_config::ThemeRegistry;
use caret_core::Rgb;
use std::path::PathBuf;

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("caret-themes-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

const MINIMAL: &str = r##"
name = "midnight"
background = "#101018"
foreground = "#e0e0f0"
normal = ["#101018", "#e05a5a", "#5ae07a", "#e0d05a", "#5a8ae0", "#c05ae0", "#5ad0e0", "#c0c0d0"]
bright = ["#404050", "#f07a7a", "#7af09a", "#f0e07a", "#7aaaf0", "#d07af0", "#7af0f0", "#f0f0ff"]
"##;

#[test]
fn builtins_are_always_available() {
    let registry = ThemeRegistry::builtin_only();
    assert!(registry.themes().len() >= 10);
    assert!(registry.get("dracula").is_some());
}

#[test]
fn lookup_ignores_case_and_separators() {
    let registry = ThemeRegistry::builtin_only();
    assert!(registry.get("Phosphor Green").is_some());
    assert!(registry.get("phosphor_green").is_some());
}

#[test]
fn resolve_falls_back_rather_than_failing() {
    let registry = ThemeRegistry::builtin_only();
    assert_eq!(registry.resolve("nord").name, "nord");
    assert_eq!(
        registry.resolve("does-not-exist").name,
        caret_theme::builtin::DEFAULT
    );
}

#[test]
fn a_user_theme_is_loaded_and_its_extras_derived() {
    let dir = temp_dir("minimal");
    std::fs::write(dir.join("midnight.toml"), MINIMAL).unwrap();

    let (registry, warnings) = ThemeRegistry::load_from_dir(&dir);
    assert!(warnings.is_empty(), "{warnings:?}");

    let theme = registry.get("midnight").expect("user theme should load");
    assert_eq!(theme.bg, Rgb::new(0x10, 0x10, 0x18));
    assert_eq!(theme.cursor, theme.fg, "cursor defaults to the foreground");
    assert_eq!(
        theme.ui.accent, theme.palette.normal[4],
        "accent defaults to blue"
    );
    assert!(theme.is_dark(), "appearance inferred from the background");
    assert_ne!(theme.ui.border, theme.bg, "chrome is still derived");
}

#[test]
fn a_user_theme_overrides_a_builtin_of_the_same_name() {
    let dir = temp_dir("override");
    std::fs::write(
        dir.join("dracula.toml"),
        MINIMAL.replace("midnight", "dracula"),
    )
    .unwrap();

    let (registry, warnings) = ThemeRegistry::load_from_dir(&dir);
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(
        registry.get("dracula").unwrap().bg,
        Rgb::new(0x10, 0x10, 0x18)
    );
    assert_eq!(
        registry.themes().len(),
        ThemeRegistry::builtin_only().themes().len(),
        "it replaces rather than adds"
    );
}

#[test]
fn short_hex_and_bare_hex_are_accepted() {
    let dir = temp_dir("hex");
    let text = MINIMAL
        .replace("\"#101018\"\nforeground", "\"#123\"\nforeground")
        .replace("foreground = \"#e0e0f0\"", "foreground = \"e0e0f0\"");
    std::fs::write(dir.join("hex.toml"), text).unwrap();

    let (registry, warnings) = ThemeRegistry::load_from_dir(&dir);
    assert!(warnings.is_empty(), "{warnings:?}");
    let theme = registry.get("midnight").unwrap();
    assert_eq!(theme.bg, Rgb::new(0x11, 0x22, 0x33));
    assert_eq!(theme.fg, Rgb::new(0xe0, 0xe0, 0xf0));
}

#[test]
fn a_broken_theme_warns_without_losing_the_builtins() {
    let dir = temp_dir("broken");
    std::fs::write(
        dir.join("bad.toml"),
        "background = \"#nope\"\nforeground = \"#fff\"\nnormal = []\nbright = []\n",
    )
    .unwrap();
    std::fs::write(dir.join("short.toml"), MINIMAL.replace(", \"#c0c0d0\"", "")).unwrap();

    let (registry, warnings) = ThemeRegistry::load_from_dir(&dir);
    assert_eq!(warnings.len(), 2, "{warnings:?}");
    assert!(warnings.iter().any(|w| w.contains("hex")), "{warnings:?}");
    assert!(
        warnings.iter().any(|w| w.contains("8 colours")),
        "{warnings:?}"
    );
    assert!(registry.get("dracula").is_some());
}

#[test]
fn an_unknown_field_in_a_theme_is_rejected_outright() {
    let dir = temp_dir("strict");
    std::fs::write(
        dir.join("t.toml"),
        format!("{MINIMAL}\nbackgrnd = \"#000\"\n"),
    )
    .unwrap();
    let (_, warnings) = ThemeRegistry::load_from_dir(&dir);
    assert_eq!(warnings.len(), 1, "a theme is all or nothing: {warnings:?}");
}

#[test]
fn a_missing_themes_directory_is_fine() {
    let (registry, warnings) = ThemeRegistry::load_from_dir(&PathBuf::from("/nonexistent/caret"));
    assert!(warnings.is_empty());
    assert!(!registry.themes().is_empty());
}
