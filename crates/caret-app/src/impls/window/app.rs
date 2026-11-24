#[allow(unused_imports)]
use crate::prelude::*;

use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};

use caret_render::{FontLibrary, GlyphCache};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::ModifiersState;
use winit::window::{CursorIcon, Window, WindowId};

use caret_ui::{StripHit, StripLayout, TabStrip};

use crate::impls::window::keys;
use crate::impls::window::paint::Painter;
use crate::impls::window::snapshot::Snapshot;
use crate::impls::window::strip;

/// How often the pseudoterminals are serviced while anything is happening.
const FRAME: Duration = Duration::from_millis(8);

/// And how often once nothing has been for a while.
///
/// A pane that prints on its own is noticed one tick later at worst, which
/// is not something anybody can see, and the window stops waking the
/// processor 125 times a second to be told there is nothing to do. A key or
/// a click is an event in its own right and wakes the loop immediately, so
/// typing is never waiting on this.
const IDLE: Duration = Duration::from_millis(60);

/// How long after the last sign of life the window keeps to `FRAME`.
const BUSY: Duration = Duration::from_millis(500);

/// How long a snapshot waits after typing before it captures.
const SETTLE: Duration = Duration::from_millis(700);

/// Every window the program has open, and what they share.
///
/// The event loop and the rasterised glyphs belong here rather than to a
/// window: a second window on the same display draws with the same glyphs,
/// and rasterising them again would cost a megabyte to say the same thing
/// twice. A window that moves to a display of another scale gets a painter
/// of its own, because the glyphs genuinely differ there.
pub struct WindowApp {
    windows: Vec<WindowState>,
    /// Painters by the scale they rasterised for, in hundredths.
    painters: Vec<(u32, Painter)>,
    /// The session built before the loop started, waiting for a window to
    /// put it in. A failure to start one is worth an exit code, which is
    /// why it is not left until the loop is running.
    pending: Option<Session>,
    snapshot: Option<Snapshot>,
    /// When the snapshot is due; `None` until the first window is up.
    snapshot_at: Option<std::time::Instant>,
    exiting: bool,
}

