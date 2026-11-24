#[allow(unused_imports)]
use crate::prelude::*;

use caret_theme::UiColors;
use caret_ui::{PxRect, StripHit, StripLayout, TabItem, TabStrip};

use crate::impls::window::paint::Painter;

/// Draws the tab strip in pixels.
///
/// Nothing here is quantised to the cell box: tabs are pills with antialiased
/// corners at whatever width the layout gave them, which is the whole reason
/// the strip is not part of the composed grid.
pub fn draw(
    painter: &mut Painter,
    canvas: &mut Canvas,
    strip: &TabStrip,
    layout: &StripLayout,
    hover: StripHit,
    theme: &Theme,
) {
    let ui = &theme.ui;
    canvas.fill(0, 0, (canvas.width, strip.height as usize), ui.chrome_bg);

    for (index, rect) in layout.tabs.iter().enumerate() {
        let Some(tab) = strip.tabs.get(index) else {
            continue;
        };
        draw_tab(
            painter,
            canvas,
            strip,
            layout,
            Slot {
                index,
                rect: *rect,
                tab,
            },
            hover,
            ui,
        );
    }

    let plus = layout.new_tab;
    if hover == StripHit::NewTab {
        canvas.rounded(
            plus.centred(strip.px(22.0), strip.px(22.0)),
            strip.px(TabStrip::RADIUS) as f32,
            mix(ui.chrome_bg, ui.tab_active_bg, 1, 1),
        );
    }
    centred_text(painter, canvas, plus, "+", ui.tab_inactive_fg, false);

    // A hairline, not a border: the strip and the grid are one surface, and a
    // full-strength rule would cut the window in two.
    canvas.fill(
        0,
        strip.height as usize - 1,
        (canvas.width, 1),
        mix(ui.chrome_bg, ui.border, 2, 3),
    );
}

struct Slot<'a> {
    index: usize,
    rect: PxRect,
    tab: &'a TabItem,
}

fn draw_tab(
    painter: &mut Painter,
    canvas: &mut Canvas,
    strip: &TabStrip,
    layout: &StripLayout,
    slot: Slot,
    hover: StripHit,
    ui: &UiColors,
) {
    let Slot { index, rect, tab } = slot;
    let active = index == strip.active;
    let hovered = matches!(hover, StripHit::Tab(i) | StripHit::Close(i) if i == index);

    let pill = rect.inset(strip.px(TabStrip::PILL_INSET), strip.px(5.0));
    let radius = strip.px(TabStrip::RADIUS) as f32;
    if active || hovered {
        canvas.rounded(pill, radius, pill_bg(active, hovered, ui));
    }

    let (fg, number_fg) = if active {
        (ui.tab_active_fg, ui.accent)
    } else {
        (ui.tab_inactive_fg, ui.tab_inactive_fg)
    };

    let pad = strip.px(TabStrip::PAD);
    let cell_w = painter.cell.0 as i32;
    let top = text_top(painter, rect);

    let mut x = pill.x + pad as i32;
    if strip.numbered() {
        let number = (index + 1).to_string();
        x = painter.text_px(canvas, (x, top), pill.w, &number, number_fg, false);
        x += cell_w / 2;
    }

    // The badge and the close button are measured out of the title's room
    // first, so a long title is shortened rather than overrun.
    let close = layout.close_for(index).filter(|_| active || hovered);
    let badge = tab.badge();
    let reserved = close.map_or(0, |c| c.w)
        + badge
            .as_ref()
            .map_or(0, |b| (b.chars().count() as u32 + 2) * cell_w as u32);
    let room = (pill.right() - pad as i32 - x).max(0) as u32;

    let after = painter.text_px(
        canvas,
        (x, top),
        room.saturating_sub(reserved),
        &tab.title,
        fg,
        active,
    );

    if let Some(badge) = badge {
        draw_badge(
            painter,
            canvas,
            (after + cell_w / 2, rect),
            &badge,
            Tone {
                fill: pill_bg(active, hovered, ui),
                fg,
                accent: ui.accent,
            },
            tab.zoomed,
        );
    }
    if let Some(close) = close {
        if hover == StripHit::Close(index) {
            canvas.rounded(
                close,
                close.w as f32 / 2.0,
                mix(ui.chrome_bg, ui.tab_inactive_fg, 1, 3),
            );
        }
        centred_text(
            painter,
            canvas,
            close,
            "\u{00d7}",
            ui.tab_inactive_fg,
            false,
        );
    }
}

/// The colours a badge borrows from the tab it sits in.
struct Tone {
    fill: Rgb,
    fg: Rgb,
    accent: Rgb,
}

/// Halfway to the active fill on hover: present under the pointer, but never
/// mistakable for the selected tab.
fn pill_bg(active: bool, hovered: bool, ui: &UiColors) -> Rgb {
    match (active, hovered) {
        (true, _) => ui.tab_active_bg,
        (false, true) => mix(ui.chrome_bg, ui.tab_active_bg, 1, 2),
        (false, false) => ui.chrome_bg,
    }
}

fn draw_badge(
    painter: &mut Painter,
    canvas: &mut Canvas,
    at: (i32, PxRect),
    text: &str,
    tone: Tone,
    zoomed: bool,
) {
    let (x, rect) = at;
    let (cell_w, cell_h) = (painter.cell.0 as u32, painter.cell.1 as u32);
    let w = (text.chars().count() as u32 + 1) * cell_w;
    let h = (cell_h as f32 * 1.1) as u32;
    let pill = PxRect::new(x, rect.y + (rect.h.saturating_sub(h) / 2) as i32, w, h);

    // Quiet by default: the count is a fact about the tab, not an alert. A
    // zoomed tab is the exception, because something is hidden.
    let (fill, fg) = match zoomed {
        true => (tone.accent, tone.fill),
        false => (mix(tone.fill, tone.fg, 1, 4), tone.fg),
    };
    canvas.rounded(pill, h as f32 / 2.0, fill);
    centred_text(painter, canvas, pill, text, fg, false);
}

fn centred_text(
    painter: &mut Painter,
    canvas: &mut Canvas,
    rect: PxRect,
    text: &str,
    colour: Rgb,
    bold: bool,
) {
    let width = text.chars().count() as u32 * painter.cell.0 as u32;
    let x = rect.x + (rect.w.saturating_sub(width) / 2) as i32;
    painter.text_px(
        canvas,
        (x, text_top(painter, rect)),
        rect.w.max(width),
        text,
        colour,
        bold,
    );
}

/// Where a line of text starts so it sits on the rectangle's centre line.
fn text_top(painter: &Painter, rect: PxRect) -> i32 {
    rect.y + (rect.h.saturating_sub(painter.cell.1 as u32) / 2) as i32
}

/// Part of the way from one colour to another.
fn mix(base: Rgb, toward: Rgb, numerator: i32, denominator: i32) -> Rgb {
    let channel = |a: u8, b: u8| {
        let (a, b) = (a as i32, b as i32);
        (a + (b - a) * numerator / denominator).clamp(0, 255) as u8
    };
    Rgb::new(
        channel(base.r, toward.r),
        channel(base.g, toward.g),
        channel(base.b, toward.b),
    )
}
