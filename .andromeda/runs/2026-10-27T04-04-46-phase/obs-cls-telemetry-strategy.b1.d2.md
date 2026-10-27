# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · standing S1 · the #[instrument(skip_all, …)] declaration each seam function carries; the chunk adds no new seam
telemetry-strategy#1.2 · yes · the chunk's new refusal lines (over-size or malformed reads, send-refused, channel and hook refusals) are closed-enum events of the primary log
telemetry-strategy#1.3 · yes · the catch_unwind around the vt100 feed, which the re-reads of gate.rs and screen.rs check, and the one panic record it writes
telemetry-strategy#1.4 · yes · the events.ndjson skipped counts and the event and detail counts that viola.compat.v_rejected derives, which the new -32602 refusals feed
telemetry-strategy#1.5 · yes · the events.ndjson reader the scope names copies no event payload into a process log, and the stream is never rotated
telemetry-strategy#2.1 · yes · the forced vt100 panic witness, run inside the bounded exemption (tests/chaos_feed_panic.rs), still writes one single-line panic record
telemetry-strategy#3.1 · yes · channel frames on both sides: no params or result body in channel-request or channel-response, -32602 / -32600 at WARN, -32603 exactly internal error with data null, from logged unauthenticated
telemetry-strategy#3.2 · yes · the paste rule at the CLI send, the MCP send and the wrapper: send-refused codes only for control-character and empty-text, text_bytes the only paste value a line holds
telemetry-strategy#3.3 · yes · hook stdin through take(MAX_FRAME + 1): oversize or malformed stdin fails open to hook-<name>.ndjson with a fail-open hook-decision detail, and a missing VIOLA_NAME writes no line
telemetry-strategy#3.4 · yes · mcp stdin and tool arguments: a rejected call logs codes only, and instance-unreachable writes no line
telemetry-strategy#3.5 · yes · the R8-stripped CLAUDE* floor over the claude --version and claude agents --json spawns the chunk bounds (names and counts only), and fixed messages for the new refusals
telemetry-strategy#3.6 · yes · the home and ~/.claude file reads the chunk bounds: their state-recovered and skipped lines, and the budget.json failures that write no line
telemetry-strategy#3.7 · yes · the claude --version and claude agents --json spawns: their process-start subject values and the claude-agents-json parse-rejected line
telemetry-strategy#3.8 · no · the chunk adds no dependency or tracing feature, so the logging stack's feature set and the cargo-deny gates stand unchanged
telemetry-strategy#3.9 · no · the chunk adds no correlation field, and corr / conn and trace-context propagation are Not needed
telemetry-strategy#3.10 · no · the chunk adds no duration sample, counter or budget row
telemetry-strategy#3.11 · yes · state-recovered at WARN from read_snapshot_or_replay, one per call, which the bounded snapshot read keeps
telemetry-strategy#3.12 · no · the chunk adds no log file, append path, file mode, CI job or per-OS artifact
telemetry-strategy#3.13 · yes · zero-unlogged-panics, which the scope names: a forced panic still fires the one event:"panic" line, so the forced vt100 panic test runs in the bounded exemption
telemetry-strategy#3.14 · no · retention and rotation are Not needed, since no rotation is added to events.ndjson or diagnostics/
telemetry-strategy#3.15 · yes · the chunk's new or extended reader lines must follow the §3 schema verbatim and use §6 catalogued values
telemetry-strategy#3.16 · yes · a strict-modes refusal before a process log opens writes no line and no detail code, which the stamps.json and budget.json reads keep; the Direction 4 completion is owned by "Exit-cause code catalogue"
telemetry-strategy#3.17 · yes · the installed-CLI reads (claude --version, claude agents --json) and the events.ndjson skipped counts, as parse-rejected and skipped lines under the §5 counters
telemetry-strategy#3.18 · yes · the vt100 gate outcome, or a count on the boundary event, is the only feed signal logged, with no per-chunk line
