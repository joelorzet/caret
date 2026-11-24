use crate::support::parse;
use caret_config::{Action, Config, KeyBinding, KeyResolution};

#[test]
fn the_five_presets_exist_without_configuration() {
    let config = Config::default();
    assert_eq!(config.layouts.len(), 5);
    for name in [
        "even-horizontal",
        "even-vertical",
        "main-vertical",
        "main-horizontal",
        "tiled",
    ] {
        assert!(config.layout(name).is_some(), "{name} is missing");
    }
}

#[test]
fn a_preset_layout_can_be_saved_by_name() {
    let (config, warnings) = parse(
        "[layouts.dev]\ndescription = \"editor and shells\"\npreset = \"main-vertical\"\npanes = 3\nmain = 60.0\n",
    );
    assert!(warnings.is_empty(), "{warnings:?}");

    let layout = config.layout("dev").expect("dev layout");
    assert_eq!(layout.description, "editor and shells");
    assert_eq!(layout.pane_count(1), 3, "the saved count wins");
}

#[test]
fn an_explicit_shape_can_be_saved() {
    let (config, warnings) = parse("[layouts.review]\nsplit = \"h(60:*, 40:v(*, *, *))\"\n");
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(config.layout("review").unwrap().pane_count(1), 4);
}

#[test]
fn a_saved_layout_can_replace_a_preset() {
    let (config, _) = parse("[layouts.tiled]\npreset = \"even-vertical\"\npanes = 2\n");
    assert_eq!(config.layouts.len(), 5, "replaced, not added");
    assert_eq!(config.layout("tiled").unwrap().pane_count(9), 2);
}

#[test]
fn layout_lookup_ignores_case_and_separators() {
    let config = Config::default();
    assert!(config.layout("Main Vertical").is_some());
    assert!(config.layout("main_vertical").is_some());
    assert!(config.layout("nope").is_none());
}

#[test]
fn a_layout_needing_both_or_neither_is_reported() {
    let (_, warnings) = parse("[layouts.a]\n");
    assert!(
        warnings[0].contains("either preset or split"),
        "{warnings:?}"
    );

    let (_, warnings) = parse("[layouts.b]\npreset = \"tiled\"\nsplit = \"h(*, *)\"\n");
    assert!(warnings[0].contains("not both"), "{warnings:?}");
}

#[test]
fn a_bad_layout_is_reported_with_the_valid_presets() {
    let (config, warnings) = parse("[layouts.a]\npreset = \"spiral\"\n");
    assert!(warnings[0].contains("main-vertical"), "{warnings:?}");
    assert!(config.layout("a").is_none());

    let (_, warnings) = parse("[layouts.b]\nsplit = \"h(*)\"\n");
    assert!(warnings[0].contains("at least two"), "{warnings:?}");
}

#[test]
fn typos_inside_a_layout_are_caught() {
    let (_, warnings) = parse("[layouts.a]\npreset = \"tiled\"\npanez = 3\n");
    assert!(warnings.iter().any(|w| w.contains("panez")), "{warnings:?}");
}

#[test]
fn layouts_can_be_bound_to_keys() {
    let (config, warnings) = parse(
        "[layouts.dev]\npreset = \"main-vertical\"\n\n[keys.prefixed]\n\"D\" = \"apply_layout:dev\"\n",
    );
    assert!(warnings.is_empty(), "{warnings:?}");
    assert_eq!(
        config.keymap.resolve(KeyBinding::parse("D").unwrap(), true),
        KeyResolution::Run(Action::ApplyLayout("dev".into()))
    );
}
