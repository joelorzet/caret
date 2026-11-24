use caret_app::opens_links;
use caret_config::Modifiers;

// A plain click still places the cursor and starts a selection, so opening
// takes the modifier people's hands already go to on their platform.
#[test]
fn opening_takes_the_modifier_this_platform_uses() {
    let command = Modifiers::SUPER;
    let control = Modifiers::CTRL;
    if cfg!(target_os = "macos") {
        assert!(opens_links(command));
        assert!(!opens_links(control));
    } else {
        assert!(opens_links(control));
        assert!(!opens_links(command));
    }
}

#[test]
fn a_plain_click_never_opens_anything() {
    assert!(!opens_links(Modifiers::NONE));
    assert!(!opens_links(Modifiers::SHIFT));
    assert!(!opens_links(Modifiers::ALT));
}
