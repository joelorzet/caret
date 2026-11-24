use caret_app::{marked_target, MarkedLink};

/// A target a program marked, as it would arrive from the escape.
fn url(target: &str) -> Option<String> {
    match marked_target(target) {
        Some(MarkedLink::Url(url)) => Some(url),
        _ => None,
    }
}

fn path(target: &str) -> Option<String> {
    match marked_target(target) {
        Some(MarkedLink::Path(path)) => Some(path),
        _ => None,
    }
}

#[test]
fn the_web_schemes_are_opened() {
    assert_eq!(
        url("https://example.com/build/7"),
        Some("https://example.com/build/7".into())
    );
    assert_eq!(url("http://example.com"), Some("http://example.com".into()));
    assert_eq!(
        url("mailto:someone@example.com"),
        Some("mailto:someone@example.com".into())
    );
}

#[test]
fn a_scheme_written_in_capitals_is_the_same_scheme() {
    assert_eq!(
        url("HTTPS://example.com/x"),
        Some("HTTPS://example.com/x".into()),
        "the target is passed on as it was given"
    );
}

// The marked text can read "https://my-bank.example" and point anywhere, so
// what the target says is the only thing that decides.
#[test]
fn a_scheme_we_do_not_know_is_left_as_text() {
    for target in [
        "javascript:alert(1)",
        "data:text/html,<script>x</script>",
        "vscode://file/etc/passwd",
        "ssh://elsewhere.example",
        "ftp://example.com/x",
        "tel:+15551234",
        "/etc/passwd",
        "example.com",
    ] {
        assert!(
            marked_target(target).is_none(),
            "{target:?} is not ours to open"
        );
    }
}

#[test]
fn a_scheme_with_nothing_after_it_is_not_a_link() {
    for target in ["https://", "http://", "mailto:", "", "file://"] {
        assert!(marked_target(target).is_none(), "{target:?}");
    }
}

#[test]
fn a_local_file_target_becomes_a_path() {
    assert_eq!(path("file:///etc/hosts"), Some("/etc/hosts".into()));
    assert_eq!(
        path("file://localhost/etc/hosts"),
        Some("/etc/hosts".into()),
        "localhost is this machine"
    );
}

// Another machine's path may well exist here too, and would be a different
// file: opening it would show the wrong thing with no sign of it.
#[test]
fn a_file_on_another_machine_is_not_opened() {
    assert!(marked_target("file://build-07.example/etc/hosts").is_none());
    assert!(marked_target("file://10.0.0.4/home/someone/notes").is_none());
}

#[test]
fn a_file_path_arrives_as_the_filesystem_spells_it() {
    assert_eq!(
        path("file:///tmp/a%20file%20with%20spaces.txt"),
        Some("/tmp/a file with spaces.txt".into())
    );
    assert_eq!(
        path("file:///tmp/caf%C3%A9/notes.md"),
        Some("/tmp/café/notes.md".into()),
        "a percent escape is bytes, not characters"
    );
    assert_eq!(
        path("file:///tmp/100%/x"),
        Some("/tmp/100%/x".into()),
        "a percent that escapes nothing is itself"
    );
}
