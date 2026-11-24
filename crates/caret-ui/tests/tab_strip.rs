use caret_ui::{StripHit, TabItem, TabStrip};

fn tab(title: &str) -> TabItem {
    TabItem {
        title: title.into(),
        panes: 1,
        zoomed: false,
    }
}

fn strip(titles: &[&str]) -> TabStrip {
    TabStrip {
        tabs: titles.iter().map(|t| tab(t)).collect(),
        active: 0,
        inset_left: 0,
        height: 36,
        cell_width: 8,
        scale: 1.0,
    }
}

#[test]
fn a_tab_is_as_wide_as_what_it_holds() {
    let s = strip(&["zsh", "caret"]);
    let layout = s.layout(1200);
    assert!(
        layout.tabs[0].w < layout.tabs[1].w,
        "the short title got a narrower tab: {:?}",
        layout.tabs
    );
}

#[test]
fn a_long_title_stops_growing_at_the_limit() {
    let s = strip(&["a-really-very-extremely-long-directory-name-here"]);
    assert_eq!(s.layout(1200).tabs[0].w, 220);
}

#[test]
fn a_short_title_still_looks_like_a_tab() {
    let s = strip(&["x"]);
    assert!(s.layout(1200).tabs[0].w >= 52);
}

#[test]
fn tabs_do_not_overlap_or_touch() {
    let s = strip(&["one", "two", "three"]);
    let layout = s.layout(1200);
    for pair in layout.tabs.windows(2) {
        assert!(pair[0].right() < pair[1].x);
    }
}

#[test]
fn many_tabs_give_up_their_width_uniformly() {
    let s = strip(&["caret"; 14]);
    let layout = s.layout(900);
    let first = layout.tabs[0].w;
    assert!(first < 220, "shrunk to fit");
    assert!(layout.tabs.iter().all(|t| t.w == first), "shared evenly");
    assert!(
        layout.new_tab.right() <= 900,
        "the button stays in the window"
    );
}

#[test]
fn tabs_never_shrink_below_a_usable_width() {
    let s = strip(&["caret"; 60]);
    assert!(s.layout(800).tabs.iter().all(|t| t.w >= 52));
}

// Squeezing must not stretch a tab that was already narrower than its share.
#[test]
fn a_narrow_tab_is_not_padded_out_when_the_others_shrink() {
    let mut s = strip(&["x"]);
    s.tabs.extend(std::iter::repeat_n(tab("caret"), 12));
    // The same strip with room to spare, so the only difference is the
    // squeeze rather than how many tabs there are.
    let roomy = s.layout(2400).tabs[0].w;
    assert_eq!(s.layout(900).tabs[0].w, roomy);
}

#[test]
fn a_pane_count_earns_room_for_the_badge() {
    let plain = strip(&["zsh"]).layout(1200).tabs[0].w;
    let mut s = strip(&["zsh"]);
    s.tabs[0].panes = 4;
    assert!(s.layout(1200).tabs[0].w > plain);
}

#[test]
fn the_window_controls_are_left_alone() {
    let mut s = strip(&["zsh", "zsh"]);
    s.inset_left = 78;
    assert_eq!(s.layout(1200).tabs[0].x, 78);
}

#[test]
fn the_new_tab_button_follows_the_last_tab() {
    let s = strip(&["one", "two", "three"]);
    let layout = s.layout(1200);
    let last = layout.tabs.last().unwrap();
    assert!(layout.new_tab.x >= last.right());
    assert!(layout.new_tab.x - last.right() < 10, "it sits right after");
}

#[test]
fn clicks_land_on_the_right_thing() {
    let s = strip(&["one", "two", "three"]);
    let layout = s.layout(1200);
    let middle = layout.tabs[1];
    assert_eq!(s.hit(&layout, middle.x + 4, 10), StripHit::Tab(1));
    assert_eq!(s.hit(&layout, layout.new_tab.x + 5, 10), StripHit::NewTab);
    assert_eq!(s.hit(&layout, 20_000, 10), StripHit::Background);
}

