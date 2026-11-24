#[allow(unused_imports)]
use crate::prelude::*;

use std::path::Path;

impl Config {
    /// Loads the config from its standard location, falling back to defaults.
    pub fn load() -> LoadedConfig {
        match crate::impls::paths::config_path() {
            Some(path) if path.exists() => Config::load_from(&path),
            Some(path) => LoadedConfig {
                config: Config::default(),
                path: Some(path),
                warnings: Vec::new(),
            },
            None => LoadedConfig {
                config: Config::default(),
                path: None,
                warnings: Vec::new(),
            },
        }
    }

    pub fn load_from(path: &Path) -> LoadedConfig {
        match std::fs::read_to_string(path) {
            Ok(text) => {
                let (config, warnings) = Config::parse(&text);
                LoadedConfig {
                    config,
                    path: Some(path.to_path_buf()),
                    warnings,
                }
            }
            Err(e) => LoadedConfig {
                config: Config::default(),
                path: Some(path.to_path_buf()),
                warnings: vec![format!("could not read {}: {e}", path.display())],
            },
        }
    }

    /// Parses config text over the defaults.
    ///
    /// Anything invalid is reported and left at its default rather than
    /// failing the load: the terminal has to start even with a broken config.
    pub fn parse(text: &str) -> (Config, Vec<String>) {
        let mut config = Config::default();
        let mut warnings: Vec<String> = crate::impls::config::schema::unknown_keys(text)
            .into_iter()
            .map(|key| format!("unknown setting {key:?}"))
            .collect();

        match toml::from_str::<RawConfig>(text) {
            Ok(raw) => config.apply(raw, &mut warnings),
            Err(e) => warnings.push(format!(
                "could not parse config: {}",
                first_line(&e.to_string())
            )),
        }

        (config, warnings)
    }

    /// The chrome geometry this config asks for.
    pub fn chrome(&self) -> Chrome {
        Chrome {
            tab_bar: u16::from(self.chrome.tab_bar),
            padding: self.chrome.padding,
            status: if self.chrome.status {
                Chrome::STATUS_ROWS
            } else {
                0
            },
            left: Dock {
                kind: DockKind::Explorer,
                cells: self.sidebars.explorer.width,
                visible: self.sidebars.explorer.visible,
            },
            right: Dock {
                kind: DockKind::Agent,
                cells: self.sidebars.agent.width,
                visible: self.sidebars.agent.visible,
            },
        }
    }

    /// A saved layout by name, matched the way theme names are.
    pub fn layout(&self, name: &str) -> Option<&LayoutTemplate> {
        let wanted = normalise(name);
        self.layouts.iter().find(|t| normalise(&t.name) == wanted)
    }

    /// The shell to launch: the configured one, else `$SHELL`, else `/bin/sh`.
    pub fn shell_program(&self) -> String {
        self.terminal
            .shell
            .clone()
            .or_else(|| std::env::var("SHELL").ok().filter(|s| !s.is_empty()))
            .unwrap_or_else(|| "/bin/sh".into())
    }
}

fn normalise(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '-' | '_' | ' '))
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn first_line(text: &str) -> String {
    text.lines().next().unwrap_or(text).trim().to_string()
}
