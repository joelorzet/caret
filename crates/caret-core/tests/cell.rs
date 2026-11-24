use caret_core::{Cell, Color, Flags};

#[test]
fn stays_sixteen_bytes() {
    assert_eq!(std::mem::size_of::<Cell>(), 16);
}

#[test]
fn flag_set_operations() {
    let mut f = Flags::NONE;
    assert!(f.is_empty());

    f.insert(Flags::BOLD | Flags::ITALIC);
    assert!(f.contains(Flags::BOLD | Flags::ITALIC));
    assert!(!f.contains(Flags::DIM));

    f.remove(Flags::BOLD);
    assert!(!f.contains(Flags::BOLD));
    assert!(f.contains(Flags::ITALIC));

    f.set(Flags::DIM, true);
    assert!(f.contains(Flags::DIM));
    f.set(Flags::DIM, false);
    assert!(!f.contains(Flags::DIM));
}

#[test]
fn sgr_mask_excludes_positional_markers() {
    assert!(!Flags::SGR_MASK.contains(Flags::WIDE));
    assert!(!Flags::SGR_MASK.contains(Flags::WIDE_TAIL));
    assert!(Flags::SGR_MASK.contains(Flags::BOLD));
    assert!(Flags::SGR_MASK.contains(Flags::UNDERLINE_CURLY));
}

#[test]
fn any_underline_covers_every_style() {
    assert!(Flags::UNDERLINE.any_underline());
    assert!(Flags::UNDERLINE_DOUBLE.any_underline());
    assert!(Flags::UNDERLINE_CURLY.any_underline());
    assert!(!Flags::BOLD.any_underline());
}

#[test]
fn blank_carries_the_background() {
    let bg = Color::rgb(10, 20, 30);
    let c = Cell::blank(bg);
    assert_eq!(c.ch, ' ');
    assert_eq!(c.bg, bg);
    assert!(c.is_blank());
}

#[test]
fn wide_cell_reports_two_columns() {
    let mut c = Cell::EMPTY;
    assert_eq!(c.width(), 1);
    c.flags.insert(Flags::WIDE);
    assert_eq!(c.width(), 2);
}
