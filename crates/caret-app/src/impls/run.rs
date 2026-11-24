use caret_config::Keymap;
use caret_layout::{Axis, Chrome, Rect, Workspace};

use crate::impls::{config_cmd, fonts, preview, shortcuts, themes};
use crate::Session;

const USAGE: &str = "\
usage: caret [command]

  run                    start the terminal in its own window (default)
  inline                 run inside this terminal instead, for SSH
  keys                   list the keyboard shortcuts
  themes                 preview the built-in themes
  fonts [query]          list installed fonts, monospaced first
  config [show|example|init]
                         show settings, print a starter config, or write one
  layout                 print the pane-tree geometry
";

pub fn run() {
    // The project was called main-terminal until it was called caret, and a
    // rename must not quietly lose someone's settings.
    caret_config::paths::adopt_former_config();

    let command = std::env::args().nth(1);
    let code = match command.as_deref() {
        None | Some("run") => window(),
        Some("keys") => {
            print!("{}", shortcuts::render(&Keymap::default()));
            0
        }
        Some("themes") => {
            print!(
                "{}",
                themes::render(caret_config::ThemeRegistry::load().0.themes())
            );
            0
        }
        Some("window") => window(),
        Some("inline") => inline(),
        Some("config") => config_cmd::run(std::env::args().nth(2).as_deref()),
        Some("fonts") => {
            print!("{}", fonts::render(std::env::args().nth(2).as_deref()));
            0
        }
        Some("layout") => {
            print!("{}", demo_layout());
            0
        }
        Some("help") | Some("--help") | Some("-h") => {
            print!("{USAGE}");
            0
        }
        Some(other) => {
            eprintln!("unknown command {other:?}\n");
            eprint!("{USAGE}");
            2
        }
    };
    if code != 0 {
        std::process::exit(code);
    }
}

fn window() -> i32 {
    match crate::impls::window::app::run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("caret: {e}");
            eprintln!("Try `caret inline` to run inside this terminal instead.");
            1
        }
    }
}

/// Draws into the terminal that launched us, for SSH or a nested session.
fn inline() -> i32 {
    // Raw mode needs a real terminal; a pipe would fail in tcgetattr with a
    // message that explains nothing.
    if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
        eprintln!("caret run needs a terminal on stdin.");
        eprintln!("Run it directly rather than through a pipe.");
        return 1;
    }

    match Session::new().and_then(|mut session| session.run()) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("caret: {e}");
            1
        }
    }
}

fn demo_layout() -> String {
    let mut ws = Workspace::new();
    ws.split(Axis::Horizontal, false);
    ws.split(Axis::Vertical, false);
    ws.new_tab();
    ws.select_tab(0);

    let mut chrome = Chrome::default();
    chrome.left.visible = true;
    chrome.right.visible = true;

    let mut out = preview::ascii(&ws, &chrome, Rect::new(0, 0, 100, 20));
    out.push_str(&format!(
        "\n\n{} tab(s), {} pane(s) in the active tab, focus on pane {}\n",
        ws.tab_count(),
        ws.active_tab().pane_count(),
        ws.active_pane().0
    ));
    out
}
