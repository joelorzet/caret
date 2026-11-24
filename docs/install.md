# Installing

## macOS, as an app

```
./scripts/bundle-mac.sh --install
```

Builds `caret.app` and copies it to `~/Applications`, then tells
Spotlight about it so it is findable straight away. Type `caret`.

Add `--dmg` for a disk image in `target/macos/`, with a shortcut to
`/Applications` to drag onto, and `--universal` to build for both Apple
silicon and Intel in one bundle, which is what a release ships. `--install` and `--dmg` can be used together or
on their own; neither is required to build the bundle.

The app is signed ad-hoc, which is enough for it to open on the machine that
built it. On another machine Gatekeeper will object, because it is not signed
with a Developer ID: right-click the app and choose Open the first time.

The icon is drawn by `scripts/icon.py` rather than stored as a binary, and
rebuilt with each bundle.

## Reinstalling

The same command. It replaces whatever is in `~/Applications`:

```
./scripts/bundle-mac.sh --install
```

Nothing has to be uninstalled first, and your configuration is not touched:
it lives in `~/.config/caret/`, not in the bundle.

A copy that is already running keeps running the old build until you quit it,
because macOS lets the bundle be replaced underneath it. The script says so
when that is the case. Quit caret, then open it again.

To remove it entirely, delete `~/Applications/caret.app`. That leaves your
configuration; delete `~/.config/caret/` as well to be rid of that too.

## As a command

```
cargo install --path .
```

Puts `caret` on your PATH, and the same command reinstalls it. `caret` with no
arguments opens the window, which is the same thing the app does; `caret
inline` runs inside whatever terminal you are already in, for SSH. `caret
help` lists the rest.

## Leaving

Quitting hangs up every pane's child. That also happens on `kill`, on logout
and on a `pkill`, because the signals that mean stop are caught and the panes
are closed on the way out. `kill -9` cannot be caught, and will leave the
children running on a pty nothing is reading.
