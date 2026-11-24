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

/// How long the cursor goes on blinking after the last key or click.
///
/// Flashing the cursor means handing the compositor an entire new frame, so
/// one cell of animation costs a full-screen redraw twice a second for as
/// long as the window is open. The blink is there to help you find the
/// cursor while you are working, so it runs while you are and settles into a
/// steady cursor when you stop.
const BLINK_REST: Duration = Duration::from_secs(10);

/// How long a snapshot waits after typing before it captures.
const SETTLE: Duration = Duration::from_millis(700);

/// A terminal in a window of its own.
pub struct WindowApp {
    session: Session,
    painter: Option<Painter>,
    window: Option<Arc<Window>>,
    context: Option<softbuffer::Context<Arc<Window>>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    modifiers: ModifiersState,
    pointer: (u16, u16),
    /// Where the pointer is in pixels, for the chrome, which is not on the
    /// cell grid.
    pointer_px: (f64, f64),
    hover: StripHit,
    /// The shape the pointer is currently wearing.
    shape: CursorIcon,
    strip: StripLayout,
    scale: f32,
    blink: Blink,
    /// Whether the window has the keyboard. An unfocused window has no
    /// business blinking a cursor at a nobody, and blinking is the one thing
    /// that makes an idle terminal redraw.
    focused: bool,
    /// When the window last had output, a key or a click.
    active_at: Instant,
    /// When somebody last typed or clicked, which is what the cursor blinks
    /// for.
    typed_at: Instant,
    snapshot: Option<Snapshot>,
    /// When the snapshot is due; `None` until the window is up.
    snapshot_at: Option<std::time::Instant>,
    exiting: bool,
    title: String,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    signals::watch_termination();
    let session = Session::new()?;
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = WindowApp {
        session,
        painter: None,
        window: None,
        context: None,
        surface: None,
        modifiers: ModifiersState::empty(),
        pointer: (0, 0),
        pointer_px: (0.0, 0.0),
        hover: StripHit::Background,
        shape: CursorIcon::Default,
        strip: StripLayout::default(),
        scale: 1.0,
        blink: Blink::new(),
        focused: true,
        active_at: Instant::now(),
        typed_at: Instant::now(),
        snapshot: Snapshot::from_env(),
        snapshot_at: None,
        exiting: false,
        title: String::new(),
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}

impl WindowApp {
    /// Whether the cursor should be blinking at this moment.
    ///
    /// Three things have to hold: the configuration and the program in the
    /// pane both want it, the window has the keyboard, and somebody has used
    /// it recently. A window nobody is working in holds a steady cursor and
    /// draws nothing at all.
    fn blinking(&self) -> bool {
        self.focused && self.typed_at.elapsed() < BLINK_REST && self.session.cursor_blinks()
    }

    /// A painter with the glyphs rasterised for `scale`.
    ///
    /// Device pixels, not logical ones: a cache built for one display and
    /// used on another gives text at the wrong size, because the font was
    /// measured against a scale the window no longer has.
    fn painter_at(&self, scale: f32) -> Result<Painter, String> {
        let font = self.session.font_config().clone();
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

        let padding = (self.session.window_config().padding as f32 * scale) as usize;
        let mut painter = Painter::new(cache, padding, font.builtin_box_drawing);
        painter.chrome = painter.strip_height();
        Ok(painter)
    }

    /// Builds the window and the glyph cache.
    ///
    /// The cache is created at the display's scale, so text is rasterised at
    /// device resolution rather than scaled up and blurred.
    fn start(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        let attributes = chrome_attributes(Window::default_attributes().with_title("caret"));
        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .map_err(|e| e.to_string())?,
        );

        let scale = window.scale_factor() as f32;
        let painter = self.painter_at(scale)?;
        self.scale = scale;
        self.session.take_over_tab_bar();

        let window_config = self.session.window_config();
        let (width, height) = painter.window_size(window_config.cols, window_config.rows);
        let _ = window.request_inner_size(PhysicalSize::new(width, height));

        let context = softbuffer::Context::new(window.clone()).map_err(|e| e.to_string())?;
        let surface =
            softbuffer::Surface::new(&context, window.clone()).map_err(|e| e.to_string())?;

