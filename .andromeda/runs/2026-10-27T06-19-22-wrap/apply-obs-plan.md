# Apply — obs-plan

## Verdicts
- fanout-obs-plan--cascade.md 1 · applied · .andromeda/obs-plan.md:125#1
- fanout-obs-plan--cascade.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:99#1
- fanout-obs-plan--cascade.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:212#1
- fanout-obs-plan.md 1 · folded into fanout-obs-plan--cascade.md 3
- fanout-obs-plan--cascade.md 4 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:500#1
- fanout-obs-plan--cascade.md 5 · applied · .andromeda/obs-plan-detail/log-coverage.md:20#1
- fanout-obs-plan--cascade.md 6 · applied · .andromeda/obs-plan-detail/log-coverage.md:34#1
- fanout-obs-plan--cascade.md 7 · applied · .andromeda/obs-plan-detail/log-coverage.md:34+#1
- fanout-obs-plan--cascade.md 8 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:31+#1
- fanout-obs-plan--cascade.md 9 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:37+#1
- fanout-obs-plan--cascade.md 10 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:32+#1

## Edits
- line: .andromeda/obs-plan.md:125
  old: «stdio MCP server, Tokio allowed)»
  new: «stdio MCP server, Tokio allowed; its stdin read in lines of at most `MAX_FRAME`, a longer line refused whole and the session ended as at a closed stdin)»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:99
  old: «own response. Instrument `hook-invoked` / `hook-decision` and MCP tool-call handling.»
  new: «own response; `viola mcp` reads that payload in lines of at most `MAX_FRAME`, a longer line refused whole. Instrument `hook-invoked` / `hook-decision`, MCP tool-call handling and the `viola mcp` stdin cut (one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`, no byte of the line).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:212
  old: «so the writer must be set explicitly.»
  new: «so the writer must be set explicitly. rmcp reads stdin only through viola's line cap (`CappedLines` at `MAX_FRAME`): a line of at most `MAX_FRAME` bytes, its newline counted, passes whole; a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, stderr empty, its one record the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:500
  old: «channel framing at `MAX_FRAME`, hook stdin,»
  new: «channel framing at `MAX_FRAME`, the `viola mcp` stdin at `MAX_FRAME`, hook stdin,»
- line: .andromeda/obs-plan-detail/log-coverage.md:20
  old: «|channel-frame|prompt-submitted|»
  new: «|channel-frame|mcp-stdin`, `prompt-submitted|»
- line: .andromeda/obs-plan-detail/log-coverage.md:34
  old: «`hook-stdin`, `vt100-feed` when»
  new: «`hook-stdin`, `mcp-stdin` (a `viola mcp` stdin line past `MAX_FRAME`), `vt100-feed` when»
- after: .andromeda/obs-plan-detail/log-coverage.md:34
  anchor: «- `parse-rejected` (D-05) details, per `parser`: `malformed` (any parser), `unreadable` (`config-json`, `ledger-stamps`, `state-entry`), `oversize` (`channel-frame` beyond `MAX_FRAME`, `hook-stdin`, `vt100-feed` when the bounded feed queue drops a copy — one line per poisoning episode, `claude-agents-json` for an answer past `MAX_FRAME`, `AgentsFailure::Oversize`), `unknown-keys` (`config-json` skipped keys, counted in `count`), `panicked` (`vt100-feed` under `catch_unwind`), `control-character` (`paste-text`), `symlink-ignored` (`state-entry`), `strict-modes-failed` (`ledger-stamps`: `run`'s version gate refused the stamps file on the strict-modes check, and the read counts as unverified). `diag-line.v1.json` constrains `parse-rejected.detail` to this list (D-31).»
  add: «- `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` is written at `WARN` into `mcp.ndjson` by `viola_mcp::serve` through `obs_event!`, once per session, after the session ends, only when the stdin reader cut a line past `MAX_FRAME`; it holds no byte of the cut line.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:31
  anchor: «- vt100 panics are caught on `run`'s feed thread (`src/run/gate.rs`) by a `catch_unwind` around each feed and resize, not inside `run.readiness_gate`. Each poisoning episode gives exactly one `parse-rejected{parser:"vt100-feed", count:1}` at `WARN` with no `corr`, its `detail` set by the cause: `panicked` for a caught vt100 panic, `oversize` for a copy the bounded feed queue dropped (a size message carries the drop count at its offer, so a drop after a queued resize poisons the fresh screen again). The model stays poisoned until the host size changes, and `run` continues. `viola_panic_hook` runs before the catch returns, so a panic poisoning also writes the hook's `event:"panic"` role line and its detail line. Witnessed at run level by `tests/chaos_feed_panic.rs` (chunk 2026-10-04-confirmed-send-with-cl-1-records): exactly one `parse-rejected{vt100-feed, panicked}` and one `event:"panic"` line, both asserted present, then `viola send` refused `send-refused{detail:"input-not-ready"}` (exit 13) while a typed key still reaches the child. G2's single seam exemption is unchanged: that test's home lives outside `target/e2e-home` (`TestHome::outside_scan()`), so G2, G4 and the secret scan do not read it (the founder's ruling, live, 2026-10-04, relayed by the overseer).»
  add: «- The same test also witnesses the passthrough after the forced feed panic: none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` reaches the outer terminal on any OS, and on Unix the child's bytes `e4 b8 ad` reach it; the Windows byte reading is owed to the route entry "Windows-only live measurements", since ConPTY re-renders a wide character on a 1-column screen.»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:37
  anchor: «- MCP rejects on a refused instance check: `viola mcp`'s frame tools (`send`, `wait`, `last`, `answer`) run `check_instance` on the home and the instance directory in `frame` (`crates/viola-mcp/src/tools.rs`) before `live_endpoint`; a refusal returns `instance-unreachable` with `detail: null`, the `isError` code of the liveness pre-check, and writes no frame and no line.»
  add: «- A `viola mcp` stdin line past `MAX_FRAME` is no tool reject: it gives no `isError` result and no rmcp line (the reader signals the cut as end of stream, never as an I/O error, and `rmcp` is `OFF` in the sink); the session ends as at a closed stdin, with `process-exit` 0 once `initialize` has completed, or `ServeError::Initialize` (exit 11) when the cut lands before it; its one record is the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:32
  anchor: «     - The scan also asserts that the canary never appears in a home-level `diagnostics/*.ndjson`. It may appear only in `detail-*.ndjson`.»
  add: «     - The `mcp_tools` E2E case `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` runs on its own fresh home, writes the canary inside its over-long stdin line, and asserts the canary in no home-level file of that home.»

## Patterns
[[pattern]]
id = "obs-01"
fixed = "hook-stdin|channel-frame|prompt-submitted"

[[pattern]]
id = "obs-02"
fixed = "channel framing at `MAX_FRAME`, hook stdin"

[[pattern]]
id = "obs-03"
fixed = "`hook-stdin`, `vt100-feed` when the bounded feed queue"
