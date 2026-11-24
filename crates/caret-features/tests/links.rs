use caret_features::{link_at, links_in, LinkTarget};

fn url(line: &str, column: usize) -> String {
    match link_at(line, column).map(|link| link.target) {
        Some(LinkTarget::Url(url)) => url,
        other => panic!("wanted a url, got {other:?}"),
    }
}

fn file(line: &str, column: usize) -> (String, Option<u32>, Option<u16>) {
    match link_at(line, column).map(|link| link.target) {
        Some(LinkTarget::File { path, line, column }) => (path, line, column),
        other => panic!("wanted a file, got {other:?}"),
    }
}

#[test]
fn a_url_is_found_wherever_in_it_you_click() {
    let line = "see https://example.com/thing for more";
    // Four characters of "see ", then twenty-five of the link.
    for column in 4..29 {
        assert_eq!(
            url(line, column),
            "https://example.com/thing",
            "at {column}"
        );
    }
    assert!(
        link_at(line, 0).is_none(),
        "the word before it is not a link"
    );
    assert!(link_at(line, 31).is_none(), "nor the word after");
}

// A link at the end of a sentence does not own the full stop, and one in
// brackets does not own the bracket.
#[test]
fn punctuation_around_a_url_is_not_part_of_it() {
    assert_eq!(
        url("read https://example.com/x.", 10),
        "https://example.com/x"
    );
    assert_eq!(
        url("read (https://example.com/y)", 10),
        "https://example.com/y"
    );
    assert_eq!(
        url("read \"https://example.com/z\"", 10),
        "https://example.com/z"
    );
}

// What every compiler, linter and test runner prints.
#[test]
fn a_path_carries_the_place_in_it() {
    assert_eq!(
        file("error at crates/caret-core/src/lib.rs:42:9: oh dear", 12),
        (
            "crates/caret-core/src/lib.rs".to_string(),
            Some(42),
            Some(9)
        )
    );
    assert_eq!(
        file("see src/main.rs:17 for the rest", 6),
        ("src/main.rs".to_string(), Some(17), None)
    );
    assert_eq!(
        file("./scripts/bundle-mac.sh --install", 2),
        ("./scripts/bundle-mac.sh".to_string(), None, None)
    );
}

// A word is not a path, however much it looks like one on its own.
#[test]
fn a_plain_word_is_not_a_link() {
    assert!(link_at("cargo test --workspace", 0).is_none());
    assert!(link_at("cargo test --workspace", 6).is_none());
    assert!(
        link_at("README.md is a file", 0).is_none(),
        "no separator in it"
    );
}

#[test]
fn several_links_on_one_line_are_all_found() {
    let line = "see src/a.rs:1 and https://example.com/b and src/c.rs";
    let found = links_in(line);
    assert_eq!(found.len(), 3, "{found:?}");
    assert!(matches!(found[1].target, LinkTarget::Url(_)));
}

#[test]
fn nothing_in_an_empty_line_and_nothing_out_of_range() {
    assert!(links_in("").is_empty());
    assert!(link_at("https://example.com", 500).is_none());
}
