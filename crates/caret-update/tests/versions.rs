use caret_update::Version;

fn v(text: &str) -> Version {
    Version::parse(text).unwrap_or_else(|| panic!("{text} should parse"))
}

#[test]
fn a_version_reads_with_or_without_its_v() {
    assert_eq!(v("0.1.1"), v("v0.1.1"));
    assert_eq!(v("1.20.300").to_string(), "1.20.300");
}

#[test]
fn anything_that_is_not_three_numbers_is_not_a_version() {
    for text in ["", "v", "1", "1.2", "1.2.3.4", "1.2.x", "latest", "v1.2.-3"] {
        assert!(Version::parse(text).is_none(), "{text}");
    }
}

#[test]
fn newer_means_newer_in_the_usual_order() {
    assert!(v("0.2.0").is_newer_than(&v("0.1.9")));
    assert!(v("0.1.10").is_newer_than(&v("0.1.9")));
    assert!(v("1.0.0").is_newer_than(&v("0.99.99")));
    assert!(!v("0.1.0").is_newer_than(&v("0.1.0")));
    assert!(!v("0.1.0").is_newer_than(&v("0.1.1")));
}

// A release candidate comes before the release it is named for, which is
// what semantic versioning says and what the tags would look like if a 1.0
// were ever worth rehearsing.
#[test]
fn a_prerelease_comes_before_the_release_it_is_for() {
    assert!(v("0.2.0").is_newer_than(&v("0.2.0-rc1")));
    assert!(v("0.2.0-rc2").is_newer_than(&v("0.2.0-rc1")));
    assert!(!v("0.2.0-rc1").is_newer_than(&v("0.2.0")));
    assert_eq!(v("0.2.0-rc1").to_string(), "0.2.0-rc1");
}
