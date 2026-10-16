//! The `claude agents --json` reader over arbitrary bytes: it never panics, and every row it
//! returns carries its `sessionId` unchanged in the row it keeps.

#![no_main]

use libfuzzer_sys::fuzz_target;
use viola_agent_claude::agents;

fuzz_target!(|bytes: &[u8]| {
    if let Ok(list) = agents::parse(bytes) {
        for row in list.rows {
            assert_eq!(
                row.raw.get("sessionId").and_then(|v| v.as_str()),
                Some(row.session_id.as_str())
            );
        }
    }
});
