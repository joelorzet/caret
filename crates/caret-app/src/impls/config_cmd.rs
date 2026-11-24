use caret_config::{Config, ThemeRegistry};

/// `caret config` shows what is in effect; `example` prints a starter file;
/// `init` writes one.
pub fn run(arg: Option<&str>) -> i32 {
    match arg {
        None | Some("show") => {
            print!("{}", show());
            0
        }
        Some("example") => {
            print!("{}", caret_config::EXAMPLE);
            0
        }
        Some("init") => init(),
        Some(other) => {
            eprintln!("unknown config command {other:?}");
            eprintln!("usage: caret config [show|example|init]");
            2
        }
    }
}

fn init() -> i32 {
    let Some(path) = caret_config::paths::config_path() else {
        eprintln!("could not work out where the config should live");
        return 1;
    };
    match Config::write_example(&path) {
        Ok(()) => {
            println!("wrote {}", path.display());
            0
        }
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

fn show() -> String {
    let loaded = Config::load();
    let (registry, theme_warnings) = ThemeRegistry::load();
    let c = &loaded.config;

    let mut out = String::new();
    match &loaded.path {
        Some(p) if p.exists() => out.push_str(&format!("config    {}\n", p.display())),
        Some(p) => out.push_str(&format!(
            "config    {} (not present, using defaults)\n",
            p.display()
        )),
        None => out.push_str("config    no config directory found\n"),
    }

    let theme = registry.resolve(&c.theme);
    if theme.name != c.theme {
        out.push_str(&format!(
            "theme     {} (requested {:?} was not found)\n",
            theme.name, c.theme
        ));
    } else {
        out.push_str(&format!("theme     {}\n", theme.name));
    }
    out.push_str(&format!(
        "themes    {} available\n",
        registry.themes().len()
    ));

    out.push_str(&format!(
        "font      {} {}pt, line height {}\n",
        c.font.family, c.font.size, c.font.line_height
    ));
    out.push_str(&format!(
        "window    {}x{} cells, padding {}, opacity {}\n",
        c.window.cols, c.window.rows, c.window.padding, c.window.opacity
    ));
    out.push_str(&format!(
        "shell     {} {:?}\n",
        c.shell_program(),
        c.terminal.shell_args
    ));
    out.push_str(&format!(
        "terminal  {} lines scrollback, {} cursor, bell {}\n",
        c.terminal.scrollback,
        c.terminal.cursor.name(),
        c.terminal.bell.name()
    ));
    out.push_str(&format!(
        "explorer  {}, {} cells\n",
        visible(c.sidebars.explorer.visible),
        c.sidebars.explorer.width
    ));
    out.push_str(&format!(
        "agent     {}, {} cells, runs {:?}\n",
        visible(c.sidebars.agent.visible),
        c.sidebars.agent.width,
        c.sidebars.agent.command
    ));

    let prefix = c
        .keymap
        .prefix()
        .map(|p| p.to_string())
        .unwrap_or_else(|| "none".into());
    let count: usize = c.keymap.shortcuts().iter().map(|(_, r)| r.len()).sum();
    out.push_str(&format!(
        "keys      prefix {prefix}, {count} actions bound\n"
    ));

    let warnings: Vec<&String> = loaded
        .warnings
        .iter()
        .chain(theme_warnings.iter())
        .collect();
    if warnings.is_empty() {
        out.push_str("\nno problems found\n");
    } else {
        out.push_str(&format!("\n{} problem(s):\n", warnings.len()));
        for w in warnings {
            out.push_str(&format!("  - {w}\n"));
        }
    }
    out
}

fn visible(on: bool) -> &'static str {
    if on {
        "shown"
    } else {
        "hidden"
    }
}
