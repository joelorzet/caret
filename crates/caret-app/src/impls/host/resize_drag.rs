#[allow(unused_imports)]
use crate::prelude::*;

impl Session {
    /// Starts a drag if the pointer is on a border.
    ///
    /// Borders are one cell wide, which is a small target, so the cell either
    /// side counts as a hit too.
    pub(crate) fn begin_resize_drag(&mut self, event: MouseEvent) -> bool {
        match self.resizable_at(event.col, event.row) {
            Some((drag, pane)) => {
                if let Some(pane) = pane {
                    // Resizing works on the focused pane's border, so focus
                    // the pane this border belongs to before dragging it.
                    self.workspace.active_tab_mut().set_focus(pane);
                    self.focus = FocusTarget::Panes;
                }
                self.drag = Some(drag);
                true
            }
            None => false,
        }
    }

    /// The border under the pointer, if there is one, and the pane it belongs
    /// to.
    ///
    /// Shared by the drag and by the pointer shape: a border that can be
    /// dragged but does not say so is a border nobody finds.
    pub(crate) fn resizable_at(&self, col: u16, row: u16) -> Option<(DividerDrag, Option<PaneId>)> {
        let frame = self.chrome.layout(self.window_rect());

        for (kind, rect) in [
            (DockKind::Explorer, frame.left),
            (DockKind::Agent, frame.right),
        ] {
            let Some(rect) = rect else { continue };
            let edge = match kind {
                DockKind::Explorer => rect.right().saturating_sub(1),
                DockKind::Agent => rect.x,
            };
            if near(col, edge) && row >= rect.y && row < rect.bottom() {
                return Some((
                    DividerDrag {
                        target: DragTarget::Dock(kind),
                        anchor: col,
                    },
                    None,
                ));
            }
        }

        let resolved = self.workspace.active_tab().resolve(frame.center);
        let (divider, pane) = resolved.divider_at(col, row)?;
        Some((
            DividerDrag {
                target: DragTarget::Pane(divider.axis),
                anchor: match divider.axis {
                    Axis::Horizontal => col,
                    Axis::Vertical => row,
                },
            },
            Some(pane),
        ))
    }

    /// Which way a border under the pointer can be dragged, for the shape the
    /// pointer should take. A dock edge only ever moves sideways.
    pub fn resize_axis_at(&self, col: u16, row: u16) -> Option<Axis> {
        let (drag, _) = self.resizable_at(col, row)?;
        Some(match drag.target {
            DragTarget::Dock(_) => Axis::Horizontal,
            DragTarget::Pane(axis) => axis,
        })
    }

    /// Moves the border being dragged to follow the pointer.
    pub(crate) fn continue_resize_drag(&mut self, event: MouseEvent) {
        let Some(drag) = self.drag else { return };
        let position = match drag.target {
            DragTarget::Pane(Axis::Vertical) => event.row,
            _ => event.col,
        };
        let delta = position as i32 - drag.anchor as i32;
        if delta == 0 {
            return;
        }

        match drag.target {
            DragTarget::Dock(kind) => {
                // The agent dock grows leftwards, so the pointer and the width
                // move in opposite directions.
                let signed = match kind {
                    DockKind::Explorer => delta,
                    DockKind::Agent => -delta,
                };
                let dock = self.dock_mut(kind);
                let next = dock.cells as i32 + signed;
                dock.resize(next.clamp(0, u16::MAX as i32) as u16);
            }
            DragTarget::Pane(axis) => {
                let direction = match (axis, delta > 0) {
                    (Axis::Horizontal, true) => Direction::Right,
                    (Axis::Horizontal, false) => Direction::Left,
                    (Axis::Vertical, true) => Direction::Down,
                    (Axis::Vertical, false) => Direction::Up,
                };
                let area = self.center_rect();
                self.workspace.active_tab_mut().resize(
                    area,
                    direction,
                    delta.unsigned_abs() as u16,
                );
            }
        }

        self.drag = Some(DividerDrag {
            anchor: position,
            ..drag
        });
        self.needs_draw = true;
    }

    pub(crate) fn end_resize_drag(&mut self) {
        self.drag = None;
    }

    pub(crate) fn is_resizing(&self) -> bool {
        self.drag.is_some()
    }
}

/// A border is one cell wide; the cells either side count, so it can be
/// grabbed without pixel-hunting.
fn near(position: u16, edge: u16) -> bool {
    position + 1 >= edge && position <= edge + 1
}
