# caret: scope and state

**It runs in its own window.** `caret run` opens a native window: tmux-style
splits, tabs, themes, scrollback with search, mouse, selection and copy, a file
explorer and an agent dock. `caret inline` runs it inside another terminal, for
SSH or a nested session.

## Requested features

| # | Feature | State |
|---|---------|-------|
| 1 | Themes (10+, incl. black/green and orange families) | **done**: 56, contrast-gated |
| 2 | Resize (window + pane) | **done** |
| 3 | Splitting, tmux-style, native | **done** |
| 4 | Memory / process efficiency | **done**: grid grows with use, one poll loop, one pty per pane, idle costs 0.15% of a core |
| 5 | Command autocomplete | **open question** (below) |
| 6 | Multi-tab | **done** |
| 7 | Agent dock (Claude, right) | **done**: a pane running the configured command |
| 8 | File explorer dock (left), arrow nav | **done** |
| 9 | Shortcuts + discoverable help | **done**: `ctrl+b ?` overlay |
| 10 | Full config section | **done**: TOML, user themes, rebinding, saved layouts |
| 11 | Font selector + ligature settings | **done**: `caret fonts`; see limitations |
| 12 | Layout memory bank + pane commands | **done**: presets, saved layouts, spec language |

## Crates

Ten crates, 659 tests, 0 clippy findings. `caret-core` and `caret-layout` have no
dependencies at all, which is what makes them exhaustively testable.

| Crate | Dependencies | Tests | What it is |
|-------|--------------|-------|------------|
| `caret-core` | none | 84 | cells, colours, grid, cursor, selection, search |
| `caret-layout` | none | 123 | pane tree, tabs, saved layouts, chrome |
| `caret-theme` | `caret-core` | 27 | theme model, 56 built-in themes |
| `caret-config` | core, layout, theme, serde, toml | 100 | actions, keymap, config, theme registry |
| `caret-vt` | `caret-core`, vte | 84 | escape sequences onto the terminal state |
| `caret-pty` | libc | 15 | pseudoterminals, one `poll` for every pane |
| `caret-render` | core, theme, fontdb, fontdue | 34 | font discovery, metrics, glyph raster |
| `caret-features` | none | 109 | the file explorer and the file preview |
| `caret-ui` | none | 20 | the tab strip: layout and hit testing |
| `caret-app` | all, winit, softbuffer | 63 | window, event loop, input, both renderers |

## Running it

```
cargo build --release
./target/release/caret run          # the terminal, in its own window
./target/release/caret inline       # or inside this terminal, for SSH
./target/release/caret keys         # shortcuts
./target/release/caret themes       # colour previews
./target/release/caret fonts [q]    # installed fonts, monospaced first
./target/release/caret config       # effective settings and any problems
./target/release/caret config init  # write a starter config
```

`CARET_TRACE=/tmp/caret.log` logs actions and spawns; a terminal cannot log to its
own stdout.

## What works

**Panes.** Split with `ctrl+b %` and `ctrl+b "`, move with arrows or `hjkl`,
resize with `ctrl+b` + ctrl-arrows, zoom with `ctrl+b z`, equalise with
`ctrl+b E`. Closing the last one exits.

**Layouts.** Five presets (`ctrl+b alt+1..5`, `ctrl+b space` cycles) plus your
own in config, either as a preset with a pane count or as an explicit shape in
the spec language (`h(60:*, 40:v(*, *))`). Applying a layout reuses the panes
already open, so rearranging never restarts a shell.

`ctrl+b t` turns the split holding the focused pane the other way, which is
the quick way to change one split without rebuilding the tab, and
`ctrl+b ctrl+o` moves the panes round the arrangement without changing its
shape.

**Scrollback.** `shift+pageup`, `shift+up`, and the wheel. `ctrl+b /` searches
the whole history, case-insensitive until you type a capital; up and down move
between matches, enter keeps the match selected so `ctrl+b y` copies it.

**Mouse.** Click focuses a pane or a tab, drag selects, double click a word,
triple click a line, wheel scrolls. Applications that ask for mouse reports get
them; holding shift selects anyway.

**Copy.** `ctrl+b y` puts the selection on the system clipboard with OSC 52, so
it works over SSH and needs no platform clipboard library. Paste arrives from
the surrounding terminal and is bracketed when the application asked for it.

**Sidebars.** `ctrl+b e` toggles the file explorer on the left, `ctrl+b a` the
agent on the right. The explorer navigates with arrows (right enters, left goes
back, `.` toggles hidden files) and follows the focused pane's directory;
pressing enter on a file types its path into the shell. The agent is an
ordinary pane running `claude`, so it costs one child process.

## Not done

- **Ligature shaping**: one glyph per cell, so `!=` stays two glyphs.
- **GPU rendering**: the window rasterises and blends on the CPU.
- **Reflow on resize**: columns truncate instead of rewrapping.
- **Combining characters, DCS sequences, character sets**: see
  [limitations.md](limitations.md).
- **Windows**: needs ConPTY; see [platforms.md](platforms.md).

## Open question

**#5 autocomplete.** Asked for, then "that already exists". Shells already
complete commands, so the question is whether you want a *terminal-level*
inline suggestion, ghost text from your own history, shell-independent, like
Warp or fish autosuggest. Not blocking; the seam is there.
