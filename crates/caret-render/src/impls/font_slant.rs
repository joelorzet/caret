#[allow(unused_imports)]
use crate::prelude::*;

impl FontSlant {
    pub(crate) fn from_db(style: fontdb::Style) -> FontSlant {
        match style {
            fontdb::Style::Normal => FontSlant::Upright,
            fontdb::Style::Italic => FontSlant::Italic,
            fontdb::Style::Oblique => FontSlant::Oblique,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            FontSlant::Upright => "upright",
            FontSlant::Italic => "italic",
            FontSlant::Oblique => "oblique",
        }
    }

    pub const fn is_slanted(self) -> bool {
        !matches!(self, FontSlant::Upright)
    }
}
