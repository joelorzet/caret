use caret_update::{hash_of, pick_asset};

/// What a release actually publishes, as its own checksums list it. Taken
/// from v0.1.0 rather than made up, because the point of these tests is
/// that nothing here depends on guessing the names right.
const LISTING: &str = "\
3cb2f7  caret-0.1.0-linux-x86_64.tar.gz
9ac41e  caret-0.1.0-macos-arm64.app.zip
71d0b8  caret-0.1.0-macos-arm64.dmg
e5f207  caret-0.1.0-macos-x86_64.app.zip
AA12FF *caret-0.1.0-macos-x86_64.dmg
";

#[test]
fn each_machine_is_handed_the_build_that_is_for_it() {
    assert_eq!(
        pick_asset(LISTING, "macos", "aarch64").as_deref(),
        Some("caret-0.1.0-macos-arm64.dmg")
    );
    assert_eq!(
        pick_asset(LISTING, "macos", "x86_64").as_deref(),
        Some("caret-0.1.0-macos-x86_64.dmg")
    );
    assert_eq!(
        pick_asset(LISTING, "linux", "x86_64").as_deref(),
        Some("caret-0.1.0-linux-x86_64.tar.gz")
    );
}

// macOS publishes an image and a zipped bundle. The image is the one to
// hand somebody, and the zip is there for anyone who wants the bundle.
#[test]
fn the_disk_image_is_preferred_over_the_zipped_bundle() {
    let picked = pick_asset(LISTING, "macos", "aarch64").expect("a build");
    assert!(picked.ends_with(".dmg"), "{picked}");
}

#[test]
fn a_machine_with_no_build_is_told_so_rather_than_handed_the_wrong_one() {
    assert!(pick_asset(LISTING, "linux", "aarch64").is_none());
    assert!(pick_asset(LISTING, "windows", "x86_64").is_none());
    assert!(pick_asset(LISTING, "macos", "powerpc").is_none());
    assert!(pick_asset("", "macos", "aarch64").is_none());
}

// The names are whatever the release workflow called them. Renaming the
// files, or publishing a build for something new, needs no change here as
// long as the name still says which machine it is for.
#[test]
fn a_renamed_release_is_still_understood() {
    let renamed = "\
111  caret_2.0.0_linux_x86_64.tar.gz
222  caret_2.0.0_macos_arm64.dmg
";
    assert_eq!(
        pick_asset(renamed, "macos", "aarch64").as_deref(),
        Some("caret_2.0.0_macos_arm64.dmg")
    );
    assert_eq!(
        pick_asset(renamed, "linux", "x86_64").as_deref(),
        Some("caret_2.0.0_linux_x86_64.tar.gz")
    );
}

#[test]
fn a_checksum_is_found_by_the_name_beside_it() {
    assert_eq!(
        hash_of(LISTING, "caret-0.1.0-macos-arm64.dmg").as_deref(),
        Some("71d0b8")
    );
    // Some tools mark a binary file with a star, and case is not a
    // difference that matters in a hash.
    assert_eq!(
        hash_of(LISTING, "caret-0.1.0-macos-x86_64.dmg").as_deref(),
        Some("aa12ff")
    );
    assert!(hash_of(LISTING, "caret-9.9.9-macos-arm64.dmg").is_none());
    assert!(hash_of("", "anything").is_none());
}

/// One macOS runner builds both architectures into one binary, so a
/// release may name a file for the platform and say nothing about the
/// architecture at all.
const UNIVERSAL: &str = "\
3cb2f7  caret-0.2.0-linux-x86_64.tar.gz
9ac41e  caret-0.2.0-macos.app.zip
71d0b8  caret-0.2.0-macos.dmg
";

#[test]
fn a_build_that_names_no_architecture_is_for_every_architecture() {
    for arch in ["aarch64", "x86_64"] {
        assert_eq!(
            pick_asset(UNIVERSAL, "macos", arch).as_deref(),
            Some("caret-0.2.0-macos.dmg"),
            "{arch}"
        );
    }
}

// A release that publishes both kinds hands over the one built for this
// machine, not the one that merely fits it.
#[test]
fn a_named_architecture_is_preferred_over_a_universal_build() {
    let both = "\
111  caret-0.2.0-macos.dmg
222  caret-0.2.0-macos-arm64.dmg
";
    assert_eq!(
        pick_asset(both, "macos", "aarch64").as_deref(),
        Some("caret-0.2.0-macos-arm64.dmg")
    );
    assert_eq!(
        pick_asset(both, "macos", "x86_64").as_deref(),
        Some("caret-0.2.0-macos.dmg"),
        "and falls back when its own is not there"
    );
}
