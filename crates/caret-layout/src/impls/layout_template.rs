#[allow(unused_imports)]
use crate::prelude::*;

impl LayoutTemplate {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        shape: LayoutShape,
    ) -> LayoutTemplate {
        LayoutTemplate {
            name: name.into(),
            description: description.into(),
            shape,
        }
    }

    /// One template per preset, so layouts work with no configuration.
    pub fn presets() -> Vec<LayoutTemplate> {
        LayoutPreset::ALL
            .into_iter()
            .map(|preset| {
                LayoutTemplate::new(
                    preset.name(),
                    preset.description(),
                    LayoutShape::Preset {
                        preset,
                        panes: None,
                        main: 50.0,
                    },
                )
            })
            .collect()
    }

    /// How many panes this template wants, given what the tab already has.
    pub fn pane_count(&self, current: usize) -> usize {
        match &self.shape {
            LayoutShape::Preset { panes, .. } => panes.unwrap_or(current).max(1),
            LayoutShape::Explicit(node) => node.pane_count(),
        }
    }

    pub fn shape_for(&self, current: usize) -> LayoutNode {
        match &self.shape {
            LayoutShape::Preset {
                preset,
                panes,
                main,
            } => preset.shape(panes.unwrap_or(current).max(1), *main),
            LayoutShape::Explicit(node) => node.clone(),
        }
    }
}
