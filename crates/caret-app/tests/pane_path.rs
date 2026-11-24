use caret_app::path_with;
use std::path::Path;

// An app bundle puts nothing on PATH, so a pane started from one cannot run
// `caret` and the terminal's own advice to run `caret update` would be
// wrong. Panes are handed the directory the running binary is in.
#[test]
fn the_running_binary_goes_in_front_of_the_path() {
    let joined = path_with(
        "/usr/bin:/bin",
        Path::new("/Applications/caret.app/Contents/MacOS"),
    )
    .expect("a new path");
    assert_eq!(
        joined,
        "/Applications/caret.app/Contents/MacOS:/usr/bin:/bin"
    );
}

#[test]
fn a_directory_already_there_is_not_added_twice() {
    assert!(path_with("/usr/bin:/opt/caret:/bin", Path::new("/opt/caret")).is_none());
    assert!(path_with("/opt/caret", Path::new("/opt/caret")).is_none());
}

#[test]
fn an_empty_path_still_gets_the_directory() {
    assert_eq!(
        path_with("", Path::new("/opt/caret")).as_deref(),
        Some("/opt/caret:")
    );
}