/// A scale as a key: floating point does not compare, and a hundredth of a
/// scale factor is finer than any display reports.
fn scale_key(scale: f32) -> u32 {
    (scale * 100.0).round() as u32
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    signals::watch_termination();
    let session = Session::new()?;
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = WindowApp {
        windows: Vec::new(),
        painters: Vec::new(),
        pending: Some(session),
        snapshot: Snapshot::from_env(),
        snapshot_at: None,
        exiting: false,
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}

impl WindowApp {
    fn at(&self, id: WindowId) -> Option<usize> {
        self.windows.iter().position(|w| w.window.id() == id)
    }

    /// The painter for a scale, rasterising the glyphs the first time that
    /// scale is asked for.
    ///
    /// Device pixels, not logical ones: a cache built for one display and
    /// used on another gives text at the wrong size, because the font was
    /// measured against a scale the window no longer has.
    fn painter_for(
        &mut self,
        scale: f32,
        font: &caret_config::FontConfig,
        padding: u16,
    ) -> Result<usize, String> {
        let key = scale_key(scale);
        if let Some(at) = self.painters.iter().position(|(k, _)| *k == key) {
            return Ok(at);
        }

        let library = FontLibrary::load();
        let cache = GlyphCache::new(
            &library,
            &font.family,
            &font.fallback,
            font.size * scale,
            font.line_height,
            font.letter_spacing * scale,
        )
        .ok_or_else(|| {
            format!(
                "no usable font: none of {:?} or its fallbacks are installed",
                font.family
            )
        })?;

        let padding = (padding as f32 * scale) as usize;
        let mut painter = Painter::new(cache, padding, font.builtin_box_drawing);
        painter.ligatures = font.ligatures;
        painter.chrome = painter.strip_height();
        self.painters.push((key, painter));
        Ok(self.painters.len() - 1)
    }

    /// Opens a window for `session`.
    fn open(&mut self, event_loop: &ActiveEventLoop, mut session: Session) -> Result<(), String> {
        let attributes = chrome_attributes(Window::default_attributes().with_title("caret"));
        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .map_err(|e| e.to_string())?,
        );

        let scale = window.scale_factor() as f32;
        let font = session.font_config().clone();
        let padding = session.window_config().padding;
        let at = self.painter_for(scale, &font, padding)?;
        session.take_over_tab_bar();

        let window_config = session.window_config();
        let (width, height) = self.painters[at]
            .1
            .window_size(window_config.cols, window_config.rows);
        let _ = window.request_inner_size(PhysicalSize::new(width, height));

        let context = softbuffer::Context::new(window.clone()).map_err(|e| e.to_string())?;
        let surface =
            softbuffer::Surface::new(&context, window.clone()).map_err(|e| e.to_string())?;

        // A snapshot is a development capture of the first window, and the
        // keys it asks for are typed into that one.
        if self.windows.is_empty() {
            if let Some(snapshot) = &self.snapshot {
                self.snapshot_at = Some(std::time::Instant::now() + snapshot.after);
                for binding in snapshot.keys.clone() {
                    let _ = session.handle_key(KeyPress {
                        binding,
                        bytes: Vec::new(),
                    });
                }
            }
        }

        session.set_native_window(true);
        self.windows.push(WindowState {
            session,
            window,
            _context: context,
            surface,
            scale,
            modifiers: ModifiersState::empty(),
            pointer: (0, 0),
            pointer_px: (0.0, 0.0),
            hover: StripHit::Background,
            shape: CursorIcon::Default,
            strip: StripLayout::default(),
            blink: Blink::new(),
            focused: true,
            active_at: Instant::now(),
            typed_at: Instant::now(),
            title: String::new(),
        });
        Ok(())
    }

    /// Takes a window away, hanging up every pane in it, and ends the
    /// program when the last one goes.
    fn close(&mut self, at: usize, event_loop: &ActiveEventLoop) {
        if at < self.windows.len() {
            self.windows.remove(at);
        }
        // Nothing else holds a window's painter but the windows, and a scale
        // nobody is drawing at any more is a cache nobody will read.
        self.painters
            .retain(|(key, _)| self.windows.iter().any(|w| scale_key(w.scale) == *key));
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }

    fn redraw(&mut self, at: usize) {
        let Some(state) = self.windows.get(at) else {
            return;
        };
        let scale = state.scale;
        let font = state.session.font_config().clone();
        let padding = state.session.window_config().padding;
        let Ok(pi) = self.painter_for(scale, &font, padding) else {
            return;
        };
        let painter = &mut self.painters[pi].1;
        let state = &mut self.windows[at];

        let window = state.window.clone();
        let blinking = state.blinking();
        let (band, cell_w) = (painter.chrome as u32, painter.cell.0 as u32);

        let size = window.inner_size();
        let (width, height) = (size.width.max(1), size.height.max(1));
        let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) else {
            return;
        };
        if state.surface.resize(w, h).is_err() {
            return;
        }

        let (cols, rows) = painter.grid_size(width as usize, height as usize);
        state.session.set_size(cols, rows);

        let model = tab_strip(&state.session, state.scale, cell_w, band);
        state.strip = model.layout(width);
        let (frame, cursor) = state.session.paint();
        let background = state.session.theme().bg;

        let Ok(mut buffer) = state.surface.buffer_mut() else {
            return;
        };
        painter.paint(
            &mut buffer,
            (width as usize, height as usize),
            &frame,
            background,
        );

        if let Some(mut cursor) = cursor {
            if blinking && !state.blink.visible() {
                cursor.alpha = 0;
            }
            let under = frame.get(cursor.at.0, cursor.at.1);
            // What the cell shows rather than what it holds: a cell with a
            // mark on it carries a handle, not a character.
            let under_text = under.map(|cell| frame.grapheme(&cell).to_string());
            let mut canvas = Canvas {
                pixels: &mut buffer,
                width: width as usize,
                height: height as usize,
            };
            painter.draw_cursor(&mut canvas, cursor, under, under_text.as_deref());
        }

        let mut canvas = Canvas {
            pixels: &mut buffer,
            width: width as usize,
            height: height as usize,
        };
        strip::draw(
            painter,
            &mut canvas,
            &model,
            &state.strip,
            state.hover,
            state.session.theme(),
        );

        // The snapshot belongs to the first window, which is the only one a
        // capture ever opens.
        let due = at == 0
            && self
                .snapshot_at
                .is_some_and(|due| std::time::Instant::now() >= due);
        if due {
            // Typing waits for the window to settle, and the capture waits
            // again after it so the shell has echoed the line.
            if let Some(text) = self.snapshot.as_mut().and_then(|s| s.typing.take()) {
                // One key at a time rather than a paste: a shell marks pasted
                // text as a region of its own and colours it, which is not
                // what typing looks like.
                for ch in text.chars() {
                    let _ = state.session.handle_key(KeyPress {
                        binding: KeyBinding::ch(ch),
                        bytes: ch.to_string().into_bytes(),
                    });
                }
                self.snapshot_at = Some(std::time::Instant::now() + SETTLE);
            } else if let Some(snapshot) = self.snapshot.take() {
                if let Err(e) = snapshot.write(&buffer, width as usize, height as usize) {
                    eprintln!("caret: snapshot: {e}");
                }
                self.exiting = true;
            }
        }

        let _ = buffer.present();
        state.session.mark_drawn();

        let title = state.session.window_title();
        if title != state.title {
            window.set_title(&title);
            state.title = title;
        }
    }

    /// The painter a window draws with, if its glyphs have been made yet.
    fn painter_of(&self, at: usize) -> Option<&Painter> {
        let key = scale_key(self.windows.get(at)?.scale);
        self.painters
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, painter)| painter)
    }

    fn strip_hit(&self, at: usize) -> StripHit {
        let (Some(state), Some(painter)) = (self.windows.get(at), self.painter_of(at)) else {
            return StripHit::Background;
        };
        let model = tab_strip(
            &state.session,
            state.scale,
            painter.cell.0 as u32,
            painter.chrome as u32,
        );
        model.hit(
            &state.strip,
            state.pointer_px.0 as i32,
            state.pointer_px.1 as i32,
        )
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.windows.is_empty() {
            return;
        }
        let Some(session) = self.pending.take() else {
            return;
        };
        if let Err(e) = self.open(event_loop, session) {
            eprintln!("caret: {e}");
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(at) = self.at(id) else {
            return;
        };
        // Anything a window is told about is a sign of life, and its panes
        // are serviced at full speed for a moment afterwards.
        if !matches!(event, WindowEvent::RedrawRequested) {
            self.windows[at].active_at = Instant::now();
        }

        match event {
            WindowEvent::CloseRequested => self.close(at, event_loop),

            // The cursor stops blinking when the keyboard goes elsewhere.
            // A window nobody is typing into has nothing to announce, and
            // the blink is the only reason an idle terminal redraws at all.
            WindowEvent::Focused(has_keyboard) => {
                let state = &mut self.windows[at];
                state.focused = has_keyboard;
                state.session.set_window_focused(has_keyboard);
                state.typed_at = Instant::now();
                state.blink.restart();
                state.window.request_redraw();
            }

            WindowEvent::RedrawRequested => self.redraw(at),

            WindowEvent::Resized(_) => self.windows[at].window.request_redraw(),

            // Dragged onto a display with a different pixel density. The
            // glyphs were rasterised for the old one, so they have to be
            // drawn again or the text comes out the wrong size.
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let scale = scale_factor as f32;
                if (scale - self.windows[at].scale).abs() > f32::EPSILON {
                    let font = self.windows[at].session.font_config().clone();
                    let padding = self.windows[at].session.window_config().padding;
                    match self.painter_for(scale, &font, padding) {
                        Ok(_) => self.windows[at].scale = scale,
                        Err(e) => eprintln!("caret: {e}"),
                    }
                }
                self.windows[at].window.request_redraw();
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                self.windows[at].modifiers = modifiers.state();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if !event.state.is_pressed() {
                    return;
                }
                let text = event.text.as_ref().map(|t| t.as_str());
                let state = &mut self.windows[at];
                if let Some(press) = keys::key_press(&event.logical_key, text, state.modifiers) {
                    state.typed_at = Instant::now();
                    state.blink.restart();
                    let _ = state.session.handle_key(press);
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.windows[at].pointer_px = (position.x, position.y);
                let hovered = self.strip_hit(at);
                if hovered != self.windows[at].hover {
                    self.windows[at].hover = hovered;
                    self.windows[at].window.request_redraw();
                }
                let Some(painter) = self.painter_of(at) else {
                    return;
                };
                if position.y < painter.chrome as f64 {
                    return;
                }

                let cell = cell_at(painter, position.x, position.y);
                if cell != self.windows[at].pointer {
                    self.windows[at].pointer = cell;
                    self.windows[at].show_resize_shape(cell);
                    // Motion only matters while dragging or to an application
                    // that asked for it; the session decides which.
                    self.windows[at].mouse(MouseButton::None, MouseKind::Motion);
                }
            }

            WindowEvent::CursorLeft { .. } if self.windows[at].hover != StripHit::Background => {
                self.windows[at].hover = StripHit::Background;
                self.windows[at].window.request_redraw();
            }

            WindowEvent::MouseInput { state, button, .. } => {
                self.windows[at].typed_at = Instant::now();
                let in_strip = self
                    .painter_of(at)
                    .is_some_and(|painter| self.windows[at].pointer_px.1 < painter.chrome as f64);
                if in_strip {
                    if state.is_pressed() {
                        let hit = self.strip_hit(at);
                        let _ = self.windows[at].session.on_strip_hit(hit);
                    }
                    return;
                }
                if let Some(button) = keys::mouse_button(button) {
                    self.windows[at].mouse(button, keys::mouse_kind(state));
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let lines = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(position) => position.y as f32 / 20.0,
                };
                if lines.abs() < 0.1 {
                    return;
                }
                let button = if lines > 0.0 {
                    MouseButton::WheelUp
                } else {
                    MouseButton::WheelDown
                };
                self.windows[at].mouse(button, MouseKind::Press);
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // A `kill` or a logout stops the loop rather than the process, so
        // every pane's child is hung up on the way out instead of being
        // orphaned onto a pty nothing is reading.
        if self.exiting || signals::ending() {
            event_loop.exit();
            return;
        }

        // A window asking for another one, and windows that have run out of
        // panes. Both are answered before anything is drawn, so a window
        // that is going away is not painted first.
        let mut wanted = Vec::new();
        for at in 0..self.windows.len() {
            if self.windows[at].session.took_window_request() {
                wanted.push(at);
            }
        }
        for from in wanted {
            let cwd = self.windows[from].session.working_dir();
            match Session::beside(&self.windows[from].session, cwd) {
                Ok(session) => {
                    if let Err(e) = self.open(event_loop, session) {
                        eprintln!("caret: {e}");
                    }
                }
                Err(e) => eprintln!("caret: could not open a window: {e}"),
            }
        }

        if self.windows.iter().any(|w| w.session.is_quitting()) {
            event_loop.exit();
            return;
        }
        for at in (0..self.windows.len()).rev() {
            if !self.windows[at].session.is_running() {
                self.close(at, event_loop);
            }
        }
        if self.windows.is_empty() {
            event_loop.exit();
            return;
        }

        let mut soonest = IDLE;
        for at in 0..self.windows.len() {
            let state = &mut self.windows[at];
            if state.session.pump(0).unwrap_or(false) {
                state.active_at = Instant::now();
            }

            let beat = state.blinking() && state.blink.flipped();
            // A snapshot waiting on a delay needs a frame of its own: by the
            // time it falls due the window may have had nothing to redraw
            // for seconds.
            let due = at == 0
                && self.snapshot.is_some()
                && self.snapshot_at.is_some_and(|at| Instant::now() >= at);
            let state = &mut self.windows[at];
            if state.session.needs_draw() || beat || due {
                state.window.request_redraw();
            }

            // The blink has to land on its own edge, which can be sooner
            // than the next service tick is due.
            let mut wake = if state.active_at.elapsed() < BUSY {
                FRAME
            } else {
                IDLE
            };
            if state.blinking() {
                wake = wake.min(state.blink.until_flip());
            }
            soonest = soonest.min(wake);
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + soonest));
    }
}

/// Turns a pixel position into the cell under it.
fn cell_at(painter: &Painter, x: f64, y: f64) -> (u16, u16) {
    let origin = (painter.chrome + painter.padding) as f64;
    let col = ((x - painter.padding as f64).max(0.0) / painter.cell.0 as f64) as u16;
    let row = ((y - origin).max(0.0) / painter.cell.1 as f64) as u16;
    (col, row)
}

fn tab_strip(session: &Session, scale: f32, cell_width: u32, height: u32) -> TabStrip {
    TabStrip {
        tabs: session.tab_items(),
        active: session.active_tab_index(),
        inset_left: window_controls_width(scale),
        height,
        cell_width,
        scale,
    }
}

/// The window controls sit over the content once the title bar is hidden, so
/// the strip starts past them.
#[cfg(target_os = "macos")]
fn window_controls_width(scale: f32) -> u32 {
    (78.0 * scale).round() as u32
}

#[cfg(not(target_os = "macos"))]
fn window_controls_width(_scale: f32) -> u32 {
    0
}

/// Lets the strip occupy the title bar rather than sit below it, which is
/// what makes it read as the window's own tab bar instead of something
/// painted inside the content.
#[cfg(target_os = "macos")]
fn chrome_attributes(
    attributes: winit::window::WindowAttributes,
) -> winit::window::WindowAttributes {
    use winit::platform::macos::WindowAttributesExtMacOS;
    attributes
        .with_titlebar_transparent(true)
        .with_fullsize_content_view(true)
        .with_title_hidden(true)
}

#[cfg(not(target_os = "macos"))]
fn chrome_attributes(
    attributes: winit::window::WindowAttributes,
) -> winit::window::WindowAttributes {
    attributes
}
