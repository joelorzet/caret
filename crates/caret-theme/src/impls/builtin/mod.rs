pub mod ayu_dark;
pub mod ayu_dark_contrast;
pub mod ayu_light;
pub mod ayu_mirage;
pub mod carbonfox;
pub mod carbonfox_contrast;
pub mod catppuccin_frappe;
pub mod catppuccin_latte;
pub mod catppuccin_latte_contrast;
pub mod catppuccin_macchiato;
pub mod catppuccin_mocha;
pub mod catppuccin_mocha_contrast;
pub mod cobalt2;
pub mod dracula;
pub mod dracula_contrast;
pub mod duskfox;
pub mod ember;
pub mod everforest_dark;
pub mod everforest_dark_contrast;
pub mod everforest_light;
pub mod github_dark;
pub mod github_dark_contrast;
pub mod github_light;
pub mod github_light_contrast;
pub mod gruvbox_dark;
pub mod gruvbox_dark_contrast;
pub mod gruvbox_light;
pub mod gruvbox_material;
pub mod horizon_dark;
pub mod iceberg;
pub mod kanagawa_dragon;
pub mod kanagawa_wave;
pub mod kanagawa_wave_contrast;
pub mod monokai_pro;
pub mod monokai_pro_contrast;
pub mod nord;
pub mod nord_contrast;
pub mod one_dark_pro;
pub mod one_dark_pro_contrast;
pub mod one_dark_pro_darker;
pub mod one_dark_pro_night_flat;
pub mod phosphor_amber;
pub mod phosphor_amber_soft;
pub mod phosphor_blue;
pub mod phosphor_cyan;
pub mod phosphor_green;
pub mod phosphor_green_soft;
pub mod phosphor_magenta;
pub mod phosphor_orange;
pub mod phosphor_white;
pub mod phosphor_yellow;
pub mod solarized_dark;
pub mod solarized_light;
pub mod solarized_light_contrast;
pub mod tokyo_night;
pub mod tokyo_night_contrast;

#[allow(unused_imports)]
use crate::prelude::*;

/// Every built-in theme, in presentation order.
pub const SPECS: &[&ThemeSpec] = &[
    &dracula::SPEC,
    &one_dark_pro::SPEC,
    &one_dark_pro_darker::SPEC,
    &one_dark_pro_night_flat::SPEC,
    &nord::SPEC,
    &gruvbox_dark::SPEC,
    &tokyo_night::SPEC,
    &catppuccin_mocha::SPEC,
    &solarized_dark::SPEC,
    &solarized_light::SPEC,
    &phosphor_green::SPEC,
    &phosphor_green_soft::SPEC,
    &phosphor_amber::SPEC,
    &phosphor_amber_soft::SPEC,
    &phosphor_yellow::SPEC,
    &phosphor_orange::SPEC,
    &phosphor_cyan::SPEC,
    &phosphor_blue::SPEC,
    &phosphor_magenta::SPEC,
    &phosphor_white::SPEC,
    &dracula_contrast::SPEC,
    &tokyo_night_contrast::SPEC,
    &gruvbox_dark_contrast::SPEC,
    &nord_contrast::SPEC,
    &one_dark_pro_contrast::SPEC,
    &catppuccin_mocha_contrast::SPEC,
    &kanagawa_wave_contrast::SPEC,
    &everforest_dark_contrast::SPEC,
    &ayu_dark_contrast::SPEC,
    &monokai_pro_contrast::SPEC,
    &carbonfox_contrast::SPEC,
    &github_dark_contrast::SPEC,
    &github_light_contrast::SPEC,
    &solarized_light_contrast::SPEC,
    &catppuccin_latte_contrast::SPEC,
    &ember::SPEC,
    &catppuccin_latte::SPEC,
    &catppuccin_frappe::SPEC,
    &catppuccin_macchiato::SPEC,
    &gruvbox_light::SPEC,
    &gruvbox_material::SPEC,
    &everforest_dark::SPEC,
    &everforest_light::SPEC,
    &kanagawa_wave::SPEC,
    &kanagawa_dragon::SPEC,
    &monokai_pro::SPEC,
    &ayu_dark::SPEC,
    &ayu_mirage::SPEC,
    &ayu_light::SPEC,
    &github_dark::SPEC,
    &github_light::SPEC,
    &horizon_dark::SPEC,
    &iceberg::SPEC,
    &duskfox::SPEC,
    &carbonfox::SPEC,
    &cobalt2::SPEC,
];

/// The theme used when nothing is configured.
pub const DEFAULT: &str = "tokyo-night";

pub fn all() -> Vec<Theme> {
    SPECS.iter().map(|s| Theme::from_spec(s)).collect()
}

pub fn names() -> Vec<&'static str> {
    SPECS.iter().map(|s| s.name).collect()
}

/// Looks a theme up forgivingly: case and `-`/`_`/space differences are
/// ignored, so `"Tokyo Night"` and `tokyo_night` both work.
pub fn by_name(name: &str) -> Option<Theme> {
    let wanted = normalise(name);
    SPECS
        .iter()
        .find(|s| normalise(s.name) == wanted)
        .map(|s| Theme::from_spec(s))
}

pub fn default_theme() -> Theme {
    by_name(DEFAULT).expect("the default theme must be built in")
}

fn normalise(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '-' | '_' | ' '))
        .flat_map(|c| c.to_lowercase())
        .collect()
}
