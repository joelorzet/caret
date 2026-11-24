// Each integration test binary compiles this separately, so a helper can be
// unused in some of them.
#![allow(dead_code)]

/// Reports a skip, and refuses to skip on a build runner.
///
/// A machine with no fonts installed is a legitimate state for a laptop and a
/// fair reason to pass on the tests that need one. It is not a legitimate
/// state for CI: a test that goes green because there was nothing to test is
/// worse than one that fails.
pub fn skipping(reason: &str) {
    assert!(
        std::env::var_os("CI").is_none(),
        "{reason}: a CI run must have fonts installed"
    );
    eprintln!("{reason}; skipping");
}
