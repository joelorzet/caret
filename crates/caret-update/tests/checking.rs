use caret_update::{check, Outcome, Release, Version, RELEASES};

fn v(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

fn tag_url(tag: &str) -> String {
    format!("{RELEASES}/tag/{tag}")
}

#[test]
fn a_newer_tag_is_reported_with_its_release() {
    match check(&v("0.1.0"), || Some(tag_url("v0.2.0"))) {
        Outcome::Newer(release) => {
            assert_eq!(release.tag, "v0.2.0");
            assert_eq!(release.version, v("0.2.0"));
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_same_version_is_current_and_so_is_an_older_one() {
    assert_eq!(
        check(&v("0.2.0"), || Some(tag_url("v0.2.0"))),
        Outcome::Current
    );
    assert_eq!(
        check(&v("0.3.0"), || Some(tag_url("v0.2.0"))),
        Outcome::Current
    );
}

// No network, no curl, a redirect to somewhere unexpected: none of it is
// worth interrupting somebody who is trying to use a terminal.
#[test]
fn anything_unanswerable_is_simply_unknown() {
    assert_eq!(check(&v("0.1.0"), || None), Outcome::Unknown);
    for url in [
        String::new(),
        RELEASES.to_string(),
        "https://github.com/login?return_to=%2Fcaret".to_string(),
        tag_url("nightly"),
    ] {
        assert_eq!(
            check(&v("0.1.0"), || Some(url.clone())),
            Outcome::Unknown,
            "{url}"
        );
    }
}

#[test]
fn a_release_names_its_own_downloads() {
    let release = Release::from_redirect(&tag_url("v0.2.0")).expect("a release");
    assert_eq!(
        release.download_url("SHA256SUMS"),
        format!("{RELEASES}/download/v0.2.0/SHA256SUMS")
    );
}

// The address comes from the manifest, so a fork updates from its own
// repository without touching any code.
#[test]
fn the_release_address_is_the_one_the_manifest_declares() {
    assert_eq!(
        RELEASES,
        format!("{}/releases", env!("CARGO_PKG_REPOSITORY"))
    );
}
