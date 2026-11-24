mod support;

use caret_render::{FontLibrary, GlyphCache, GlyphStyle};
use support::skipping;

fn cache_for(family: &str) -> Option<GlyphCache> {
    let library = FontLibrary::load();
    GlyphCache::new(&library, family, &[], 14.0, 1.2, 0.0)
}

fn any_monospaced() -> Option<GlyphCache> {
    let library = FontLibrary::load();
    let family = library.monospaced().first().map(|f| f.name.clone())?;
    GlyphCache::new(&library, &family, &[], 14.0, 1.2, 0.0)
}

/// A family installed here that is known to draw ligatures, if there is one.
///
/// Build runners ship none of these, so the tests that need one say what
/// they could not check rather than passing quietly.
fn ligature_family() -> Option<String> {
    const KNOWN: [&str; 6] = [
        "Cascadia Code",
        "CaskaydiaCove Nerd Font Mono",
        "Fira Code",
        "JetBrains Mono",
        "Iosevka",
        "Monaspace Neon",
    ];
    let library = FontLibrary::load();
    KNOWN
        .into_iter()
        .find(|name| library.resolve(name, &[]).is_some())
        .map(|name| name.to_string())
}

// The gate that keeps shaping off ordinary text, which is most of it.
#[test]
fn a_run_of_letters_is_not_worth_shaping() {
    assert!(!GlyphCache::may_ligate("the quick brown fox"));
    assert!(!GlyphCache::may_ligate("Cargo"));
    assert!(!GlyphCache::may_ligate(""));
    assert!(!GlyphCache::may_ligate("1234567890"));
}

#[test]
fn a_run_with_the_punctuation_ligatures_are_made_of_is_offered() {
    for text in ["!=", "=>", "->", "a != b", "/* x */", "<=>", "::"] {
        assert!(GlyphCache::may_ligate(text), "{text:?}");
    }
}

// A font with nothing to join must answer plainly, so the painter stays on
// its per-cell path instead of walking pieces that say nothing.
#[test]
fn a_font_without_ligatures_reports_none() {
    let Some(mut cache) = any_monospaced() else {
        return skipping("no monospaced font installed");
    };
    if ligature_family().is_some_and(|family| {
        FontLibrary::load()
            .resolve(&family, &[])
            .is_some_and(|resolved| cache_for(&family).is_some() && resolved.family == family)
    }) {
        // The first monospaced family on this machine might itself have
        // ligatures, in which case this is not the test for it.
        if cache.pieces("!=", GlyphStyle::Regular).is_some() {
            return;
        }
    }
    assert!(cache.pieces("!=", GlyphStyle::Regular).is_none());
}

#[test]
fn asking_twice_gives_the_same_answer() {
    let Some(mut cache) = any_monospaced() else {
        return skipping("no monospaced font installed");
    };
    let first = cache.pieces("!=", GlyphStyle::Regular).map(<[_]>::to_vec);
    let again = cache.pieces("!=", GlyphStyle::Regular).map(<[_]>::to_vec);
    assert_eq!(first, again, "the second answer comes from the cache");
}

// Fonts join characters two ways. Some substitute one wide glyph for the
// run; Cascadia Code substitutes each character for a part of the joined
// shape, so the run stays one glyph per cell and the glyphs change. Either
// way the pieces have to cover exactly the cells the characters took, or
// they could not be laid out in a grid.
#[test]
fn a_ligature_font_draws_the_operators_programmers_use_its_own_way() {
    let Some(family) = ligature_family() else {
        eprintln!("no ligature font installed; skipping the shaped cases");
        return;
    };
    let mut cache = cache_for(&family).expect("the family loads");

    for text in ["!=", "=>", "->"] {
        let pieces = cache
            .pieces(text, GlyphStyle::Regular)
            .unwrap_or_else(|| panic!("{family} draws {text:?} its own way"))
            .to_vec();
        assert_eq!(
            pieces.iter().map(|piece| piece.cells).sum::<u16>(),
            text.chars().count() as u16,
            "{text:?} covers exactly its own cells: {pieces:?}"
        );
        assert!(
            pieces.iter().all(|piece| piece.cells >= 1),
            "{text:?} has no empty piece: {pieces:?}"
        );
    }
}

#[test]
fn a_joined_glyph_is_drawn_differently_from_the_character_alone() {
    let Some(family) = ligature_family() else {
        eprintln!("no ligature font installed; skipping the shaped cases");
        return;
    };
    let mut cache = cache_for(&family).expect("the family loads");
    let pieces = cache
        .pieces("!=", GlyphStyle::Regular)
        .map(<[_]>::to_vec)
        .unwrap_or_else(|| panic!("{family} draws != its own way"));

    let alone = cache.glyph('!', GlyphStyle::Regular).clone();
    let joined = cache
        .shaped_glyph(pieces[0].glyph, GlyphStyle::Regular)
        .clone();

    assert!(joined.width > 0 && joined.height > 0, "the piece has ink");
    assert!(
        joined.coverage != alone.coverage || joined.width != alone.width,
        "the first half of != is not the bare exclamation mark"
    );
}

#[test]
fn a_run_of_letters_is_left_alone_even_by_a_ligature_font() {
    let Some(family) = ligature_family() else {
        eprintln!("no ligature font installed; skipping the shaped cases");
        return;
    };
    let mut cache = cache_for(&family).expect("the family loads");
    assert!(cache.pieces("hello", GlyphStyle::Regular).is_none());
}
