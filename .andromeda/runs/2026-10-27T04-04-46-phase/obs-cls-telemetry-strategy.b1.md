# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · yes · the seams its readers sit in (channel dispatch, hook parse, mcp tool calls, snapshot read) keep their declared spans, and the PTY read loop and vt100 feed get none, read not being a listed PTY seam
telemetry-strategy#1.2 · yes · each refusal and boundary call the chunk adds or re-verifies (send-refused, parse-rejected, state-recovered, hook-decision fail-open, channel -32602 / -32600, input-not-ready) logs as a closed event
telemetry-strategy#1.3 · yes · the vt100 feed's catch_unwind sites in src/run/gate.rs and crates/viola-agent-claude/src/screen.rs, re-read by the chunk, keep the panic degrade
telemetry-strategy#1.4 · yes · the chunk's new refusal values and the events.ndjson skip counts feed the jq-derived counts, including the viola.compat.v_rejected -32602 over-count, and no in-process instrument is added
telemetry-strategy#1.5 · yes · the events.ndjson reader the chunk bounds keeps its three skip counts, counts an unknown kind or field, and copies no event payload into a process log; the stream is read, never rotated or truncated
telemetry-strategy#2.1 · yes · the forced vt100 panic witness must fire exactly one panic record inside the bounded exemption (tests/chaos_feed_panic.rs, outside target/e2e-home/), and the re-verified catch sites add no unlogged panic
telemetry-strategy#3.1 · yes · channel frames on both sides: channel-request and channel-response never record params or result bodies, -32602 and -32600 log at WARN, -32603 stays exactly internal error with data null, and from is logged as unauthenticated
telemetry-strategy#3.2 · yes · the paste refusal at the CLI send, the MCP send and the wrapper logs send-refused for control-character and empty-text as codes only, the paste reaching a line only as text_bytes
telemetry-strategy#3.3 · yes · hook stdin through take(MAX_FRAME + 1): oversize or malformed stdin fails open (exit 0, no body) and logs only a fail-open hook-decision detail to hook-<name>.ndjson, and a strict-modes refusal writes no line
telemetry-strategy#3.4 · yes · mcp stdin and tool arguments: a rejected call is an isError with codes only, its log line is codes-only with no upstream text, path or type name, and instance-unreachable writes no line
telemetry-strategy#3.5 · yes · the R8-stripped CLAUDE* floor over the claude --version and claude agents --json spawns: names and counts only on process-start, never values, with fixed messages
telemetry-strategy#3.6 · yes · the home and ~/.claude reads the chunk bounds (budget.json, ledger/stamps.json, snapshot.json, config.json, events.ndjson, statusline-source.json, settings.json) keep their strict-modes check before the read, their state-recovered and skipped lines, and no line on failure; names are checked before path joins
telemetry-strategy#3.7 · yes · the claude --version and claude agents --json spawns keep closed subject values on process-start and process-exit, and an agents parse failure logs parse-rejected{parser:"claude-agents-json"} with the value unknown
telemetry-strategy#3.8 · no · the chunk adds and changes no crate, feature or CI gate, so the logging-stack feature set and the cargo-deny gates are not reached
telemetry-strategy#3.9 · no · the chunk adds no correlation field, and the corr / conn join key and trace-context propagation are Not needed
telemetry-strategy#3.10 · no · the chunk measures and changes no perf budget, so the hyperfine gates and the §10 budget rows are not reached
telemetry-strategy#3.11 · yes · the state-recovered{detail:snapshot-replayed or snapshot-unsupported-v} line from read_snapshot_or_replay, one per call at WARN, which the bounded snapshot read must keep emitting
telemetry-strategy#3.12 · no · the chunk creates no log file, adds no append and changes no endpoint_kind or pty_backend field, so the per-OS log perms, concurrent-append check and per-OS artifacts are not reached
telemetry-strategy#3.13 · no · standing S5 · a new test meets the zero-unlogged-panics, zero-secret-hit and zero-schema-failure budgets, with no retry-once policy
telemetry-strategy#3.14 · no · the chunk adds no rotation or retention, and audit-log retention is Not needed
telemetry-strategy#3.15 · no · standing S2 · a new event value, field, parser value or detail code joins the §6 catalog and the diag-line schema's default-deny, checked by G4
telemetry-strategy#3.16 · yes · a strict-modes refusal on the stamps, budget.json and frame-verb reads writes no line and carries no detail code (exit code only, or the closed --json document), a rule the chunk's new refusal paths must keep
telemetry-strategy#3.17 · yes · the parse-rejected lines for config-json (one per bad key) and claude-agents-json, and the events.ndjson skipped counts, are the installed-CLI measured surfaces the chunk adds to
telemetry-strategy#3.18 · yes · the readiness gate logs its outcome only (input-not-ready on a vt100 degrade), never screen text, the no-screen-scraping rule
