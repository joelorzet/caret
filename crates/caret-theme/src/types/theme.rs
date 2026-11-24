#[allow(unused_imports)]
use crate::prelude::*;

use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Theme {
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub appearance: Appearance,
    pub palette: Palette,
    pub bg: Rgb,
    pub fg: Rgb,
    pub cursor: Rgb,
    pub cursor_text: Rgb,
    pub selection_bg: Rgb,
    pub selection_fg: Rgb,
    pub ui: UiColors,
}
