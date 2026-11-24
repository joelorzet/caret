#[allow(unused_imports)]
use crate::prelude::*;

/// The 64 printable characters base64 maps its six-bit groups onto, in the
/// order RFC 4648 gives them. This is a wire format, not a choice: a
/// different order would decode to different bytes at the other end.
const DIGITS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
/// Three bytes in, four characters out.
const GROUP: usize = 3;
const ENCODED: usize = 4;
/// Six bits per character, and the mask that takes them.
const BITS: u32 = 6;
const MASK: u32 = (1 << BITS) - 1;
/// What stands in for a group that was not full.
const PAD: u8 = b'=';

/// Nothing here is about written language. The clipboard sequence is a text
/// protocol, so arbitrary bytes travel as printable characters, and base64
/// is the encoding it specifies.
pub fn base64(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(GROUP) * ENCODED);
    for chunk in data.chunks(GROUP) {
        let bytes = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let packed =
            (bytes[0] as u32) << (BITS * 2 + 4) | (bytes[1] as u32) << (BITS + 2) | bytes[2] as u32;

        // The high six bits first, then down. The last two characters are
        // padding when the group they would describe was not there.
        let digit = |shift: u32| DIGITS[(packed >> shift & MASK) as usize] as char;
        out.push(digit(BITS * 3));
        out.push(digit(BITS * 2));
        out.push(if chunk.len() > 1 {
            digit(BITS)
        } else {
            PAD as char
        });
        out.push(if chunk.len() > 2 {
            digit(0)
        } else {
            PAD as char
        });
    }
    out
}

/// The bytes behind base64 text, or nothing if it is not base64.
///
/// Strict about what it accepts: this decodes whatever a program running in
/// a pane sends, and a lenient decoder would turn a malformed request into
/// bytes nobody wrote. Whitespace is allowed, because terminals wrap long
/// sequences, and anything else is a refusal.
pub fn unbase64(text: &str) -> Option<Vec<u8>> {
    let mut packed: u32 = 0;
    let mut have = 0;
    let mut padding = 0;
    let mut out = Vec::with_capacity(text.len() / ENCODED * GROUP);

    for byte in text.bytes() {
        if byte.is_ascii_whitespace() {
            continue;
        }
        if byte == PAD {
            padding += 1;
            continue;
        }
        if padding > 0 {
            return None;
        }
        let value = DIGITS.iter().position(|digit| *digit == byte)? as u32;
        packed = packed << BITS | value;
        have += 1;
        if have == ENCODED {
            out.extend_from_slice(&[(packed >> 16) as u8, (packed >> 8) as u8, packed as u8]);
            packed = 0;
            have = 0;
        }
    }

    // A last group is two characters for one byte or three for two, with
    // padding to four or none at all: senders differ on whether they pad,
    // but padding to the wrong length is a malformed request, not a style.
    match (have, padding) {
        (0, 0) => Some(out),
        (2, 2) | (2, 0) => {
            out.push((packed >> 4) as u8);
            Some(out)
        }
        (3, 1) | (3, 0) => {
            out.push((packed >> 10) as u8);
            out.push((packed >> 2) as u8);
            Some(out)
        }
        // One character carries six bits, which is not a byte, and
        // anything else does not add up.
        _ => None,
    }
}
