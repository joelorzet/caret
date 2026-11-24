use crate::support::{kind_of, tokens};
use caret_features::{Language, TokenKind};

#[test]
fn a_name_before_a_bracket_is_a_call() {
    assert_eq!(
        kind_of(Language::Rust, "let n = compute(x);", "compute"),
        Some(TokenKind::Function)
    );
    assert_eq!(
        kind_of(Language::Python, "result = helper (a)", "helper"),
        Some(TokenKind::Function),
        "whitespace before the bracket still counts"
    );
}

#[test]
fn a_capitalised_name_is_a_type() {
    assert_eq!(
        kind_of(Language::Rust, "let v: Vec<u8> = read();", "Vec"),
        Some(TokenKind::Type)
    );
    assert_eq!(kind_of(Language::Rust, "let value = 1;", "value"), None);
}

#[test]
fn a_key_at_the_start_of_a_line_is_a_property() {
    assert_eq!(
        kind_of(Language::Toml, "version = \"0.2.32\"", "version"),
        Some(TokenKind::Property)
    );
    assert_eq!(
        kind_of(Language::Yaml, "  name: build", "name"),
        Some(TokenKind::Property),
        "indentation does not stop it being first"
    );
    // Mid-line it is just a name.
    assert_eq!(kind_of(Language::Toml, "a = b = c", "b"), None);
}

#[test]
fn keywords_still_win_over_the_shape_rules() {
    assert_eq!(
        kind_of(Language::Rust, "if (x) { }", "if"),
        Some(TokenKind::Keyword),
        "not a call, even though a bracket follows"
    );
}

// A TOML file should come out with several colours, not one.
#[test]
fn a_config_line_gets_more_than_one_colour() {
    let line = "version = \"0.2.32\" # pinned";
    let found: Vec<TokenKind> = tokens(Language::Toml, line)
        .into_iter()
        .map(|t| t.kind)
        .collect();
    assert!(found.contains(&TokenKind::Property), "{found:?}");
    assert!(found.contains(&TokenKind::Text), "{found:?}");
    assert!(found.contains(&TokenKind::Comment), "{found:?}");
}

#[test]
fn a_rust_line_gets_the_full_range() {
    let line = "pub fn load(path: &Path) -> Option<Font> { 42 }";
    let found: Vec<TokenKind> = tokens(Language::Rust, line)
        .into_iter()
        .map(|t| t.kind)
        .collect();
    for wanted in [
        TokenKind::Keyword,
        TokenKind::Function,
        TokenKind::Type,
        TokenKind::Number,
    ] {
        assert!(found.contains(&wanted), "{wanted:?} missing from {found:?}");
    }
}
