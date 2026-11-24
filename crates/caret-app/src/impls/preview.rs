use caret_layout::{Chrome, Frame, PaneId, Rect, Resolved, Workspace};

/// Draws the resolved layout as text.
///
/// Stands in for the renderer until it lands, and doubles as the quickest way
/// to see what a layout change actually did.
pub fn ascii(ws: &Workspace, chrome: &Chrome, area: Rect) -> String {
    let mut grid = vec![vec![' '; area.w as usize]; area.h as usize];
    let frame = chrome.layout(area);

    for (rect, label) in regions(&frame) {
        draw_box(&mut grid, rect, label);
    }

    let tab = ws.active_tab();
    let resolved = tab.resolve(frame.center);
    for (id, rect) in &resolved.panes {
        let label = pane_label(tab.focus(), *id, tab.zoomed());
        draw_box(&mut grid, *rect, &label);
    }
    draw_dividers(&mut grid, &resolved);

    grid.into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn regions(frame: &Frame) -> Vec<(Rect, &'static str)> {
    let mut out = Vec::new();
    if let Some(r) = frame.tab_bar {
        out.push((r, "tabs"));
    }
    if let Some(r) = frame.left {
        out.push((r, "explorer"));
    }
    if let Some(r) = frame.right {
        out.push((r, "agent"));
    }
    if let Some(r) = frame.status {
        out.push((r, "status"));
    }
    out
}

fn pane_label(focus: PaneId, id: PaneId, zoomed: Option<PaneId>) -> String {
    let mut label = format!("pane {}", id.0);
    if id == focus {
        label.push('*');
    }
    if zoomed == Some(id) {
        label.push('Z');
    }
    label
}

fn draw_dividers(grid: &mut [Vec<char>], resolved: &Resolved) {
    for d in &resolved.dividers {
        let ch = match d.axis {
            caret_layout::Axis::Horizontal => '│',
            caret_layout::Axis::Vertical => '─',
        };
        for y in d.rect.y..d.rect.bottom() {
            for x in d.rect.x..d.rect.right() {
                put(grid, x, y, ch);
            }
        }
    }
}

fn draw_box(grid: &mut [Vec<char>], rect: Rect, label: &str) {
    if rect.is_empty() {
        return;
    }
    if rect.h == 1 {
        write_text(grid, rect.x + 1, rect.y, rect.w.saturating_sub(2), label);
        return;
    }

    let (x0, y0) = (rect.x, rect.y);
    let (x1, y1) = (rect.right() - 1, rect.bottom() - 1);

    for x in x0..=x1 {
        put(grid, x, y0, '─');
        put(grid, x, y1, '─');
    }
    for y in y0..=y1 {
        put(grid, x0, y, '│');
        put(grid, x1, y, '│');
    }
    put(grid, x0, y0, '┌');
    put(grid, x1, y0, '┐');
    put(grid, x0, y1, '└');
    put(grid, x1, y1, '┘');

    write_text(grid, x0 + 1, y0 + 1, rect.w.saturating_sub(2), label);
}

fn write_text(grid: &mut [Vec<char>], x: u16, y: u16, max: u16, text: &str) {
    for (i, ch) in text.chars().take(max as usize).enumerate() {
        put(grid, x + i as u16, y, ch);
    }
}

fn put(grid: &mut [Vec<char>], x: u16, y: u16, ch: char) {
    if let Some(row) = grid.get_mut(y as usize) {
        if let Some(cell) = row.get_mut(x as usize) {
            *cell = ch;
        }
    }
}
