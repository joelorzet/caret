# Known limitations

Referenced from the code where each one bites, so the behaviour is a recorded
decision rather than a surprise.

## Grid

**No combining characters.** A cell holds one `char`. Zero-width marks (an
accent arriving after its base letter) are dropped rather than composed.
Storing a grapheme cluster per cell would mean a side table and an indirection
on every cell, which is the one thing `caret-core` is built to avoid. Precomposed
characters work; decomposed ones lose the mark.
*`caret-core/src/impls/screen/printing.rs`*

## Panes

**Linux drops what a program wrote as it exited.** A pty is drained before
it is closed, so output a program wrote just before finishing is handed
over rather than thrown away with the pane. On Linux the kernel discards
whatever is still buffered the moment the last slave closes, so a command
that writes and exits between two wake-ups can lose its last words there,
where macOS keeps them. Holding a slave descriptor open in the parent
would fix it and is the obvious next step.
*`caret-pty/src/impls/pty_pool.rs`*

## Rendering

**The inline backend cannot use the font or Cmd settings.** `caret inline` draws
through the terminal that launched it, so that terminal owns the font and keeps
Cmd for itself. `font.family`, `size` and `letter_spacing` apply in the window
(`caret run`) only, and the shortcut panel hides Cmd bindings when they cannot
fire.

**The window renders on the CPU.** Glyphs are rasterised once and cached, then
blended into a `softbuffer` framebuffer. That is fast enough at a normal window
size but does more work per frame than a GPU backend would at 4K.

**A frame costs what it costs to hand over, not what it costs to draw.**
Measured on a full-screen retina window: painting every glyph takes 1 to 3ms,
and giving the finished buffer to the compositor takes **17 to 26ms**, because
the platform wants a fresh image each time and there is no partial update. So
the ceiling is around 40 frames a second at that size, and anything that
redraws on a timer is expensive whether or not much changed. It is why the
cursor stops blinking when the window loses focus or when nobody has typed for
ten seconds: one cell of animation was costing a full screen, twice a second,
all day. A GPU backend is the real fix.

**Ligatures are not shaped.** `font.ligatures` is accepted but the window draws
one glyph per cell, so `!=` stays two glyphs. Shaping needs a layout pass over
runs rather than per-cell rasterisation.

**No ligature detection.** `font.ligatures` is honoured as a setting, but
`caret fonts` cannot say which families actually provide ligatures; that needs
reading the GSUB feature table.

## Protocol

**Device control strings are discarded.** DCS sequences are parsed and
swallowed. Nothing that uses them (DECRQSS, Sixel, Kitty graphics) is
supported.
*`caret-vt/src/impls/performer/perform.rs`*

**Character sets are ignored.** SO, SI and the `ESC ( ` designators are parsed
and dropped; only the default set is drawn. Line-drawing via DEC Special
Graphics will render as letters.
*`caret-vt/src/impls/performer/control.rs`*

## Platform

**Windows is unsupported.** It needs a ConPTY backend; see
[platforms.md](platforms.md). The crate fails to compile there with a message
saying so rather than appearing to work.
