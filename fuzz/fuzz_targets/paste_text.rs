//! `validate_paste_text` over arbitrary bytes read as lossy UTF-8: it never panics and agrees with
//! the literal per-char rule.

#![no_main]

use libfuzzer_sys::fuzz_target;
use viola_core::{NotDelivered, validate_paste_text};

/// LF, CR and TAB pass; every other C0, DEL and C1 refuses — written out independently.
fn oracle(text: &str) -> Result<(), NotDelivered> {
    for c in text.chars() {
        let code = u32::from(c);
        let allowed = matches!(code, 0x09 | 0x0a | 0x0d);
        let refused = code < 0x20 || (0x7f..=0x9f).contains(&code);
        if refused && !allowed {
            return Err(NotDelivered::ControlCharacter);
        }
    }
    Ok(())
}

fuzz_target!(|bytes: &[u8]| {
    let text = String::from_utf8_lossy(bytes);
    assert_eq!(validate_paste_text(&text), oracle(&text));
});