        if let Some(snapshot) = &self.snapshot {
            self.snapshot_at = Some(std::time::Instant::now() + snapshot.after);
            for binding in snapshot.keys.clone() {
                let _ = self.session.handle_key(KeyPress {
                    binding,
                    bytes: Vec::new(),
                });
            }
        }

        self.session.set_native_window(true);
        self.painter = Some(painter);
        self.context = Some(context);
        self.surface = Some(surface);
        self.window = Some(window);
        Ok(())
    }

    fn redraw(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let blinking = self.blinking();
        let (band, cell_w) = self
            .painter
            .as_ref()
            .map_or((0, 1), |p| (p.chrome as u32, p.cell.0 as u32));
        let Some(painter) = self.painter.as_mut() else {
            return;
        };
        let Some(surface) = self.surface.as_mut() else {
            return;
        };

        let size = window.inner_size();
        let (width, height) = (size.width.max(1), size.height.max(1));
        let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) else {
            return;
        };
        if surface.resize(w, h).is_err() {
            return;
        }

        let (cols, rows) = painter.grid_size(width as usize, height as usize);
        self.session.set_size(cols, rows);

        let model = tab_strip(&self.session, self.scale, cell_w, band);
        self.strip = model.layout(width);

        let (frame, cursor) = self.session.paint();
        let background = self.session.theme().bg;

        let Ok(mut buffer) = surface.buffer_mut() else {
            return;
        };
        painter.paint(
            &mut buffer,
            (width as usize, height as usize),
            &frame,
            background,
        );

