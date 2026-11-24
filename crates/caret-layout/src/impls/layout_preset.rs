#[allow(unused_imports)]
use crate::prelude::*;

/// The main pane's share, as a percentage of the tab.
const WHOLE: f32 = 100.0;
const MIN_MAIN: f32 = 10.0;
const MAX_MAIN: f32 = 90.0;

impl LayoutPreset {
    pub const EVEN_HORIZONTAL: &'static str = "even-horizontal";
    pub const EVEN_VERTICAL: &'static str = "even-vertical";
    pub const MAIN_VERTICAL: &'static str = "main-vertical";
    pub const MAIN_HORIZONTAL: &'static str = "main-horizontal";
    pub const TILED: &'static str = "tiled";

    /// Canonical name first, then accepted aliases.
    const NAMES: [(LayoutPreset, &'static [&'static str]); 5] = [
        (
            LayoutPreset::EvenHorizontal,
            &[Self::EVEN_HORIZONTAL, "columns", "even-h"],
        ),
        (
            LayoutPreset::EvenVertical,
            &[Self::EVEN_VERTICAL, "rows", "even-v"],
        ),
        (LayoutPreset::MainVertical, &[Self::MAIN_VERTICAL, "main-v"]),
        (
            LayoutPreset::MainHorizontal,
            &[Self::MAIN_HORIZONTAL, "main-h"],
        ),
        (LayoutPreset::Tiled, &[Self::TILED, "grid"]),
    ];

    pub const ALL: [LayoutPreset; 5] = [
        LayoutPreset::EvenHorizontal,
        LayoutPreset::EvenVertical,
        LayoutPreset::MainVertical,
        LayoutPreset::MainHorizontal,
        LayoutPreset::Tiled,
    ];

    pub fn parse(name: &str) -> Option<LayoutPreset> {
        let lower = name.trim().to_ascii_lowercase();
        Self::NAMES
            .iter()
            .find(|(_, names)| names.contains(&lower.as_str()))
            .map(|(value, _)| *value)
    }

    pub const fn name(self) -> &'static str {
        match self {
            LayoutPreset::EvenHorizontal => Self::EVEN_HORIZONTAL,
            LayoutPreset::EvenVertical => Self::EVEN_VERTICAL,
            LayoutPreset::MainVertical => Self::MAIN_VERTICAL,
            LayoutPreset::MainHorizontal => Self::MAIN_HORIZONTAL,
            LayoutPreset::Tiled => Self::TILED,
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            LayoutPreset::EvenHorizontal => "Equal columns",
            LayoutPreset::EvenVertical => "Equal rows",
            LayoutPreset::MainVertical => "Main pane left, the rest stacked right",
            LayoutPreset::MainHorizontal => "Main pane on top, the rest side by side below",
            LayoutPreset::Tiled => "A grid",
        }
    }

    pub fn accepted() -> Vec<&'static str> {
        Self::NAMES.iter().map(|(_, names)| names[0]).collect()
    }

    /// The shape for `panes` panes, giving the main pane `main` percent.
    pub fn shape(self, panes: usize, main: f32) -> LayoutNode {
        let panes = panes.max(1);
        if panes == 1 {
            return LayoutNode::pane();
        }
        let main = main.clamp(MIN_MAIN, MAX_MAIN);
        let rest = WHOLE - main;

        match self {
            LayoutPreset::EvenHorizontal => LayoutNode::split(Axis::Horizontal, even(panes)),
            LayoutPreset::EvenVertical => LayoutNode::split(Axis::Vertical, even(panes)),
            LayoutPreset::MainVertical => LayoutNode::split(
                Axis::Horizontal,
                vec![
                    LayoutNode::pane().weighted(main),
                    stack(Axis::Vertical, panes - 1).weighted(rest),
                ],
            ),
            LayoutPreset::MainHorizontal => LayoutNode::split(
                Axis::Vertical,
                vec![
                    LayoutNode::pane().weighted(main),
                    stack(Axis::Horizontal, panes - 1).weighted(rest),
                ],
            ),
            LayoutPreset::Tiled => tiled(panes),
        }
    }
}

fn even(count: usize) -> Vec<LayoutNode> {
    (0..count).map(|_| LayoutNode::pane()).collect()
}

/// A split of `count` panes, collapsing to a bare pane when count is 1 so the
/// tree never carries a split with a single child.
fn stack(axis: Axis, count: usize) -> LayoutNode {
    if count <= 1 {
        LayoutNode::pane()
    } else {
        LayoutNode::split(axis, even(count))
    }
}

fn tiled(panes: usize) -> LayoutNode {
    let rows = (panes as f64).sqrt().ceil() as usize;
    let base = panes / rows;
    let extra = panes % rows;

    let children = (0..rows)
        .map(|row| {
            let in_row = base + usize::from(row < extra);
            stack(Axis::Horizontal, in_row)
        })
        .collect();
    LayoutNode::split(Axis::Vertical, children)
}
