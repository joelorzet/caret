# Conventions

## Crate layout

Every crate separates declarations from code:

```
src/
├── types/     one struct per file
├── enums/     one enum per file
├── traits/    one trait per file  (where a crate has any)
├── impls/     the code: `impl` blocks and free functions
├── prelude.rs crate-internal re-exports of the above
└── lib.rs     module wiring and the public re-exports
```

A large type's `impl` splits into a folder of its own, grouped by concern:

```
impls/screen/{access,printing,editing,scrolling,resize,damage,...}.rs
```

Because `impl` blocks live in different modules from their type, struct fields
and private helpers are `pub(crate)`. That is crate-internal only, nothing
extra is visible to consumers.

Every `impls/` and `types/` file opens with `use crate::prelude::*;`, so adding
a type does not mean editing import lists across the crate.

## One file per thing it does

`impls/` holds the code, and a file there covers one subject. When an `impl`
block grows past a few hundred lines it is usually covering several, and the
fix is a folder: `impls/host/draw/` is the drawing layer, with the tab bar,
the docks, the status line, the palette and the rest each in a file of their
own, and `mod.rs` holding only what they share and the entry point that calls
them.

Rust has no inheritance, and reaching for a trait to get it is how a codebase
ends up with interfaces that have exactly one implementor. A trait earns its
place when two things really are interchangeable at a call site. Splitting a
long `impl` across files needs no trait at all: the methods keep working on
the same type, and each file states what it is for.

Keep a constant and the helper functions beside the code that reads them. A
value used in two modules belongs to the parent; one used in a single file
belongs in that file.

## Tests

Tests never live beside the code. Each crate has `tests/`, one file per
concern, using only the public API:

```
tests/
├── common/mod.rs   shared helpers
├── splitting.rs
├── navigation.rs
└── resizing.rs
```

A subject with several concerns of its own becomes a folder, the same way an
`impl` does. Cargo compiles `tests/<name>/main.rs` as one test binary, so the
folder is a single binary made of small files rather than a thousand line one:

```
tests/screen/
├── main.rs      only `mod` lines, one per file below
├── support.rs   helpers the modules share
├── printing.rs
├── scrolling.rs
├── editing.rs
└── resizing.rs
```

A module inside the folder reaches a shared helper through the crate root:
`use crate::support::write;`, and a `tests/common/mod.rs` shared with other
binaries is pulled in by `main.rs` with `#[path = "../common/mod.rs"]`.

A test that has to set an environment variable goes in a file of its own. The
environment is process wide and the tests in one binary run in parallel, so a
file is the only scope that holds.

Skipping is a decision, not a default. A test that cannot run on a machine
with no fonts installed may skip there and must fail on CI, where an empty
system is a broken runner rather than a legitimate state.

## No bare numbers

A literal in the middle of a `match` or a condition is a value nobody can
check. Protocol numbers, key codes, mode numbers, colour indices, sizes and
thresholds all get a name first, and the name says what the value means
rather than what it is.

Protocol tables go in a private `mod` beside the code that reads them, named
after what the numbers select:

```rust
/// C0 and C1 control bytes, by the names the standard gives them.
mod code {
    /// BEL
    pub const BELL: u8 = 0x07;
}
```

A value with several named cases that the rest of the program also reasons
about becomes an enum instead, in `enums/`, with a `from_code` in `impls/`:
`DecMode`, `AnsiMode`. Prefer that whenever the value is passed around rather
than matched once.

The exceptions are `0` and `1` where they mean nothing but zero and one:
an empty count, a single step, the first element.

## Comments

The code carries the meaning; comments cover only what it cannot. A comment
earns its place when it explains a decision, a non-obvious invariant, or a
behaviour copied from another terminal, not when it restates a signature.

## Looking at the chrome

The window writes one frame to a file and exits, which beats screenshotting a
window that has to be frontmost and has to clear the recording permission:

```
CARET_SNAPSHOT=/tmp/shot.bmp CARET_SNAPSHOT_KEYS='ctrl+b,c,ctrl+b,%,ctrl+b,e' caret
```

`CARET_SNAPSHOT_KEYS` is a comma-separated list of bindings, each in the same
spelling `keys.direct` uses, performed before the capture, one key per entry,
so a prefixed command is two of them. The pixels are what shipped, so colours
can be measured from the file rather than guessed at.

## Dependencies

`caret-core` and `caret-layout` have none, and never will: they are pure domain and
pure geometry, which is what makes them exhaustively testable. Dependencies
enter only at the edges (PTY, fonts, windowing).
