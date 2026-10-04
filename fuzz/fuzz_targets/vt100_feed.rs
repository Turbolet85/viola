//! The screen model's feed over arbitrary bytes at a size the input picks: a caught vt100 panic
//! poisons the model and its verdict is then `input-not-ready`. Findings are hangs, OOMs and a
//! panic escaping the catch, never vt100's caught panic, which is the documented degrade.

#![no_main]

use std::panic::{self, AssertUnwindSafe, catch_unwind};
use std::sync::Once;
use std::time::{Duration, Instant};

use libfuzzer_sys::fuzz_target;
use viola_agent_claude::screen::{GateStep, Readiness, Screen, Signatures};

const SIGS: Signatures = Signatures {
    input_box: &["> "],
    modals: &["Do you want to proceed?"],
};

static SILENT_HOOK: Once = Once::new();

fuzz_target!(|bytes: &[u8]| {
    // libfuzzer-sys's own panic hook aborts on every panic, a caught one included; an escape of
    // the catch below still aborts through its catch_unwind around this body.
    SILENT_HOOK.call_once(|| panic::set_hook(Box::new(|_| {})));
    let [r, c, rest @ ..] = bytes else {
        return;
    };
    let rows = 1 + u16::from(*r % 64);
    let cols = 1 + u16::from(*c % 200);
    let base = Instant::now();
    let mut screen = Screen::new(rows, cols, base);
    for chunk in rest.chunks(rest.len().div_ceil(4).max(1)) {
        if catch_unwind(AssertUnwindSafe(|| screen.feed(chunk, base))).is_err() {
            screen.poison();
            assert_eq!(
                screen.verdict(Some(&SIGS), base, base + Duration::from_secs(1)),
                GateStep::Done(Readiness::InputNotReady)
            );
        }
    }
});
