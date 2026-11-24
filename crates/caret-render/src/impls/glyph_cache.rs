#[allow(unused_imports)]
use crate::prelude::*;

use std::collections::HashMap;

impl GlyphCache {
    /// Loads a family at `size` points, taking whichever of its faces exist.
    pub fn new(
        library: &FontLibrary,
        family: &str,
        fallbacks: &[String],
        size: f32,
        line_height: f32,
        letter_spacing: f32,
    ) -> Option<GlyphCache> {
        let resolved = library.resolve(family, fallbacks)?;
        let mut faces = HashMap::new();

        for face in library.faces_of(&resolved.family) {
            let bold = face.weight >= 600;
            let style = GlyphStyle::of(bold, face.slant.is_slanted());
            // Keep the first face of each style; later ones are extra weights.
            if faces.contains_key(&style) {
                continue;
            }
            if let Some(font) = load(&face, size) {
                faces.insert(style, font);
            }
        }
        if !faces.contains_key(&GlyphStyle::Regular) {
            return None;
        }

        let regular = &faces[&GlyphStyle::Regular].font;
        let line = regular.horizontal_line_metrics(size)?;
        // Every column is the same width in a terminal, so one representative
        // advance decides it rather than each glyph's own.
        let advance = regular.metrics('M', size).advance_width.max(1.0);
        let metrics = CellMetrics::new(
            advance,
            line.ascent,
            -line.descent,
            line_height,
            letter_spacing,
        );

        Some(GlyphCache {
            faces,
            size,
            metrics,
            glyphs: HashMap::new(),
            blank: Glyph {
                width: 0,
                height: 0,
                left: 0,
                top: 0,
                coverage: Vec::new(),
            },
        })
    }

    pub fn metrics(&self) -> CellMetrics {
        self.metrics
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn styles(&self) -> usize {
        self.faces.len()
    }

    pub fn has_style(&self, style: GlyphStyle) -> bool {
        self.faces.contains_key(&style)
    }

    pub fn cached(&self) -> usize {
        self.glyphs.len()
    }

    /// The glyph for `ch`, rasterising it the first time it is asked for.
    pub fn glyph(&mut self, ch: char, style: GlyphStyle) -> &Glyph {
        let key = (ch, style);
        if !self.glyphs.contains_key(&key) {
            let raster = self.rasterise(ch, style);
            self.glyphs.insert(key, raster);
        }
        &self.glyphs[&key]
    }

    fn rasterise(&self, ch: char, style: GlyphStyle) -> Glyph {
        for candidate in style.fallbacks() {
            let Some(face) = self.faces.get(candidate) else {
                continue;
            };
            let Some(index) = face.glyph_index(ch) else {
                // A space has no outline but still needs its cell.
                if ch == ' ' {
                    return self.blank.clone();
                }
                continue;
            };

            let (metrics, coverage) = face.font.rasterize_indexed(index, self.size);
            return Glyph {
                width: metrics.width,
                height: metrics.height,
                left: metrics.xmin,
                top: metrics.height as i32 + metrics.ymin,
                coverage,
            };
        }
        self.blank.clone()
    }
}

fn load(descriptor: &FontFace, size: f32) -> Option<LoadedFace> {
    let path = descriptor.path.as_ref()?;
    let bytes = std::sync::Arc::new(std::fs::read(path).ok()?);
    let settings = fontdue::FontSettings {
        collection_index: descriptor.index,
        scale: size.max(1.0),
        ..fontdue::FontSettings::default()
    };
    let font = fontdue::Font::from_bytes(bytes.as_slice(), settings).ok()?;
    Some(LoadedFace {
        font,
        bytes,
        index: descriptor.index,
    })
}

impl LoadedFace {
    /// The glyph for a character, resolved through the font's own cmap.
    pub(crate) fn glyph_index(&self, ch: char) -> Option<u16> {
        let face = ttf_parser::Face::parse(self.bytes.as_slice(), self.index).ok()?;
        face.glyph_index(ch).map(|id| id.0).filter(|id| *id != 0)
    }
}
