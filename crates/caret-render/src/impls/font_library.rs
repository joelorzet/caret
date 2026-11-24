#[allow(unused_imports)]
use crate::prelude::*;

use std::path::PathBuf;

impl FontLibrary {
    /// Reads the system font directories.
    pub fn load() -> FontLibrary {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        FontLibrary { db }
    }

    /// An empty library, for tests and headless use.
    pub fn empty() -> FontLibrary {
        FontLibrary {
            db: fontdb::Database::new(),
        }
    }

    pub fn load_dir(&mut self, dir: &std::path::Path) {
        self.db.load_fonts_dir(dir);
    }

    pub fn face_count(&self) -> usize {
        self.db.faces().count()
    }

    /// Every family, with what the picker needs to describe it.
    ///
    /// Sorted so that monospaced families come first: in a terminal those are
    /// the only ones that lay out correctly, and burying them under every
    /// installed display font makes the list useless.
    pub fn families(&self) -> Vec<FontFamily> {
        let mut families: Vec<FontFamily> = Vec::new();

        for face in self.db.faces() {
            let Some(name) = primary_family(face) else {
                continue;
            };
            let bold = face.weight.0 >= 600;
            let italic = FontSlant::from_db(face.style).is_slanted();

            match families.iter_mut().find(|f| f.name == name) {
                Some(existing) => {
                    existing.faces += 1;
                    existing.has_bold |= bold;
                    existing.has_italic |= italic;
                    existing.monospaced |= face.monospaced;
                }
                None => families.push(FontFamily {
                    name,
                    monospaced: face.monospaced,
                    faces: 1,
                    has_bold: bold,
                    has_italic: italic,
                }),
            }
        }

        families.sort_by(|a, b| {
            b.monospaced
                .cmp(&a.monospaced)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        families
    }

    pub fn monospaced(&self) -> Vec<FontFamily> {
        self.families()
            .into_iter()
            .filter(|f| f.monospaced)
            .collect()
    }

    /// Families whose name contains `query`, ignoring case.
    pub fn search(&self, query: &str) -> Vec<FontFamily> {
        let needle = query.trim().to_lowercase();
        if needle.is_empty() {
            return self.families();
        }
        self.families()
            .into_iter()
            .filter(|f| f.name.to_lowercase().contains(&needle))
            .collect()
    }

    pub fn has_family(&self, name: &str) -> bool {
        let wanted = name.trim().to_lowercase();
        self.db
            .faces()
            .filter_map(primary_family)
            .any(|family| family.to_lowercase() == wanted)
    }

    /// Every face in a family.
    pub fn faces_of(&self, family: &str) -> Vec<FontFace> {
        let wanted = family.trim().to_lowercase();
        let mut faces: Vec<FontFace> = self
            .db
            .faces()
            .filter(|face| primary_family(face).is_some_and(|f| f.to_lowercase() == wanted))
            .map(describe)
            .collect();
        faces.sort_by(|a, b| a.weight.cmp(&b.weight).then(a.slant.cmp(&b.slant)));
        faces
    }

    /// The upright, regular-weight face of a family.
    pub fn regular_face(&self, family: &str) -> Option<FontFace> {
        let faces = self.faces_of(family);
        faces
            .iter()
            .find(|f| !f.slant.is_slanted() && (350..=450).contains(&f.weight))
            .or_else(|| faces.iter().find(|f| !f.slant.is_slanted()))
            .or_else(|| faces.first())
            .cloned()
    }

    /// The first of `family` then `fallbacks` that is actually installed.
    ///
    /// Returning the name chosen, rather than silently substituting, lets the
    /// caller say which font it ended up with.
    pub fn resolve(&self, family: &str, fallbacks: &[String]) -> Option<FontFace> {
        core::iter::once(family)
            .chain(fallbacks.iter().map(String::as_str))
            .find_map(|name| self.regular_face(name))
    }
}

/// The first family name a face reports, which is the one users recognise.
fn primary_family(face: &fontdb::FaceInfo) -> Option<String> {
    face.families.first().map(|(name, _)| name.clone())
}

fn describe(face: &fontdb::FaceInfo) -> FontFace {
    FontFace {
        family: primary_family(face).unwrap_or_default(),
        post_script_name: face.post_script_name.clone(),
        slant: FontSlant::from_db(face.style),
        weight: face.weight.0,
        monospaced: face.monospaced,
        path: source_path(&face.source),
        index: face.index,
    }
}

fn source_path(source: &fontdb::Source) -> Option<PathBuf> {
    match source {
        fontdb::Source::File(path) => Some(path.clone()),
        fontdb::Source::SharedFile(path, _) => Some(path.clone()),
        // Loaded from memory, so there is no path to report.
        fontdb::Source::Binary(_) => None,
    }
}
