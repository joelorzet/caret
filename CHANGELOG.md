# Changelog

Notable changes, newest first. Versions follow [semantic
versioning](https://semver.org): until 1.0 a minor bump may change behaviour,
and a patch never does.

## Unreleased

### Added

- **A line that runs through an alias or a shell function is coloured** like
  anything else that runs. The shell is asked once, in the background, what
  names it knows, and they join what is on `PATH`. A typo still stays plain,
  which is the property worth keeping.

### Fixed

- **Panes start a login shell**, the way every other terminal does. Without
  it a pane reads `.zshrc` but not `.zprofile`, which is where Homebrew and
  most other installers write the PATH, so anything installed that way was
  missing: `brew`, and every alias pointing at a tool it installed.

## 0.1.2

### Fixed

- **Going full screen left the prompt in the middle of a pane.** Every
  resize pads the buffer out to the new height, and that padding was
  counted as content, so the next time the window grew it was revealed the
  way scrollback is revealed. Rows below the cursor that hold nothing are
  now trimmed before a resize decides what to keep.
- A test asserted macOS behaviour on every platform and was flaky on
  Linux. What a program writes as it exits is kept by macOS and dropped by
  Linux, which is now recorded in the limitations rather than asserted
  away.

### Changed

- A release refuses to publish over a tag that already has one. A
  published tag names a download somebody may already have, so moving it
  would hand them different code under the same name.

## 0.1.1

### Added

- **Update checking.** caret asks once a day whether a newer release has been
  published, and says so in the status line when there is one. The check
  sends caret's name and version and nothing else: no identifiers, nothing
  about the machine or the person at it. `check = false` under `[update]`
  turns it off and caret never touches the network.
- **`caret update`** says whether a newer release is out, and
  **`caret update --install`** fetches it, checks it against the published
  `SHA256SUMS` and puts it in place. Nothing is installed that does not match
  its checksum, and the file to fetch is chosen from the names the release
  lists rather than guessed at, so there is one place that decides what a
  build is called.
- **Look for a newer release** and **Install the newer release** in the
  command palette, so updating is two key presses and no typing.
- **`caret` works inside caret.** A pane starts with the running binary's
  directory in front of `PATH`, so an app bundle that installs nothing on
  the system can still be told `caret update`.
- **`caret version`**, also `--version` and `-V`.

### Fixed

- A pane no longer loses what it printed. A short command that wrote and
  exited between two wake-ups was noticed as dead and closed with its output
  still in the buffer.

## 0.1.0

The first release.

### The terminal

- Its own grid, escape parser and renderer. Cells, colours, cursor, selection
  and search are plain data structures; VT sequences are turned into grid
  edits by `caret-vt`; the window rasterises glyphs itself and blends into a
  CPU framebuffer.
- Box drawing characters are drawn rather than taken from the font, so borders
  meet exactly at every cell boundary.
- One `poll` covers every pane, so panes cost a child process each and no
  extra threads.
- 10,000 lines of scrollback a pane, searchable with `ctrl+b /`, case
  insensitive until a capital is typed.
- Mouse reporting, selection with the mouse, bracketed paste, and OSC 52 for
  the clipboard so copying works over SSH.

### Panes, tabs and layouts

- tmux-style splits with both a `ctrl+b` prefix and direct Cmd shortcuts,
  named for the platform they are running on.
- Five layout presets, saved layouts in the config file, and a spec language
  for an explicit shape, for example `h(60:*, 40:v(*, *))`.
- An arrange panel (`cmd+option+m`) that previews a new layout and applies
  nothing until it is accepted. Pane numbers stay with their panes.
- Tabs named after their directory, renameable with `cmd+R`.

### Appearance and configuration

- 56 themes, 47 dark and 9 light, every one held to 4.5:1 WCAG contrast for
  body text by a test.
- A command palette that also rebinds, a settings list that writes the setting
  it is pointing at into the config file, and a theme picker that previews
  live.
- `~/.config/caret/config.toml` with every setting documented and a fallback
  for every value that is not understood.

### Panels

- A file explorer that follows the focused pane's directory, with a syntax
  coloured file preview that can hand a file to `$EDITOR`.
- An agent panel on the right running a command of your choice, `claude` by
  default.

### What it costs to run

- Idle, with a pane open and nothing running: **0.15% of a core and 65MB**.
  The window waits rather than waking 125 times a second to find nothing to
  do, and a key or a click still wakes it immediately.
- The cursor blinks while the window has the keyboard and somebody is using
  it, and holds steady otherwise. A blink is a whole frame handed to the
  compositor, which is 20ms of work at a full-screen retina size.
- The grid is sized for the lines a pane has actually printed and doubles as
  history fills, rather than allocating the whole configured scrollback at
  birth: 2MB a pane instead of 32MB until it is earned.

### Packaging

- A release ships **one macOS disk image**, universal for Apple silicon and
  Intel, built on a single runner. Two runners meant asking for two of the
  scarcest machines GitHub has, and a release waited twenty minutes before
  losing the second one.

- `scripts/bundle-mac.sh` builds `caret.app` and a disk image, with the icon
  drawn by a script rather than stored as a binary.
- Settings written under the project's former name are adopted on first run
  and the old directory is left untouched.
