# Known limitations

Referenced from the code where each one bites, so the behaviour is a recorded
decision rather than a surprise.

## Grid

**A cluster is drawn by stacking its characters, not by shaping them.** A
letter and its marks share a cell, and the window draws the base glyph and
then each mark at the same origin. A combining glyph advances by nothing and
is designed to sit over what precedes it, so this is right for marks; a
script that needs its letters reshaped by their neighbours is not drawn the
way it should be, and an emoji built out of several characters shows its base
rather than the single glyph a shaping engine would pick.
*`caret-app/src/impls/window/paint.rs`*

**A pane holds the text of 32,768 clusters at once.** Cells that need more
than one character point at a table beside the grid, which is compacted
against the grid before it is allowed to fill, so the limit is reached only
when that many different clusters are genuinely on screen or in scrollback.
Past it a cell keeps its base character and loses what combined with it.
*`caret-core/src/impls/cluster_table.rs`*

**A search matches a cluster by the character it is built on.** An accent
that has a precomposed form becomes that single character as it arrives, so
searching for the word finds it; a cluster with no precomposed form is found
by its base letter rather than by the marks on it.
*`caret-core/src/impls/screen/search.rs`*

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
