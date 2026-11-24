#[allow(unused_imports)]
use crate::prelude::*;

use std::path::Path;

impl ThemeRegistry {
    /// Built-in themes, plus any `*.toml` in the config directory's `themes/`
    /// folder. A user theme replaces a built-in with the same name.
    pub fn load() -> (ThemeRegistry, Vec<String>) {
        let mut registry = ThemeRegistry {
            themes: caret_theme::builtin::all(),
        };
        let mut warnings = Vec::new();

        if let Some(dir) = crate::impls::paths::themes_dir() {
            if dir.is_dir() {
                registry.load_dir(&dir, &mut warnings);
            }
        }
        (registry, warnings)
    }

    /// Built-ins plus every `*.toml` in `dir`.
    pub fn load_from_dir(dir: &Path) -> (ThemeRegistry, Vec<String>) {
        let mut registry = ThemeRegistry {
            themes: caret_theme::builtin::all(),
        };
        let mut warnings = Vec::new();
        if dir.is_dir() {
            registry.load_dir(dir, &mut warnings);
        }
        (registry, warnings)
    }

    pub fn builtin_only() -> ThemeRegistry {
        ThemeRegistry {
            themes: caret_theme::builtin::all(),
        }
    }

    fn load_dir(&mut self, dir: &Path, warnings: &mut Vec<String>) {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) => {
                warnings.push(format!("could not read {}: {e}", dir.display()));
                return;
            }
        };

        let mut paths: Vec<_> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "toml"))
            .collect();
        paths.sort();

        for path in paths {
            match std::fs::read_to_string(&path) {
                Ok(text) => {
                    let fallback = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("custom");
                    match parse_theme(&text, fallback) {
                        Ok(theme) => self.insert(theme),
                        Err(e) => warnings.push(format!("{}: {e}", path.display())),
                    }
                }
                Err(e) => warnings.push(format!("could not read {}: {e}", path.display())),
            }
        }
    }

    fn insert(&mut self, theme: Theme) {
        match self.themes.iter_mut().find(|t| t.name == theme.name) {
            Some(slot) => *slot = theme,
            None => self.themes.push(theme),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Theme> {
        let wanted = normalise(name);
        self.themes.iter().find(|t| normalise(&t.name) == wanted)
    }

    /// The named theme, or the default, or the first available.
    pub fn resolve(&self, name: &str) -> &Theme {
        self.get(name)
            .or_else(|| self.get(caret_theme::builtin::DEFAULT))
            .or_else(|| self.themes.first())
            .expect("at least one theme is always built in")
    }

    pub fn themes(&self) -> &[Theme] {
        &self.themes
    }

    pub fn names(&self) -> Vec<&str> {
        self.themes.iter().map(|t| t.name.as_ref()).collect()
    }
}

fn normalise(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '-' | '_' | ' '))
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn parse_theme(text: &str, fallback_name: &str) -> Result<Theme, String> {
    let raw: RawTheme = toml::from_str(text).map_err(|e| {
        e.to_string()
            .lines()
            .next()
            .unwrap_or("invalid theme")
            .trim()
            .to_string()
    })?;

    let colour = |label: &str, value: &str| -> Result<Rgb, String> {
        parse_hex(value).ok_or_else(|| format!("{label}: {value:?} is not a hex colour"))
    };

    let eight = |label: &str, values: &[String]| -> Result<[Rgb; 8], String> {
        if values.len() != 8 {
            return Err(format!(
                "{label} needs exactly 8 colours, got {}",
                values.len()
            ));
        }
        let mut out = [Rgb::BLACK; 8];
        for (i, v) in values.iter().enumerate() {
            out[i] = colour(&format!("{label}[{i}]"), v)?;
        }
        Ok(out)
    };

    let bg = colour("background", &raw.background)?;
    let fg = colour("foreground", &raw.foreground)?;
    let normal = eight("normal", &raw.normal)?;
    let bright = eight("bright", &raw.bright)?;

    let cursor = match &raw.cursor {
        Some(c) => colour("cursor", c)?,
        None => fg,
    };
    let selection = match &raw.selection {
        Some(c) => colour("selection", c)?,
        None => bg.blend(fg, 0.75),
    };
    let accent = match &raw.accent {
        Some(c) => colour("accent", c)?,
        None => normal[4],
    };
    let appearance = match &raw.appearance {
        Some(a) => Appearance::parse(a).ok_or_else(|| {
            format!(
                "appearance {a:?} is not one of {}",
                Appearance::accepted().join(", ")
            )
        })?,
        None => {
            if bg.relative_luminance() < 0.5 {
                Appearance::Dark
            } else {
                Appearance::Light
            }
        }
    };

    let name = raw.name.unwrap_or_else(|| fallback_name.to_string());
    let description = raw
        .description
        .unwrap_or_else(|| "Custom theme.".to_string());

    Ok(Theme::new(
        std::borrow::Cow::Owned(name),
        std::borrow::Cow::Owned(description),
        appearance,
        bg,
        fg,
        cursor,
        selection,
        accent,
        normal,
        bright,
    ))
}
