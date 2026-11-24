// A session reads its configuration from the environment, so these point
// it at a directory of their own. Setting an environment variable is
// process wide, which is why this is a file to itself.

use caret_app::Session;

/// A session with nothing but the defaults behind it.
fn session() -> Session {
    let root = std::env::temp_dir().join(format!("caret-windows-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("a config directory");
    std::env::set_var("CARET_CONFIG_DIR", &root);
    Session::new().expect("a session")
}

fn press(spec: &str) -> caret_config::KeyBinding {
    caret_config::KeyBinding::parse(spec).expect("a binding")
}

#[test]
fn a_window_is_asked_for_once_and_then_the_request_is_gone() {
    let mut session = session();
    assert!(!session.took_window_request(), "nothing asked for yet");

    let _ = session.handle_key(caret_app::types::key_press::KeyPress {
        binding: press("cmd+n"),
        bytes: Vec::new(),
    });

    assert!(session.took_window_request(), "the window was asked for");
    assert!(
        !session.took_window_request(),
        "and asking again would open a second one"
    );
    assert!(
        session.is_running(),
        "asking for a window changes nothing else"
    );
    assert!(!session.is_quitting());
}

// Two different endings: one closes a window, the other ends the program.
// The window layer cannot tell them apart from `is_running` alone.
#[test]
fn running_out_of_panes_is_not_the_same_as_quitting() {
    let mut closing = session();
    let _ = closing.handle_key(caret_app::types::key_press::KeyPress {
        binding: press("cmd+W"),
        bytes: Vec::new(),
    });

    assert!(!closing.is_running(), "the last tab closed");
    assert!(
        !closing.is_quitting(),
        "which closes a window, not the program"
    );

    let mut quitting = session();
    let _ = quitting.handle_key(caret_app::types::key_press::KeyPress {
        binding: press("cmd+q"),
        bytes: Vec::new(),
    });
    assert!(!quitting.is_running());
    assert!(quitting.is_quitting(), "quitting ends everything");
}

// A second window starts where the pane it was asked from is, the way a
// new split does.
#[test]
fn a_second_session_starts_in_the_directory_it_was_given() {
    let first = session();
    let somewhere = std::env::temp_dir().canonicalize().expect("a real path");
    let second = Session::beside(&first, Some(somewhere.clone())).expect("a second session");

    assert!(second.is_running());
    assert!(!second.is_quitting());
    let found = second.working_dir();
    assert_eq!(found.as_deref(), Some(somewhere.as_path()), "{found:?}");
}
