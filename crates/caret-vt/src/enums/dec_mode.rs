/// A DEC private mode, as `CSI ? Ps h` and `CSI ? Ps l` name them.
///
/// The numbers are the protocol's, so they live here once rather than in the
/// middle of the code that acts on them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DecMode {
    /// DECCKM: arrows send SS3 rather than CSI.
    ApplicationCursor,
    /// DECSCNM: the whole screen is drawn in reverse.
    ReverseVideo,
    /// DECOM: row addressing is relative to the scrolling region.
    Origin,
    /// DECAWM: text wraps at the right margin.
    AutoWrap,
    /// AT&T 610 blinking cursor.
    CursorBlink,
    /// DECTCEM: whether the cursor is drawn at all.
    CursorVisible,
    /// The original alternate screen, without the cursor save.
    AltScreen,
    /// Report button presses and releases.
    MouseClick,
    /// Also report motion while a button is held.
    MouseDrag,
    /// Report motion whether or not a button is held.
    MouseMotion,
    /// Report when the window gains or loses focus.
    FocusEvents,
    /// Report mouse positions in the extended SGR form.
    MouseSgr,
    /// The wheel sends arrow keys on the alternate screen.
    AltScroll,
    /// Save and restore the cursor on its own.
    SaveCursor,
    /// The alternate screen, clearing it on the way in.
    AltScreenClear,
    /// The alternate screen, saving the cursor and clearing on the way in.
    AltScreenSaveClear,
    /// Wrap pasted text in markers so it cannot be read as typing.
    BracketedPaste,
    /// Hold the frame until the application says it is finished.
    SynchronisedUpdate,
}

/// An ANSI mode, as `CSI Ps h` and `CSI Ps l` name them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnsiMode {
    /// IRM: typing pushes the rest of the line along.
    Insert,
    /// LNM: a line feed also returns the carriage.
    LineFeedNewLine,
}
