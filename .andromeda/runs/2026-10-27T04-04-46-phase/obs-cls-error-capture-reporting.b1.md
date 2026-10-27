# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · yes · the one event:panic line in the instance's detail file, which the re-verified vt100 panic witness asserts and G2 counts
error-capture-reporting#1.2 · no · the diagnostics open gate in src/obs.rs is reached by no chunk unit, and the new lines inherit it unchanged
error-capture-reporting#2.1 · yes · the vt100 feed thread's catch_unwind around each feed and resize (the re-read site in src/run/gate.rs) and the panic = unwind it relies on
error-capture-reporting#3.1 · yes · the pre-init window, where names are validated before path joins and each instance refusal writes no line, which the refused-name witness reads
error-capture-reporting#4.1 · yes · the vt100 feed catch_unwind degrade re-verified at src/run/gate.rs, witnessed by tests/chaos_feed_panic.rs (one parse-rejected and one event:panic line, then send-refused input-not-ready with passthrough)
error-capture-reporting#5.1 · yes · the mcp frame tools' check_instance before live_endpoint, where the chunk's rejected-call rule and the no-line instance-unreachable rule land
