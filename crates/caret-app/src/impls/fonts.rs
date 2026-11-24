use caret_config::Config;
use caret_render::{FontFamily, FontLibrary};

/// How wide the family-name column is before it is cut short.
const NAME_COLUMN: usize = 40;

/// Lists installed fonts, monospaced ones first, marking the configured
/// family and whether each has the bold and italic faces a terminal wants.
pub fn render(query: Option<&str>) -> String {
    let library = FontLibrary::load();
    let config = Config::load().config;

    let families = match query {
        Some(q) => library.search(q),
        None => library.families(),
    };

    if families.is_empty() {
        return match query {
            Some(q) => format!("no font families match {q:?}\n"),
            None => "no fonts found\n".to_string(),
        };
    }

    let width = families
        .iter()
        .map(|f| f.name.chars().count())
        .max()
        .unwrap_or(0)
        .min(NAME_COLUMN);

    let mut out = format!(
        "{} families, {} faces\n",
        library.families().len(),
        library.face_count()
    );

    let configured = library.resolve(&config.font.family, &config.font.fallback);
    match &configured {
        Some(face) if face.family == config.font.family => {
            out.push_str(&format!("in use: {}\n", face.family));
        }
        Some(face) => out.push_str(&format!(
            "in use: {} ({:?} is not installed)\n",
            face.family, config.font.family
        )),
        None => out.push_str(&format!(
            "in use: none of {:?} or its fallbacks are installed\n",
            config.font.family
        )),
    }

    let (mono, other): (Vec<_>, Vec<_>) = families.into_iter().partition(|f| f.monospaced);
    for (heading, group) in [("monospaced", mono), ("other", other)] {
        if group.is_empty() {
            continue;
        }
        out.push_str(&format!("\n{heading}\n"));
        for family in group {
            let marker = if configured.as_ref().is_some_and(|f| f.family == family.name) {
                "*"
            } else {
                " "
            };
            out.push_str(&format!(
                "{marker} {:<width$}  {}\n",
                family.name,
                describe(&family),
                width = width
            ));
        }
    }

    out.push_str("\nSet one with `font.family` in config.toml.\n");
    out
}

fn describe(family: &FontFamily) -> String {
    let faces = if family.faces == 1 {
        "1 face".to_string()
    } else {
        format!("{} faces", family.faces)
    };
    let bold = if family.has_bold { "bold" } else { "----" };
    let italic = if family.has_italic {
        "italic"
    } else {
        "------"
    };
    format!("{faces:>8}  {bold}  {italic}")
}
