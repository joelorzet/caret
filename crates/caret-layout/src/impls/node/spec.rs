#[allow(unused_imports)]
use crate::prelude::*;

impl Node {
    /// Serialises the tree back into the spec language, so a layout arranged
    /// by hand can be saved into the config.
    ///
    /// Weights that are already an equal share are left out, keeping the
    /// common case readable.
    pub fn to_spec(&self) -> String {
        match self {
            Node::Leaf(_) => "*".to_string(),
            Node::Split(sp) => {
                let even = 1.0 / sp.children().len() as f32;
                let parts: Vec<String> = sp
                    .children()
                    .iter()
                    .zip(sp.weights())
                    .map(|(child, weight)| {
                        let inner = child.to_spec();
                        if (weight - even).abs() < 0.005 {
                            inner
                        } else {
                            format!("{}:{}", (weight * 100.0).round() as i32, inner)
                        }
                    })
                    .collect();
                let axis = match sp.axis() {
                    Axis::Horizontal => 'h',
                    Axis::Vertical => 'v',
                };
                format!("{axis}({})", parts.join(", "))
            }
        }
    }
}
