# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the panic hook's raw-frame rule lives in src/panic_frames.rs and src/obs.rs, neither reached, and the session touches no panic path
error-capture-reporting#1.2 · no · the diagnostics open gate lives in src/obs.rs, not reached, and a refused open stays a failed open
error-capture-reporting#2.1 · yes · add-to · the cli catch site now prints the internal-error document on stdout under --json (F6), and revive --json prints exit-1 documents for start errors and refusals (F7, F9) but none after a pump_child error (F8)
error-capture-reporting#3.1 · yes · contradict · link and unlink's instance refusal under --json is stated as detail null, but exit 21's document now carries one closed cause in detail
error-capture-reporting#4.1 · no · the vt100 feed thread in src/run/gate.rs is not reached, and parse-rejected and run's json-false path are unchanged
error-capture-reporting#4.2 · no · the passthrough witness in tests/chaos_feed_panic.rs is not reached and run's outer-terminal output is unchanged
error-capture-reporting#5.1 · yes · contradict · an MCP refused check returns detail null, but the isError body now carries the cause as detail (F10 maps it), with isError still the only record
error-capture-reporting#5.2 · no · the stdin oversize path and src/main.rs exit_code are unchanged, since no exit moved
error-capture-reporting#5.3 · no · the stdin oversize test's exit 1, empty streams and single parse-rejected record are untouched, since no exit moved and no event or field was added
error-capture-reporting#6.1 · no · the channel crate and its peer-fault readings are not in the diff, and the client and MCP edits move refusal documents, not the fault reader
error-capture-reporting#6.2 · no · is_code_word, the refusal words and reason_of's exit 14 are unchanged, since no RefusalReason was added and no exit moved
error-capture-reporting#6.3 · yes · stale · UnreachableCause in viola-core and without_endpoint in viola-state are new items in counted crates, so the 9 thiserror::Error derives may have moved and are not shown to hold
error-capture-reporting#6.4 · no · the exit-2 UNUSABLE lines in answer.rs and send.rs are unchanged, since the session gives documents only to exit 21, exit 1 and the catch site
