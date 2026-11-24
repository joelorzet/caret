use caret_app::enums::palette_kind::PaletteKind;
use caret_app::enums::palette_row::PaletteRow;
use caret_app::group_rows;
use caret_app::types::palette_item::PaletteItem;
use caret_config::Action;

fn item(section: &str, label: &str) -> PaletteItem {
    PaletteItem {
        section: section.to_string(),
        label: label.to_string(),
        hint: String::new(),
        kind: PaletteKind::Run(Action::Quit),
    }
}

fn items() -> Vec<PaletteItem> {
    vec![
        item("Panes", "split"),
        item("Panes", "close"),
        item("Tabs", "new tab"),
        item("Panes", "zoom"),
        item("General", "quit"),
    ]
}

/// Everything matching, all at the same score.
fn all(items: &[PaletteItem]) -> Vec<(i32, usize)> {
    (0..items.len()).map(|i| (0, i)).collect()
}

fn headings(rows: &[PaletteRow]) -> Vec<(String, usize)> {
    rows.iter()
        .filter_map(|row| match row {
            PaletteRow::Heading(topic, count) => Some((topic.clone(), *count)),
            PaletteRow::Entry(_) => None,
        })
        .collect()
}

// The topic is said once, above the entries under it, instead of against
// every one of them.
#[test]
fn each_topic_gets_one_heading_in_the_order_it_first_appears() {
    let items = items();
    let rows = group_rows(&items, &all(&items));
    assert_eq!(
        headings(&rows),
        [
            ("Panes".to_string(), 3),
            ("Tabs".to_string(), 1),
            ("General".to_string(), 1)
        ]
    );
}

#[test]
fn entries_follow_the_heading_they_belong_to() {
    let items = items();
    let rows = group_rows(&items, &all(&items));
    let mut topic = String::new();
    for row in &rows {
        match row {
            PaletteRow::Heading(name, _) => topic = name.clone(),
            PaletteRow::Entry(index) => assert_eq!(items[*index].section, topic),
        }
    }
}

#[test]
fn a_topic_nothing_matched_is_left_out_altogether() {
    let items = items();
    // Only the two Panes entries survive the filter.
    let rows = group_rows(&items, &[(0, 0), (0, 1)]);
    assert_eq!(headings(&rows), [("Panes".to_string(), 2)]);
    assert_eq!(rows.len(), 3, "one heading and two entries");
}

#[test]
fn a_heading_counts_what_is_under_it() {
    let items = items();
    let rows = group_rows(&items, &all(&items));
    let mut counted = 0;
    for row in &rows {
        match row {
            PaletteRow::Heading(_, count) => {
                assert_eq!(counted_after(&rows, row), *count);
                counted += count;
            }
            PaletteRow::Entry(_) => {}
        }
    }
    assert_eq!(counted, items.len());
}

/// Entries between this heading and the next.
fn counted_after(rows: &[PaletteRow], heading: &PaletteRow) -> usize {
    let at = rows
        .iter()
        .position(|row| std::ptr::eq(row, heading))
        .expect("heading is in the list");
    rows[at + 1..]
        .iter()
        .take_while(|row| row.entry().is_some())
        .count()
}

#[test]
fn within_a_topic_the_best_match_comes_first() {
    let items = items();
    // "close" scores higher than "split", and both are Panes.
    let rows = group_rows(&items, &[(5, 0), (9, 1), (0, 2)]);
    let first = rows
        .iter()
        .find_map(PaletteRow::entry)
        .expect("an entry under the first heading");
    assert_eq!(items[first].label, "close");
}

#[test]
fn every_entry_appears_exactly_once() {
    let items = items();
    let rows = group_rows(&items, &all(&items));
    let mut seen: Vec<usize> = rows.iter().filter_map(PaletteRow::entry).collect();
    let total = seen.len();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), total);
    assert_eq!(total, items.len());
}

#[test]
fn nothing_matching_lays_out_nothing() {
    let items = items();
    assert!(group_rows(&items, &[]).is_empty());
}
