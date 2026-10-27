# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · yes · the forced vt100 panic witness asserts the one `event:"panic"` detail line this states (written when an instance resolves, never gated by RUST_BACKTRACE), which the task's zero-unlogged-panics Need reads
error-capture-reporting#1.2 · no · the diagnostics open gate (`check_diagnostics` in `src/obs.rs`) is a component the chunk only uses: no reached file or named unit touches it, and the refused-name witness reads the name validator, not this gate
error-capture-reporting#2.1 · yes · the vt100 feed thread's `catch_unwind` containment, the channel connection worker's `-32603` Drop guard and the PTY pump's catch_unwind body are what the re-verified degrade, the channel frame answers and the PTY output cap must still honour, and `panic = "unwind"` keeps those catch sites live
error-capture-reporting#3.1 · yes · the chunk's name validators and its hook stdin, `mcp` argument and `send` text reads sit in this pre-init window, so they must come after the instance checks and write no line on a refusal
error-capture-reporting#4.1 · yes · the re-verified vt100 degrade must still produce exactly one `parse-rejected{parser:"vt100-feed"}` at WARN and the `event:"panic"` line, and the forced-panic witness (`tests/chaos_feed_panic.rs`) runs outside `target/e2e-home/` under the bounded exemption, as the task's Needs name
error-capture-reporting#5.1 · yes · the chunk's `mcp` tool-argument bounds and `invalid-params` refusals sit in the frame tools that run `check_instance` before `live_endpoint`, so `instance-unreachable` must keep writing no frame and no line
