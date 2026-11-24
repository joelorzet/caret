use crate::common::{rect_of, AREA};
use caret_layout::{LayoutNode, LayoutPreset, PaneId, Rect, Workspace};

use crate::support::template;

#[test]
fn an_unweighted_spec_divides_evenly() {
    let node = LayoutNode::parse("h(*, v(*, *))").unwrap();
    assert_eq!(node.pane_count(), 3);

    let mut n = 0;
    let tree = node.build(&mut || {
        let id = PaneId(n);
        n += 1;
        id
    });
    let r = tree.resolve(AREA);
    assert_eq!(rect_of(&r, 0), Rect::new(0, 0, 40, 24));
    assert_eq!(rect_of(&r, 1), Rect::new(41, 0, 39, 12));
}

#[test]
fn explicit_percentages_are_respected() {
    let node = LayoutNode::parse("h(70:*, 30:*)").unwrap();
    let mut n = 0;
    let tree = node.build(&mut || {
        let id = PaneId(n);
        n += 1;
        id
    });
    assert_eq!(rect_of(&tree.resolve(AREA), 0).w, 55);
}

#[test]
fn a_partially_weighted_spec_shares_out_the_remainder() {
    let node = LayoutNode::parse("h(60:*, *, *)").unwrap();
    let mut n = 0;
    let tree = node.build(&mut || {
        let id = PaneId(n);
        n += 1;
        id
    });
    let r = tree.resolve(AREA);
    assert_eq!(rect_of(&r, 0).w, 47, "60% of 78 usable");
    // 20% of 78 is 15.6, so the integer remainder gives one pane the extra
    // cell. Within one cell is as even as it can be while the total stays exact.
    let (b, c) = (rect_of(&r, 1).w as i32, rect_of(&r, 2).w as i32);
    assert!(
        (b - c).abs() <= 1,
        "the other two share what is left: {b} and {c}"
    );
    assert_eq!(rect_of(&r, 0).w + b as u16 + c as u16 + 2, AREA.w);
}

#[test]
fn whitespace_and_case_do_not_matter() {
    assert!(LayoutNode::parse("H( * , V( *, * ) )").is_ok());
}

#[test]
fn malformed_specs_say_what_is_wrong() {
    for (spec, expected) in [
        ("", "unexpected end"),
        ("x(*)", "expected '*'"),
        ("h(*)", "at least two"),
        ("h(*, *", "expected ')'"),
        ("h*", "expected '('"),
        ("h(*, *) junk", "trailing"),
        ("h(200:*, *)", "between 0 and 100"),
        ("h(50*, *)", "followed by ':'"),
    ] {
        let err = LayoutNode::parse(spec).unwrap_err();
        assert!(
            err.reason.contains(expected),
            "{spec:?} gave {:?}, expected {expected:?}",
            err.reason
        );
    }
}

#[test]
fn a_tree_serialises_back_to_a_spec() {
    let mut ws = Workspace::new();
    ws.apply_layout(&template(LayoutPreset::MainVertical, Some(3), 50.0));
    assert_eq!(ws.active_tab().root().to_spec(), "h(*, v(*, *))");

    ws.apply_layout(&template(LayoutPreset::MainVertical, Some(3), 70.0));
    assert_eq!(ws.active_tab().root().to_spec(), "h(70:*, 30:v(*, *))");
}

#[test]
fn specs_round_trip_through_serialisation() {
    for spec in [
        "*",
        "h(*, *)",
        "h(*, v(*, *))",
        "h(70:*, 30:v(*, *))",
        "v(h(*, *), h(*, *, *))",
    ] {
        let node = LayoutNode::parse(spec).unwrap();
        let mut n = 0;
        let tree = node.build(&mut || {
            let id = PaneId(n);
            n += 1;
            id
        });
        let round_tripped = tree.to_spec();
        let reparsed = LayoutNode::parse(&round_tripped).unwrap();
        assert_eq!(
            reparsed.pane_count(),
            node.pane_count(),
            "{spec} -> {round_tripped}"
        );

        let mut m = 0;
        let tree2 = reparsed.build(&mut || {
            let id = PaneId(m);
            m += 1;
            id
        });
        assert_eq!(
            tree2.resolve(AREA),
            tree.resolve(AREA),
            "{spec} -> {round_tripped}"
        );
    }
}
