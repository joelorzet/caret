#[allow(unused_imports)]
use crate::prelude::*;

/// The characters programming ligatures are built out of.
///
/// A run with none of these cannot ligate, and skipping it is what keeps the
/// cost of this feature off ordinary words. Letters and digits are never
/// part of a ligature in a font meant for code.
const JOINABLE: &str = "=<>-+*/!?:;.#&|~^$@%_\\'\"";

/// How many shaped runs to remember per face.
///
/// A row is made of a handful of runs, and the same rows are drawn again and
/// again, so this is reached only by output that keeps changing shape.
const REMEMBERED: usize = 4_096;

/// The longest run worth shaping in one go. A row is split into runs of one
/// style, and a run longer than this is shaped in parts.
pub const MAX_RUN: usize = 256;

impl GlyphCache {
    /// Whether `text` could possibly ligate, which is cheap to ask and
    /// usually no.
    pub fn may_ligate(text: &str) -> bool {
        text.chars().any(|ch| JOINABLE.contains(ch))
    }

    /// How the font draws `text` with its ligatures applied: one piece per
    /// glyph, each covering the cells its characters took.
    ///
    /// `None` when nothing ligated, which leaves the caller on the ordinary
    /// per-cell path rather than making it walk pieces it does not need.
    pub fn pieces(&mut self, text: &str, style: GlyphStyle) -> Option<&[Piece]> {
        let face = self.face_for(style)?;
        if !self
            .shaped
            .get(&face)
            .is_some_and(|runs| runs.contains_key(text))
        {
            let shaped = self.shape(text, face);
            self.shaped
                .entry(face)
                .or_default()
                .insert(text.to_owned(), shaped);
            self.forget_runs_if_crowded(face);
        }
        self.shaped
            .get(&face)
            .and_then(|runs| runs.get(text))
            .map(|pieces| pieces.as_slice())
            .filter(|pieces| !pieces.is_empty())
    }

    /// The glyph a shaped piece names, rasterised the first time it is
    /// asked for.
    pub fn shaped_glyph(&mut self, glyph: u16, style: GlyphStyle) -> &Glyph {
        let face = self.face_for(style).unwrap_or(GlyphStyle::Regular);
        let key = (glyph, face);
        if !self.by_glyph.contains_key(&key) {
            let raster = self.rasterise_glyph(glyph, face);
            self.by_glyph.insert(key, raster);
        }
        &self.by_glyph[&key]
    }

    /// Asks the font to shape the run, and reports it only if the font
    /// would draw it differently from the characters on their own.
    ///
    /// Fonts do this two ways. Some substitute one wide glyph for the whole
    /// run, which covers the cells its characters took. Others, Cascadia
    /// Code among them, substitute each character for a part of the joined
    /// shape, so the run stays one glyph per cell and only the glyphs
    /// change. Both arrive here as pieces, and a run whose glyphs come back
    /// unchanged is reported as nothing so the caller stays on its own path.
    fn shape(&self, text: &str, face: GlyphStyle) -> Vec<Piece> {
        let Some(loaded) = self.faces.get(&face) else {
            return Vec::new();
        };
        let Some(shaper) = rustybuzz::Face::from_slice(loaded.bytes.as_slice(), loaded.index)
        else {
            return Vec::new();
        };

        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(text);
        let shaped = rustybuzz::shape(&shaper, &[], buffer);

        let offsets: Vec<(usize, char)> = text.char_indices().collect();
        let infos = shaped.glyph_infos();
        let mut pieces: Vec<Piece> = Vec::with_capacity(infos.len());
        let mut changed = false;

        for (at, info) in infos.iter().enumerate() {
            let from = info.cluster as usize;
            let until = infos
                .get(at + 1)
                .map(|next| next.cluster as usize)
                .unwrap_or(text.len());
            let covered: Vec<char> = offsets
                .iter()
                .filter(|(offset, _)| *offset >= from && *offset < until)
                .map(|(_, ch)| *ch)
                .collect();
            if covered.is_empty() {
                continue;
            }

            let glyph = info.glyph_id as u16;
            let plain = (covered.len() == 1)
                .then(|| shaper.glyph_index(covered[0]).map(|id| id.0))
                .flatten();
            changed |= covered.len() > 1 || plain != Some(glyph);

            pieces.push(Piece {
                glyph,
                cells: covered.len() as u16,
            });
        }

        // Shaping that reorders or drops characters cannot be laid out in a
        // grid, so such a run is left to the per-cell path.
        let covered: u16 = pieces.iter().map(|piece| piece.cells).sum();
        if !changed || covered != offsets.len() as u16 {
            return Vec::new();
        }
        pieces
    }

    fn rasterise_glyph(&self, glyph: u16, face: GlyphStyle) -> Glyph {
        let Some(loaded) = self.faces.get(&face) else {
            return self.blank.clone();
        };
        let (metrics, coverage) = loaded.font.rasterize_indexed(glyph, self.size);
        Glyph {
            width: metrics.width,
            height: metrics.height,
            left: metrics.xmin,
            top: metrics.height as i32 + metrics.ymin,
            coverage,
        }
    }

    /// Which face a style actually draws with, so shaping and rasterising
    /// agree on one font.
    fn face_for(&self, style: GlyphStyle) -> Option<GlyphStyle> {
        style
            .fallbacks()
            .iter()
            .copied()
            .find(|candidate| self.faces.contains_key(candidate))
    }

    fn forget_runs_if_crowded(&mut self, face: GlyphStyle) {
        if let Some(runs) = self.shaped.get_mut(&face) {
            if runs.len() > REMEMBERED {
                runs.clear();
            }
        }
    }
}
