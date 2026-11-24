mod support;

use caret_render::FontLibrary;

#[test]
fn an_empty_library_reports_nothing_rather_than_guessing() {
    let library = FontLibrary::empty();
    assert_eq!(library.face_count(), 0);
    assert!(library.families().is_empty());
    assert!(library.monospaced().is_empty());
    assert!(!library.has_family("Menlo"));
    assert!(library.faces_of("Menlo").is_empty());
    assert!(library.regular_face("Menlo").is_none());
    assert!(library.resolve("Menlo", &["Courier".to_string()]).is_none());
}

#[test]
fn searching_an_empty_library_finds_nothing() {
    let library = FontLibrary::empty();
    assert!(library.search("mono").is_empty());
    assert!(library.search("").is_empty());
}

// The rest needs real fonts. Systems without any are a legitimate state, so
// this reports and returns instead of failing.
#[test]
fn the_system_library_describes_its_families() {
    let library = FontLibrary::load();
    if library.face_count() == 0 {
        support::skipping("no system fonts installed");
        return;
    }

    let families = library.families();
    assert!(!families.is_empty());
    assert!(families.iter().all(|f| !f.name.is_empty()));
    assert!(families.iter().all(|f| f.faces >= 1));

    // Monospaced families sort first, because they are the only ones a
    // terminal can use.
    let first_proportional = families.iter().position(|f| !f.monospaced);
    let last_monospaced = families.iter().rposition(|f| f.monospaced);
    if let (Some(first), Some(last)) = (first_proportional, last_monospaced) {
        assert!(last < first, "monospaced families must come first");
    }

    // Names are unique.
    let mut names: Vec<&str> = families.iter().map(|f| f.name.as_str()).collect();
    let before = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), before);
}

#[test]
fn a_known_family_resolves_to_a_regular_upright_face() {
    let library = FontLibrary::load();
    let Some(family) = library.monospaced().first().map(|f| f.name.clone()) else {
        support::skipping("no monospaced fonts installed");
        return;
    };

    assert!(library.has_family(&family));
    assert!(
        library.has_family(&family.to_uppercase()),
        "lookup ignores case"
    );

    let face = library.regular_face(&family).expect("a regular face");
    assert_eq!(face.family, family);
    assert!(!face.slant.is_slanted());
    assert!(!library.faces_of(&family).is_empty());
}

#[test]
fn resolve_falls_through_to_the_first_installed_fallback() {
    let library = FontLibrary::load();
    let Some(family) = library.monospaced().first().map(|f| f.name.clone()) else {
        support::skipping("no monospaced fonts installed");
        return;
    };

    let face = library
        .resolve("No Such Font At All", std::slice::from_ref(&family))
        .expect("the fallback should be used");
    assert_eq!(face.family, family);

    assert!(library
        .resolve("No Such Font", &["Also Missing".to_string()])
        .is_none());
}

#[test]
fn search_narrows_by_name() {
    let library = FontLibrary::load();
    let Some(family) = library.families().first().map(|f| f.name.clone()) else {
        support::skipping("no system fonts installed");
        return;
    };

    let hits = library.search(&family);
    assert!(hits.iter().any(|f| f.name == family));
    assert!(library.search("zzz-no-such-font-zzz").is_empty());
}
