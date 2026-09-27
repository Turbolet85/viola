//! Channel frames over arbitrary bytes: `read_frame` never panics, never hands back a line past
//! `MAX_FRAME` or one without its `\n`, and every frame parses to a request or a fault.

#![no_main]

use std::io::Cursor;

use libfuzzer_sys::fuzz_target;
use viola_channel::{parse_request, read_frame};

const MAX_FRAME: usize = 16 * 1024 * 1024;

fuzz_target!(|bytes: &[u8]| {
    let mut reader = Cursor::new(bytes);
    while let Ok(line) = read_frame(&mut reader) {
        assert!(line.len() <= MAX_FRAME);
        assert_eq!(line.last(), Some(&b'\n'));
        let _ = parse_request(&line);
    }
});
