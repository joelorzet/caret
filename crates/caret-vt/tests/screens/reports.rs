use crate::common::Vt;

#[test]
fn the_cursor_position_is_reported() {
    let mut vt = Vt::new(10, 5);
    vt.feed("\x1b[3;4H\x1b[6n");
    assert_eq!(vt.reply(), "\x1b[3;4R");
}

#[test]
fn the_cursor_report_respects_origin_mode() {
    let mut vt = Vt::new(10, 8);
    vt.feed("\x1b[3;6r\x1b[?6h\x1b[1;1H\x1b[6n");
    assert_eq!(vt.reply(), "\x1b[1;1R", "reported relative to the region");
}

#[test]
fn the_device_reports_itself_ready() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b[5n");
    assert_eq!(vt.reply(), "\x1b[0n");
}

#[test]
fn device_attributes_are_answered() {
    let mut vt = Vt::new(6, 1);
    vt.feed("\x1b[c");
    assert_eq!(vt.reply(), "\x1b[?62;22c");

    vt.feed("\x1b[>c");
    assert_eq!(vt.reply(), "\x1b[>0;10;1c");
}

#[test]
fn a_device_control_string_is_swallowed() {
    let mut vt = Vt::new(8, 1);
    vt.feed("\x1bP1$r0m\x1b\\ok");
    assert_eq!(vt.row(0), "ok", "the DCS body did not reach the screen");
}
