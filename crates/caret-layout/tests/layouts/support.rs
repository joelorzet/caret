use caret_layout::{LayoutPreset, LayoutShape, LayoutTemplate};

pub fn template(preset: LayoutPreset, panes: Option<usize>, main: f32) -> LayoutTemplate {
    LayoutTemplate::new(
        preset.name(),
        "",
        LayoutShape::Preset {
            preset,
            panes,
            main,
        },
    )
}
