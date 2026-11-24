# Themes

56 built-in themes, 47 dark and 9 light. Preview them in colour with `caret themes`, or pick one
interactively with `ctrl+b T` (`cmd+T` in the window): arrowing through the
list applies each one live, and enter saves it to your config.

A name ending in `-contrast` is a deeper, punchier cut of the theme it
is named after: the background taken further from the text and the
colours brought forward. `phosphor-*` are single-colour CRT palettes,
which keep red and yellow so errors and warnings still carry.

Every theme is held to 4.5:1 WCAG contrast for body text by a test; one that
falls below it does not ship. Window chrome is derived from each palette, and
dividers are solved for a consistent 2.4:1 so they read the same everywhere.

| Theme | Appearance | Text contrast |
|---|---|---|
| `dracula` | dark | 13.4:1 |
| `one-dark-pro` | dark | 6.6:1 |
| `one-dark-pro-darker` | dark | 7.0:1 |
| `one-dark-pro-night-flat` | dark | 8.3:1 |
| `nord` | dark | 9.2:1 |
| `gruvbox-dark` | dark | 10.7:1 |
| `tokyo-night` | dark | 10.6:1 |
| `catppuccin-mocha` | dark | 11.3:1 |
| `solarized-dark` | dark | 4.7:1 |
| `solarized-light` | light | 5.0:1 |
| `phosphor-green` | dark | 15.0:1 |
| `phosphor-green-soft` | dark | 11.4:1 |
| `phosphor-amber` | dark | 11.0:1 |
| `phosphor-amber-soft` | dark | 9.3:1 |
| `phosphor-yellow` | dark | 16.7:1 |
| `phosphor-orange` | dark | 8.0:1 |
| `phosphor-cyan` | dark | 14.2:1 |
| `phosphor-blue` | dark | 6.4:1 |
| `phosphor-magenta` | dark | 6.4:1 |
| `phosphor-white` | dark | 13.9:1 |
| `dracula-contrast` | dark | 17.9:1 |
| `tokyo-night-contrast` | dark | 13.8:1 |
| `gruvbox-dark-contrast` | dark | 15.1:1 |
| `nord-contrast` | dark | 14.5:1 |
| `one-dark-pro-contrast` | dark | 10.9:1 |
| `catppuccin-mocha-contrast` | dark | 14.7:1 |
| `kanagawa-wave-contrast` | dark | 14.9:1 |
| `everforest-dark-contrast` | dark | 12.5:1 |
| `ayu-dark-contrast` | dark | 11.8:1 |
| `monokai-pro-contrast` | dark | 17.5:1 |
| `carbonfox-contrast` | dark | 18.6:1 |
| `github-dark-contrast` | dark | 11.7:1 |
| `github-light-contrast` | light | 21.0:1 |
| `solarized-light-contrast` | light | 12.2:1 |
| `catppuccin-latte-contrast` | light | 16.2:1 |
| `ember` | dark | 12.9:1 |
| `catppuccin-latte` | light | 7.1:1 |
| `catppuccin-frappe` | dark | 8.1:1 |
| `catppuccin-macchiato` | dark | 9.9:1 |
| `gruvbox-light` | light | 10.2:1 |
| `gruvbox-material` | dark | 8.4:1 |
| `everforest-dark` | dark | 7.4:1 |
| `everforest-light` | light | 5.2:1 |
| `kanagawa-wave` | dark | 11.3:1 |
| `kanagawa-dragon` | dark | 10.8:1 |
| `monokai-pro` | dark | 12.9:1 |
| `ayu-dark` | dark | 9.0:1 |
| `ayu-mirage` | dark | 9.6:1 |
| `ayu-light` | light | 6.1:1 |
| `github-dark` | dark | 8.8:1 |
| `github-light` | light | 18.9:1 |
| `horizon-dark` | dark | 12.6:1 |
| `iceberg` | dark | 10.6:1 |
| `duskfox` | dark | 11.9:1 |
| `carbonfox` | dark | 16.4:1 |
| `cobalt2` | dark | 15.5:1 |

Add your own by dropping a `.toml` into `~/.config/caret/themes/`; it
overrides a built-in of the same name. Only `background`, `foreground`,
`normal` and `bright` are required, cursor, selection, accent, appearance and
the whole window chrome are derived.