        if let Some(mut cursor) = cursor {
            if blinking && !self.blink.visible() {
                cursor.alpha = 0;
            }
            let under = frame.get(cursor.at.0, cursor.at.1);
            let mut canvas = Canvas {
                pixels: &mut buffer,
                width: width as usize,
                height: height as usize,
            };
            painter.draw_cursor(&mut canvas, cursor, under);
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
            &self.strip,
            self.hover,
            self.session.theme(),
        );
        let due = self
            .snapshot_at
            .is_some_and(|at| std::time::Instant::now() >= at);
        if due {
            // Typing waits for the window to settle, and the capture waits
            // again after it so the shell has echoed the line.
            if let Some(text) = self.snapshot.as_mut().and_then(|s| s.typing.take()) {
                // One key at a time rather than a paste: a shell marks pasted
                // text as a region of its own and colours it, which is not
                // what typing looks like.
                for ch in text.chars() {
                    let _ = self.session.handle_key(KeyPress {
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
        self.session.mark_drawn();

        let title = self.session.window_title();
        if title != self.title {
            window.set_title(&title);
            self.title = title;
        }
    }

    /// Turns a pixel position into the cell under it.
    /// Puts a resize pointer over a border that can be dragged.
    ///
    /// A border that resizes but gives no sign of it is a border nobody
    /// finds; this is the only hint that dragging is possible at all.
    fn show_resize_shape(&mut self, cell: (u16, u16)) {
        let wanted = match self.session.resize_axis_at(cell.0, cell.1) {
            Some(Axis::Horizontal) => CursorIcon::ColResize,
            Some(Axis::Vertical) => CursorIcon::RowResize,
            None => CursorIcon::Default,
        };
        if wanted != self.shape {
            self.shape = wanted;
            if let Some(window) = &self.window {
                window.set_cursor(wanted);
            }
        }
    }

    fn cell_at(&self, x: f64, y: f64) -> (u16, u16) {
        let Some(painter) = self.painter.as_ref() else {
            return (0, 0);
        };
        let origin = (painter.chrome + painter.padding) as f64;
        let col = ((x - painter.padding as f64).max(0.0) / painter.cell.0 as f64) as u16;
        let row = ((y - origin).max(0.0) / painter.cell.1 as f64) as u16;
        (col, row)
    }

    /// Whether the pointer is over the strip rather than the grid.
    fn in_strip(&self, y: f64) -> bool {
        self.painter.as_ref().is_some_and(|p| y < p.chrome as f64)
    }

    fn strip_hit(&self) -> StripHit {
        let Some(painter) = self.painter.as_ref() else {
            return StripHit::Background;
        };
        let model = tab_strip(
            &self.session,
            self.scale,
            painter.cell.0 as u32,
            painter.chrome as u32,
        );
        model.hit(
            &self.strip,
            self.pointer_px.0 as i32,
            self.pointer_px.1 as i32,
        )
    }

    fn mouse(&mut self, button: MouseButton, kind: MouseKind) {
        self.session.handle_mouse(MouseEvent {
            button,
            kind,
            col: self.pointer.0,
            row: self.pointer.1,
            mods: keys::modifiers(self.modifiers),
        });
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        if let Err(e) = self.start(event_loop) {
            eprintln!("caret: {e}");
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Anything the window is told about is a sign of life, and the panes
        // are serviced at full speed for a moment afterwards.
        if !matches!(event, WindowEvent::RedrawRequested) {
            self.active_at = Instant::now();
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            // The cursor stops blinking when the keyboard goes elsewhere.
            // A window nobody is typing into has nothing to announce, and
            // the blink is the only reason an idle terminal redraws at all.
            WindowEvent::Focused(has_keyboard) => {
                self.focused = has_keyboard;
                self.session.set_window_focused(has_keyboard);
                self.typed_at = Instant::now();
                self.blink.restart();
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::RedrawRequested => self.redraw(),

            WindowEvent::Resized(_) => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            // Dragged onto a display with a different pixel density. The
            // glyphs were rasterised for the old one, so they have to be
            // drawn again or the text comes out the wrong size.
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let scale = scale_factor as f32;
                if (scale - self.scale).abs() > f32::EPSILON {
                    match self.painter_at(scale) {
                        Ok(painter) => {
                            self.scale = scale;
                            self.painter = Some(painter);
                        }
                        Err(e) => eprintln!("caret: {e}"),
                    }
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if !event.state.is_pressed() {
                    return;
                }
                let text = event.text.as_ref().map(|t| t.as_str());
                if let Some(press) = keys::key_press(&event.logical_key, text, self.modifiers) {
                    self.typed_at = Instant::now();
                    self.blink.restart();
                    let _ = self.session.handle_key(press);
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.pointer_px = (position.x, position.y);
                let hovered = self.strip_hit();
                if hovered != self.hover {
                    self.hover = hovered;
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                if self.in_strip(position.y) {
                    return;
                }

                let cell = self.cell_at(position.x, position.y);
                if cell != self.pointer {
                    self.pointer = cell;
                    self.show_resize_shape(cell);
                    // Motion only matters while dragging or to an application
                    // that asked for it; the session decides which.
                    self.mouse(MouseButton::None, MouseKind::Motion);
                }
            }

            WindowEvent::CursorLeft { .. } if self.hover != StripHit::Background => {
                self.hover = StripHit::Background;
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                self.typed_at = Instant::now();
                if self.in_strip(self.pointer_px.1) {
                    if state.is_pressed() {
                        let _ = self.session.on_strip_hit(self.strip_hit());
                    }
                    return;
                }
                if let Some(button) = keys::mouse_button(button) {
                    self.mouse(button, keys::mouse_kind(state));
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
                self.mouse(button, MouseKind::Press);
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // A `kill` or a logout stops the loop rather than the process, so
        // every pane's child is hung up on the way out instead of being
        // orphaned onto a pty nothing is reading.
        if self.exiting || !self.session.is_running() || signals::ending() {
            event_loop.exit();
            return;
        }
        if self.session.pump(0).unwrap_or(false) {
            self.active_at = Instant::now();
        }

        let beat = self.blinking() && self.blink.flipped();
        // A snapshot waiting on a delay needs a frame of its own: by the time
        // it falls due the window may have had nothing to redraw for seconds.
        let due = self
            .snapshot_at
            .is_some_and(|at| self.snapshot.is_some() && Instant::now() >= at);
        if self.session.needs_draw() || beat || due {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        // The blink has to land on its own edge, which can be sooner than
        // the next service tick is due.
        let mut wake = if self.active_at.elapsed() < BUSY {
            FRAME
        } else {
            IDLE
        };
        if self.blinking() {
            wake = wake.min(self.blink.until_flip());
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + wake));
    }
}

/// The strip as the layout sees it.
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
