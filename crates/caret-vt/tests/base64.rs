use caret_vt::{base64, unbase64};

/// The vectors RFC 4648 gives, which are what every decoder is written
/// against. Padding is the part implementations get wrong.
#[test]
fn the_published_vectors_encode_as_published() {
    for (plain, encoded) in [
        ("", ""),
        ("f", "Zg=="),
        ("fo", "Zm8="),
        ("foo", "Zm9v"),
        ("foob", "Zm9vYg=="),
        ("fooba", "Zm9vYmE="),
        ("foobar", "Zm9vYmFy"),
    ] {
        assert_eq!(base64(plain.as_bytes()), encoded, "{plain:?}");
    }
}

// base64 carries bytes, and the text is UTF-8 before it gets here, so a
// selection in any script travels the same way as one in English.
#[test]
fn text_in_any_script_survives_the_round_trip() {
    for text in [
        "hello",
        "¿dónde está el café?",
        "Grüße aus München",
        "здравствуйте",
        "日本語のテキスト",
        "مرحبا بالعالم",
        "नमस्ते दुनिया",
        "🌍 emoji 🎉 too",
        "混ぜた mixed текст",
    ] {
        let there = base64(text.as_bytes());
        let back = decode(&there);
        assert_eq!(
            String::from_utf8(back).expect("utf-8 comes back as utf-8"),
            text
        );
    }
}

#[test]
fn every_byte_value_comes_back_unchanged() {
    let all: Vec<u8> = (0..=255u8).collect();
    assert_eq!(decode(&base64(&all)), all);
}

#[test]
fn the_output_is_always_printable_and_a_multiple_of_four() {
    for length in 0..40usize {
        let data: Vec<u8> = (0..length).map(|i| (i * 37 % 256) as u8).collect();
        let encoded = base64(&data);
        assert_eq!(encoded.len() % 4, 0, "length {length}");
        assert!(
            encoded.bytes().all(|b| b.is_ascii_graphic()),
            "length {length} produced something unprintable"
        );
    }
}

/// A decoder written only for the tests, so the encoder is checked against
/// something other than itself.
fn decode(text: &str) -> Vec<u8> {
    const DIGITS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::new();
    let mut bits = 0u32;
    let mut held = 0u32;
    for byte in text.bytes().filter(|b| *b != b'=') {
        let value = DIGITS
            .iter()
            .position(|d| *d == byte)
            .unwrap_or_else(|| panic!("{:?} is not a base64 digit", byte as char));
        held = held << 6 | value as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((held >> bits & 0xff) as u8);
        }
    }
    out
}

// The same vectors in reverse, since a decoder that cannot read what the
// encoder wrote is no decoder at all.
#[test]
fn the_published_vectors_decode_as_published() {
    for (plain, encoded) in [
        ("", ""),
        ("f", "Zg=="),
        ("fo", "Zm8="),
        ("foo", "Zm9v"),
        ("foob", "Zm9vYg=="),
        ("fooba", "Zm9vYmE="),
        ("foobar", "Zm9vYmFy"),
    ] {
        assert_eq!(
            unbase64(encoded).as_deref(),
            Some(plain.as_bytes()),
            "{encoded}"
        );
    }
}

#[test]
fn anything_the_encoder_wrote_reads_back_byte_for_byte() {
    let samples: [&[u8]; 5] = [
        b"",
        b"a",
        b"hello, world",
        "accents and an emoji: é 🎉".as_bytes(),
        &[0, 1, 2, 250, 251, 252, 253, 254, 255],
    ];
    for bytes in samples {
        assert_eq!(unbase64(&base64(bytes)).as_deref(), Some(bytes));
    }
}

// This decodes whatever a program in a pane sends, so it refuses rather
// than guesses.
#[test]
fn what_is_not_base64_is_refused() {
    for text in [
        "Zm9v*",
        "Zg=",
        "Z",
        "Zm9vYg=x",
        "not base64 at all!",
        "=",
        "../../etc/passwd",
    ] {
        assert!(unbase64(text).is_none(), "{text}");
    }
}

// Long sequences get wrapped on the way through a terminal.
// Plenty of senders leave the padding off, and that is unambiguous.
#[test]
fn the_unpadded_form_is_accepted() {
    assert_eq!(unbase64("Zg").as_deref(), Some(&b"f"[..]));
    assert_eq!(unbase64("Zm8").as_deref(), Some(&b"fo"[..]));
    assert_eq!(unbase64("Zm9vYmFy").as_deref(), Some(&b"foobar"[..]));
}

#[test]
fn whitespace_between_the_characters_is_allowed() {
    assert_eq!(unbase64("Zm9v YmFy").as_deref(), Some(&b"foobar"[..]));
    assert_eq!(unbase64("Zm9v\r\nYmFy\n").as_deref(), Some(&b"foobar"[..]));
}