// The close button overlaps its own tab, so order of testing matters.
#[test]
fn the_close_button_wins_over_the_tab_under_it() {
    let s = strip(&["caret", "caret"]);
    let layout = s.layout(1200);
    let close = layout.closes[1];
    assert!(layout.tabs[1].contains(close.x, close.y + 2));
    assert_eq!(s.hit(&layout, close.x + 2, close.y + 2), StripHit::Close(1));
}

#[test]
fn narrow_tabs_drop_the_close_button_rather_than_cover_the_title() {
    let s = strip(&["caret"; 30]);
    let layout = s.layout(900);
    assert!(layout.tabs[0].w < 104);
    assert!(layout.closes.is_empty());
}

// Room for the button is held even when it is not drawn, so a title does not
// jump sideways as the pointer crosses the tab.
#[test]
fn the_title_does_not_reflow_when_the_close_button_appears() {
    let s = strip(&["caret"]);
    let wide = s.layout(1200).tabs[0].w;
    let text = (s.tabs[0].columns(0, s.numbered()) * s.cell_width) as i32;
    assert!(wide as i32 - text >= 16, "the button's room is reserved");
}

// Against the same strip at one times rather than against numbers written
// down here: a measurement copied into a test only ever proves it was copied.
#[test]
fn a_retina_display_doubles_every_metric() {
    let plain = strip(&["caret"]).layout(1200);

    let mut retina = strip(&["caret"]);
    retina.scale = 2.0;
    retina.cell_width *= 2;
    let doubled = retina.layout(2400);

    assert_eq!(doubled.new_tab.w, plain.new_tab.w * 2);
    assert_eq!(doubled.tabs[0].w, plain.tabs[0].w * 2);
    assert_eq!(doubled.tabs[0].x, plain.tabs[0].x * 2);
}

#[test]
fn an_empty_strip_still_offers_the_new_tab_button() {
    let s = strip(&[]);
    let layout = s.layout(1200);
    assert!(layout.tabs.is_empty());
    assert!(layout.new_tab.w > 0);
}

// A tab called `~` has ample room for a close button; it was refused one
// because the rule asked for four characters of title to survive and it only
// has one to keep.
#[test]
fn a_one_character_title_still_gets_its_close_button() {
    let layout = strip(&["~", "~"]).layout(1200);
    assert!(layout.close_for(0).is_some(), "{:?}", layout.tabs[0]);
    assert!(layout.close_for(1).is_some());
}

#[test]
fn the_close_button_sits_inside_its_own_tab() {
    let layout = strip(&["~", "caret"]).layout(1200);
    for index in 0..2 {
        let tab = layout.tabs[index];
        let close = layout.close_for(index).expect("a button");
        assert!(close.x >= tab.x, "tab {index}");
        assert!(close.right() <= tab.right(), "tab {index}");
    }
}

// Squeezed far enough that the title itself is being cut, the button goes
// rather than sitting on top of the name.
#[test]
fn a_tab_squeezed_past_its_title_loses_the_button() {
    let many: Vec<&str> = std::iter::repeat_n("a-long-enough-title", 20).collect();
    let layout = strip(&many).layout(600);
    assert!(layout.close_for(0).is_none(), "{:?}", layout.tabs[0]);
}

// One tab has nothing to be picked out from, so the number in front of it
// labels a choice that does not exist.
#[test]
fn a_lone_tab_is_not_numbered() {
    let alone = strip(&["caret"]);
    assert!(!alone.numbered());

    let mut pair = strip(&["caret"]);
    pair.tabs.push(tab("caret"));
    assert!(pair.numbered(), "a second tab brings the numbers back");

    assert!(
        pair.layout(1200).tabs[0].w > alone.layout(1200).tabs[0].w,
        "and the number earns its room"
    );
}
