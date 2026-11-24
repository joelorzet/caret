# Contributing

Thanks for looking. caret is a terminal emulator written from scratch, which
means most changes touch something with a standard behind it or a measurement
against it. This page is what a patch needs to go in smoothly.

## Getting set up

```sh
git clone git@github.com:joelorzet/caret.git
cd caret
cargo build
cargo run -- run        # the terminal, in its own window
cargo run -- inline     # inside the terminal you are already in
```

Rust 1.87 or newer, which is the version CI checks against as well as stable.
macOS and Linux both build; Windows needs a ConPTY backend and currently fails
to compile with a message saying so.

## What CI will run

These three, on Linux and macOS, on stable and on 1.87. A pull request cannot
merge until they pass, so run them before you open one:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Warnings are errors. There are no exceptions checked in, so if clippy objects
to your code the answer is to change the code or to argue the case in the pull
request, not to add an allow.

## The house rules

[docs/CONVENTIONS.md](docs/CONVENTIONS.md) is the full version and is worth
reading before a first patch. The short form:

**Declarations and code live apart.** One type per file in `types/`, one enum
per file in `enums/`, the code in `impls/`, and the crate's own re-exports in
`prelude.rs`. Every file in `impls/` and `types/` opens with
`use crate::prelude::*;`.

**A file covers one subject.** When an `impl` grows past a few hundred lines it
has usually taken on several, and the fix is a folder:
`impls/host/draw/{tabs,docks,status,palette}.rs`. Rust has no inheritance, and
reaching for a trait to fake it is how a codebase ends up with interfaces that
have one implementor. A trait earns its place when two things really are
interchangeable at a call site.

**Numbers get names.** A literal in a `match` or a condition is a value nobody
can check. Protocol bytes, mode numbers, key codes, sizes and thresholds all
get a name first, in a private `mod` beside the code that reads them, named
after what the numbers select. `0` and `1` meaning nothing more than zero and
one are the exception.

**Comments explain decisions.** The code says what it does. A comment is for
why it does it that way, an invariant that is not obvious, or a behaviour
copied from another terminal. Not for restating a signature.

**The dependency floor holds.** `caret-core` and `caret-layout` have no
dependencies and are not getting any: they are pure domain and pure geometry,
which is what makes them exhaustively testable. New dependencies belong at the
edges, in the pty, font and windowing crates.

## Tests

Tests never sit beside the code. Each crate has `tests/`, using only the public
API, one file per concern, and a folder when a subject has several concerns of
its own:

```
tests/screen/
├── main.rs      only `mod` lines
├── support.rs   helpers the modules share
├── printing.rs
└── scrolling.rs
```

Anything in `caret-core` or `caret-layout` can be tested without a window or a
child process, so it should be. A test name is a sentence about behaviour:
`shrinking_keeps_what_was_written_rather_than_the_blank_room_below_it` beats
`test_resize_2`.

Two rules that are easy to trip over. A test that sets an environment variable
goes in a file of its own, because the environment is process wide and the
tests in one binary run in parallel. A test that cannot run without something
the machine may not have, such as an installed font, may skip on a laptop and
must fail on CI, where an empty system is a broken runner rather than a
legitimate state.

## Working on the window

A terminal cannot log to its own stdout, and a window is awkward to inspect, so
there are two ways in:

```sh
CARET_TRACE=/tmp/caret.log caret          # actions and spawns
CARET_SNAPSHOT=/tmp/shot.bmp \
CARET_SNAPSHOT_KEYS='cmd+d,cmd+option+m' \
CARET_SNAPSHOT_DELAY=2000 caret           # write one frame and exit
```

The snapshot is the buffer that was just painted, so colours can be measured
from the file rather than guessed at.

If you are changing anything that runs every frame or every tick, measure it.
Idle cost is a feature here: a pane doing nothing costs 0.15% of a core, and
[docs/limitations.md](docs/limitations.md) records what a frame actually costs
and why.

## Branches and commits

Never commit to `main`. Branch as `<kind>/<subject>`, lower case with
hyphens:

```
feat/update-check     fix/fullscreen-resize
ref/split-session     hotfix/release-tag
```

The kinds are `feat`, `fix`, `ref`, `hotfix`, `docs`, `ci`, `chore`,
`test`, `perf`, and for commits also `build` and `revert`.

Commit messages are [Conventional Commits](https://www.conventionalcommits.org):

```
fix(core): keep written rows when a pane grows

Every resize pads the buffer out to the new height, and that padding was
counted as content, so the next time the window grew it was revealed as
scrollback. Rows below the cursor that hold nothing are now trimmed.
```

A subject is imperative, lower case after the kind, has no full stop and
fits in 72 characters. A body says why, wrapped at 72 columns. A breaking
change takes `!` after the kind or a `BREAKING CHANGE:` footer.

One subject per commit. Amend or rebase your own unpushed work rather than
adding "wip" and "fix typo" on top of it. Merging is a squash, so the
title of the pull request is the subject that lands on main and is held to
the same rule. CI checks all of this, and `scripts/check-subject.sh` is
the same check you can run by hand.

## Pull requests

- Keep a pull request to one subject; two subjects are two pull requests.
- Say what the change does and why, and what you measured if it is about speed
  or memory. A failing test before and a passing one after is the best
  argument there is.
- Update the documentation the change invalidates: [docs/KEYBINDINGS.md](docs/KEYBINDINGS.md)
  is generated by `caret keys`, [docs/THEMES.md](docs/THEMES.md) lists every
  theme and its contrast, and [CHANGELOG.md](CHANGELOG.md) gets a line under
  the unreleased heading.
- New behaviour that a user can see belongs in the README too.

## Reporting a bug

Most useful, in this order: what you expected and what happened, the output of
`caret config`, whether it happens in `caret run` or `caret inline`, the theme
and font, and the program running in the pane. An escape sequence bug is best
reported with the bytes, and `CARET_DUMP=/tmp/caret` records each pane's
raw output to `pane-<id>.bin` in that directory.

## Releases

Maintainers only. Releases are cut from a tag, never from a push to main. The
tag and the version in `Cargo.toml` have to agree, and CI checks that:

```sh
git tag -a v0.1.0 -m "caret 0.1.0"
git push origin v0.1.0
```

The tag runs the full suite, then builds the macOS disk image and the Linux
tarball and publishes them with their checksums.

A tag that has published is never moved: the download exists and somebody
may have it. If a release fails before publishing, fix it and cut the next
patch version. CI refuses to publish over a tag that already has a release.
