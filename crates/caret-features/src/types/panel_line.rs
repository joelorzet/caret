#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PanelLine {
    pub text: String,
    pub kind: LineKind,
    pub selected: bool,
}
