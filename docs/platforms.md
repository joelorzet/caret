# Platform support

| Crate | macOS | Linux | Windows |
|-------|-------|-------|---------|
| `caret-core` | yes | yes | yes, no platform code |
| `caret-layout` | yes | yes | yes, no platform code |
| `caret-theme` | yes | yes | yes, no platform code |
| `caret-config` | yes | yes | yes, path lookup falls back to `HOME` |
| `caret-vt` | yes | yes | yes, no platform code |
| `caret-render` | yes | yes | yes, no platform code |
| `caret-features` | yes | yes | yes, no platform code |
| `caret-ui` | yes | yes | yes, no platform code |
| `caret-update` | yes | yes | yes, curl and the shell tools |
| `caret-pty` | yes | yes | **no**: needs ConPTY |
| `caret-app` host mode | yes | yes | **no**: needs ConPTY and a console backend |

Nine of the eleven crates have no platform-specific code at all. That is a
consequence of the layering rather than an accident: everything that touches
the operating system is in `caret-pty` and the host loop.

## What Windows needs

`caret-pty` is POSIX: `posix_openpt`, `grantpt`, `TIOCSCTTY`, `TIOCSWINSZ`,
`termios` and `poll`. These are identical on macOS and Linux, so one
implementation covers both.

Windows has no pseudoterminal. It has ConPTY, which differs in three ways that
matter:

1. **Creation.** `CreatePseudoConsole` returns an `HPCON` plus two pipe
   handles, instead of a master fd and a slave path.
2. **Waiting.** There is no `poll` over handles with the same semantics; it
   needs overlapped (asynchronous) reads or a thread per pipe plus an IOCP.
   This is the one place the "one loop, no thread per pane" property has to be
   re-established differently.
3. **Resizing.** `ResizePseudoConsole` replaces the `TIOCSWINSZ` ioctl.

The public API, `PtyPool`, `PtySpec`, `PtyEvent`, `PtyId`, avoids fd-shaped
types in its signatures so a second backend can be added under
`impls/windows/` without changing any caller. `Pty::master()` returning a
`RawFd` is the one exception and would move behind the backend boundary.

Until then the crate fails to compile on Windows with a message saying so,
rather than appearing to work.

## Recommendation

Add it when there is a reason to. The split is already in place, the work is
contained to one crate, and nothing above `caret-pty` has to change.
