use crate::support::{kinds, text, tokens};
use caret_features::{Language, TokenKind};

#[test]
fn keywords_are_picked_out() {
    let line = "pub fn main() {";
    let found: Vec<String> = tokens(Language::Rust, line)
        .into_iter()
        .filter(|t| t.kind == TokenKind::Keyword)
        .map(|t| text(line, t))
        .collect();
    assert_eq!(found, ["pub", "fn"]);
}

// `function` contains `fn`, and `iffy` starts with `if`: neither is a keyword.
#[test]
fn a_keyword_inside_a_word_is_not_one() {
    assert!(kinds(Language::Rust, "iffy infn structure").is_empty());
}

#[test]
fn line_comments_run_to_the_end() {
    let line = "let x = 1; // the rest of this is a comment";
    let comment = tokens(Language::Rust, line)
        .into_iter()
        .find(|t| t.kind == TokenKind::Comment)
        .expect("a comment");
    assert_eq!(text(line, comment), "// the rest of this is a comment");
}

#[test]
fn strings_are_found_and_escapes_do_not_end_them() {
    let line = r#"let s = "a \" b"; let t = "c";"#;
    let found: Vec<String> = tokens(Language::Rust, line)
        .into_iter()
        .filter(|t| t.kind == TokenKind::Text)
        .map(|t| text(line, t))
        .collect();
    assert_eq!(found, [r#""a \" b""#, r#""c""#]);
}

#[test]
fn an_unterminated_string_stops_at_the_line_end() {
    let line = "let s = \"never closed";
    let token = tokens(Language::Rust, line)
        .into_iter()
        .find(|t| t.kind == TokenKind::Text)
        .unwrap();
    assert_eq!(token.end, line.chars().count());
}

#[test]
fn a_keyword_inside_a_string_is_left_alone() {
    let line = r#"let s = "fn struct impl";"#;
    let keywords = tokens(Language::Rust, line)
        .into_iter()
        .filter(|t| t.kind == TokenKind::Keyword)
        .count();
    assert_eq!(keywords, 1, "only the real `let`");
}

#[test]
fn numbers_are_found() {
    let line = "let x = 42; let y = 3.14;";
    let found: Vec<String> = tokens(Language::Rust, line)
        .into_iter()
        .filter(|t| t.kind == TokenKind::Number)
        .map(|t| text(line, t))
        .collect();
    assert_eq!(found, ["42", "3.14"]);
}

#[test]
fn tokens_are_in_order_and_do_not_overlap() {
    let line = r#"pub fn f() { let s = "x"; /* c */ return 1; } // done"#;
    let found = tokens(Language::Rust, line);
    let mut previous = 0;
    for token in &found {
        assert!(token.start >= previous, "{found:?}");
        assert!(token.end > token.start);
        assert!(token.end <= line.chars().count());
        previous = token.end;
    }
}

#[test]
fn offsets_are_characters_not_bytes() {
    // The accented word is multi-byte; the keyword after it must still line up.
    let line = "// é é é";
    let token = tokens(Language::Rust, line)[0];
    assert_eq!(token.start, 0);
    assert_eq!(token.end, line.chars().count());
}
