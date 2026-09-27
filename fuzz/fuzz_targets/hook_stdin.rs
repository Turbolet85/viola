//! The hook stdin reader over arbitrary bytes, for every registered event: it never panics, and a
//! payload it takes always yields an object `data`.

#![no_main]

use libfuzzer_sys::fuzz_target;
use viola_agent_claude::hook::{HookEvent, normalise};

fuzz_target!(|bytes: &[u8]| {
    for event in HookEvent::ALL {
        if let Ok(read) = normalise(event, bytes) {
            assert!(read.data.is_object());
        }
    }
});
