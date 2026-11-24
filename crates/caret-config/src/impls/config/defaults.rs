#[allow(unused_imports)]
use crate::prelude::*;

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            family: "JetBrains Mono".into(),
            fallback: vec![
                "SF Mono".into(),
                "Menlo".into(),
                "DejaVu Sans Mono".into(),
                "Apple Color Emoji".into(),
            ],
            size: 13.0,
            line_height: 1.2,
            bold_is_bright: false,
            ligatures: true,
            letter_spacing: 0.0,
            italics: true,
            builtin_box_drawing: true,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        WindowConfig {
            opacity: 1.0,
            padding: 8,
            decorations: true,
            cols: 100,
            rows: 30,
        }
    }
}

impl Default for TerminalConfig {
    fn default() -> Self {
        TerminalConfig {
            scrollback: 10_000,
            shell: None,
            shell_args: Vec::new(),
            cursor: CursorShape::Block,
            cursor_fill: CursorFill::Blink,
            cursor_color: None,
            bell: BellStyle::Visual,
            word_separators: " \t\n\"'`()[]{}<>|;:,".into(),
            confirm_close: true,
            highlight_input: true,
            mouse: true,
        }
    }
}

impl Default for UpdateConfig {
    fn default() -> Self {
        UpdateConfig {
            check: true,
            interval_hours: 24,
        }
    }
}

impl Default for ChromeConfig {
    fn default() -> Self {
        ChromeConfig {
            tab_bar: true,
            status: true,
            hide_tab_bar_when_single: false,
            show_memory: false,
            padding: 1,
        }
    }
}

impl Default for ExplorerConfig {
    fn default() -> Self {
        ExplorerConfig {
            visible: false,
            width: 28,
            show_hidden: false,
            follow_cwd: true,
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        AgentConfig {
            visible: false,
            width: 44,
            command: "claude".into(),
            args: Vec::new(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: caret_theme::builtin::DEFAULT.into(),
            font: FontConfig::default(),
            window: WindowConfig::default(),
            terminal: TerminalConfig::default(),
            chrome: ChromeConfig::default(),
            update: UpdateConfig::default(),
            sidebars: SidebarsConfig::default(),
            keymap: Keymap::default(),
            layouts: LayoutTemplate::presets(),
        }
    }
}
