use caret_update::{asset_for, hash_of, Version};

fn v(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

// The names the release workflow builds. If the two drift apart, an update
// downloads a 404 page and checks its hash against nothing.
#[test]
fn every_published_platform_has_a_name() {
    assert_eq!(
        asset_for(&v("0.1.1"), "macos", "aarch64").as_deref(),
        Some("caret-0.1.1-macos-arm64.dmg")
    );
    assert_eq!(
        asset_for(&v("0.1.1"), "macos", "x86_64").as_deref(),
        Some("caret-0.1.1-macos-x86_64.dmg")
    );
    assert_eq!(
        asset_for(&v("0.1.1"), "linux", "x86_64").as_deref(),
        Some("caret-0.1.1-linux-x86_64.tar.gz")
    );
}

#[test]
fn a_platform_with_no_build_has_no_name() {
    assert!(asset_for(&v("0.1.1"), "linux", "aarch64").is_none());
    assert!(asset_for(&v("0.1.1"), "windows", "x86_64").is_none());
    assert!(asset_for(&v("0.1.1"), "macos", "powerpc").is_none());
}

#[test]
fn a_checksum_is_found_by_the_name_beside_it() {
    let sums = "\
aaa111  caret-0.1.1-linux-x86_64.tar.gz
BBB222  caret-0.1.1-macos-arm64.dmg
ccc333 *caret-0.1.1-macos-x86_64.dmg
";
    assert_eq!(
        hash_of(sums, "caret-0.1.1-macos-arm64.dmg").as_deref(),
        Some("bbb222"),
        "and case is not a difference that matters"
    );
    // Some tools mark a binary file with a star; the name is still the name.
    assert_eq!(
        hash_of(sums, "caret-0.1.1-macos-x86_64.dmg").as_deref(),
        Some("ccc333")
    );
    assert!(hash_of(sums, "caret-9.9.9-macos-arm64.dmg").is_none());
    assert!(hash_of("", "anything").is_none());
}
