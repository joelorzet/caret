use crate::support::parse;
use caret_config::Config;

#[test]
fn chrome_geometry_follows_the_config() {
    let (config, _) = parse(
        r#"
[chrome]
tab_bar = false
status = true
[sidebar.explorer]
visible = true
width = 30
[sidebar.agent]
visible = true
width = 50
"#,
    );
    let chrome = config.chrome();
    assert_eq!(chrome.tab_bar, 0);
    // The status line is a band, not a row: `status = true` asks for it, and
    // how tall it is belongs to the layout.
    assert_eq!(chrome.status, caret_layout::Chrome::STATUS_ROWS);
    assert!(chrome.left.visible);
    assert_eq!(chrome.left.cells, 30);
    assert!(chrome.right.visible);
    assert_eq!(chrome.right.cells, 50);
}

#[test]
fn the_shell_falls_back_to_the_environment() {
    let (config, _) = parse("[terminal]\nshell = \"/bin/fish\"\n");
    assert_eq!(config.shell_program(), "/bin/fish");
    let (config, _) = parse("");
    assert!(!config.shell_program().is_empty());
}

#[test]
fn font_settings_round_trip() {
    let (config, warnings) = parse(
        "[font]\nligatures = false\nitalics = false\nletter_spacing = 1.5\nbuiltin_box_drawing = false\n",
    );
    assert!(warnings.is_empty(), "{warnings:?}");
    assert!(!config.font.ligatures);
    assert!(!config.font.italics);
    assert!(!config.font.builtin_box_drawing);
    assert_eq!(config.font.letter_spacing, 1.5);

    let (config, _) = parse("[font]\nletter_spacing = 999.0\n");
    assert_eq!(config.font.letter_spacing, 16.0, "clamped");
}

#[test]
fn command_line_colouring_is_on_unless_turned_off() {
    assert!(Config::default().terminal.highlight_input);

    let (config, problems) = parse(
        r#"
[terminal]
highlight_input = false
"#,
    );
    assert!(problems.is_empty(), "{problems:?}");
    assert!(!config.terminal.highlight_input);
}

// A terminal that reaches the network has to say so and has to be easy to
// stop. The default is on, and one line turns it off for good.
#[test]
fn looking_for_updates_is_on_by_default_and_the_interval_is_held_in_range() {
    let (config, problems) = parse("");
    assert!(config.update.check);
    assert_eq!(config.update.interval_hours, 24);
    assert!(problems.is_empty());

    let (off, _) = parse("[update]\ncheck = false\n");
    assert!(!off.check_updates());

    let (eager, _) = parse("[update]\ninterval_hours = 0\n");
    assert_eq!(eager.update.interval_hours, 1, "not on every start");

    let (patient, _) = parse("[update]\ninterval_hours = 100000\n");
    assert_eq!(patient.update.interval_hours, 720);
}
