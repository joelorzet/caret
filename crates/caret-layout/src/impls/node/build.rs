#[allow(unused_imports)]
use crate::prelude::*;

impl Node {
    pub fn leaf(id: PaneId) -> Node {
        Node::Leaf(id)
    }

    pub fn split(axis: Axis, children: Vec<Node>) -> Node {
        Node::Split(Split::new(axis, children))
    }
}
