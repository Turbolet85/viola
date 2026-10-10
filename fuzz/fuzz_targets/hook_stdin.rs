//! The hook stdin readers over arbitrary bytes: the event reader for every registered event, and
//! the statusline payload reader. Neither panics, and a payload the event reader takes always
//! yields an object `data`.

#![no_main]

use libfuzzer_sys::fuzz_target;
use viola_agent_claude::hook::{HookEvent, normalise};
use viola_agent_claude::statusline;

fuzz_target!(|bytes: &[u8]| {
    for event in HookEvent::ALL {
        if let Ok(read) = normalise(event, bytes) {
            assert!(read.data.is_object());
        }
    }
    let _ = statusline::reading(bytes);
});
