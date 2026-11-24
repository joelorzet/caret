mod support;

use caret_render::{FontLibrary, GlyphCache, GlyphStyle};

fn cache() -> Option<GlyphCache> {
    let library = FontLibrary::load();
    let family = library.monospaced().first().map(|f| f.name.clone())?;
    GlyphCache::new(&library, &family, &[], 14.0, 1.2, 0.0)
}

#[test]
fn style_selection_covers_the_combinations() {
    assert_eq!(GlyphStyle::of(false, false), GlyphStyle::Regular);
    assert_eq!(GlyphStyle::of(true, false), GlyphStyle::Bold);
    assert_eq!(GlyphStyle::of(false, true), GlyphStyle::Italic);
    assert_eq!(GlyphStyle::of(true, true), GlyphStyle::BoldItalic);
}

// A family without an italic face must still draw italic text.
#[test]
fn every_style_falls_back_to_regular() {
    for style in [
        GlyphStyle::Regular,
        GlyphStyle::Bold,
        GlyphStyle::Italic,
        GlyphStyle::BoldItalic,
    ] {
        let chain = style.fallbacks();
        assert_eq!(chain.first(), Some(&style), "{style:?} tries itself first");
        assert_eq!(
            chain.last(),
            Some(&GlyphStyle::Regular),
            "{style:?} ends at regular"
        );
    }
}

#[test]
fn an_unknown_family_loads_nothing() {
    let library = FontLibrary::load();
    assert!(GlyphCache::new(&library, "No Such Font", &[], 14.0, 1.2, 0.0).is_none());
}

#[test]
fn a_monospaced_family_gives_sane_cell_metrics() {
    let Some(cache) = cache() else {
        support::skipping("no monospaced fonts installed");
        return;
    };
    let m = cache.metrics();
    assert!(m.width > 1.0 && m.width < 60.0, "cell width {}", m.width);
    assert!(m.height > m.width, "a cell is taller than it is wide");
    assert!(m.ascent > 0.0 && m.descent >= 0.0);
    assert!(cache.has_style(GlyphStyle::Regular));
}

#[test]
fn glyphs_rasterise_and_are_kept() {
    let Some(mut cache) = cache() else {
        support::skipping("no monospaced fonts installed");
        return;
    };
    assert_eq!(cache.cached(), 0);

    let glyph = cache.glyph('M', GlyphStyle::Regular).clone();
    assert!(glyph.width > 0 && glyph.height > 0, "M has a bitmap");
    assert_eq!(glyph.coverage.len(), glyph.width * glyph.height);
    assert!(glyph.coverage.iter().any(|&a| a > 0), "and some ink in it");

    assert_eq!(cache.cached(), 1);
    let again = cache.glyph('M', GlyphStyle::Regular).clone();
    assert_eq!(again, glyph, "the same raster comes back");
    assert_eq!(cache.cached(), 1, "it was not rasterised twice");

    cache.glyph('M', GlyphStyle::Bold);
    assert_eq!(cache.cached(), 2, "a different style is its own entry");
}

#[test]
fn a_space_has_no_ink() {
    let Some(mut cache) = cache() else { return };
    let glyph = cache.glyph(' ', GlyphStyle::Regular);
    assert!(glyph.coverage.iter().all(|&a| a == 0));
}

#[test]
fn a_character_no_face_has_returns_a_blank_rather_than_failing() {
    let Some(mut cache) = cache() else { return };
    let glyph = cache.glyph('\u{10FFFF}', GlyphStyle::Regular).clone();
    assert_eq!(glyph.coverage.len(), glyph.width * glyph.height);
}

#[test]
fn a_larger_size_gives_a_larger_cell() {
    let library = FontLibrary::load();
    let Some(family) = library.monospaced().first().map(|f| f.name.clone()) else {
        return;
    };
    let small = GlyphCache::new(&library, &family, &[], 10.0, 1.0, 0.0).unwrap();
    let large = GlyphCache::new(&library, &family, &[], 20.0, 1.0, 0.0).unwrap();
    assert!(large.metrics().width > small.metrics().width);
    assert!(large.metrics().height > small.metrics().height);
}

// fontdue's own cmap lookup returns the wrong glyph for several Latin-1
// characters in at least Menlo: `·`, `é`, `ü` and `°` all came back as
// something else, so `·` rendered as `∑`. Lookups go through ttf-parser now.
#[test]
fn distinct_characters_get_distinct_glyphs() {
    let Some(mut cache) = cache() else {
        support::skipping("no monospaced fonts installed");
        return;
    };

    let shape = |cache: &mut GlyphCache, ch: char| {
        let glyph = cache.glyph(ch, GlyphStyle::Regular);
        (
            glyph.width,
            glyph.height,
            glyph.coverage.iter().filter(|a| **a > 0).count(),
        )
    };

    for (small, large) in [('·', '∑'), ('.', 'M')] {
        let a = shape(&mut cache, small);
        let b = shape(&mut cache, large);
        assert_ne!(a, b, "{small:?} and {large:?} rendered identically");
        assert!(a.0 < b.0, "{small:?} should be narrower than {large:?}");
    }

    // Accented letters are their own glyphs, not the unaccented ones.
    for (plain, accented) in [('e', 'é'), ('u', 'ü')] {
        let a = shape(&mut cache, plain);
        let b = shape(&mut cache, accented);
        assert_ne!(a, b, "{accented:?} rendered as {plain:?}");
        assert!(b.1 >= a.1, "{accented:?} is at least as tall as {plain:?}");
    }
}
