## 2026-10-27-bounded-inputs-at-every-boundary — the feed-panic test's passthrough half
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the vt100 panic capture block)
**Change:**
- A bullet follows the `tests/chaos_feed_panic.rs` bullet: the same test also witnesses the passthrough after the forced feed panic. None of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` reaches the outer terminal on any OS, and on Unix the child's bytes `e4 b8 ad` reach it.
- The Windows byte reading is owed to the route entry "Windows-only live measurements", since ConPTY re-renders a wide character on a 1-column screen.
**Why:** the report's Coverage of new surfaces and Outcome: `chaos_feed_panic_refuses_send_and_keeps_passthrough` is green on ubuntu, macOS and windows-2025, Windows reading the literal absence only. No Windows byte oracle is assertable, so the reading is owed by the P5 card's Beyond-the-plan CARRY on «Windows-only live measurements», approved by delegate overseer1 (founder-delegated), relayed by the operator, PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the `viola mcp` stdin line cap and the session it ends
**Section:** §7 Error Capture & Reporting (error-capture-reporting.md: the MCP instance-check reject block); §1 Obs Scope Summary (obs-plan.md: Telemetry surfaces, the `viola mcp` Surface line; obs-scope-summary.md: the Claude Code caller's scope entity, the `viola mcp` scope surface's Exporter)
**Change:**
- §7 gains: a `viola mcp` stdin line past `MAX_FRAME` is no tool reject. It gives no `isError` result and no rmcp line (the reader signals the cut as end of stream, never as an I/O error, and `rmcp` is `OFF` in the sink). The session ends as at a closed stdin: `process-exit` 0 once `initialize` has completed, or `ServeError::Initialize` (exit 11) when the cut lands before it. Its one record is the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.
- The §1 Surface line was "ipc-internal (`viola mcp`: stdio MCP server, Tokio allowed)". Now it adds that stdin is read in lines of at most `MAX_FRAME`, a longer line refused whole and the session ended as at a closed stdin.
- The caller's reason adds that `viola mcp` reads the inbound payload in lines of at most `MAX_FRAME`, a longer line refused whole. Its instrument list was `hook-invoked` / `hook-decision` and MCP tool-call handling; now it also names the stdin cut (one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`, no byte of the line).
- The Exporter adds: rmcp reads stdin only through viola's line cap (`CappedLines` at `MAX_FRAME`). A line of at most `MAX_FRAME` bytes, its newline counted, passes whole. A longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, with stderr empty.
**Why:** the report's Symbols / APIs: `viola_mcp::serve` hands rmcp `CappedLines::new(stdin, MAX_FRAME)`, and rmcp 3.4.1 reads each stdin line with an unbounded `read_until`, so the bound belongs to the reader viola hands it. Its Decisions & corrections: a truncated line would dispatch a padded request and a skipped one would go unanswered with no signal; the channel's close-on-oversize rule is the precedent. §1 is a verbatim upstream copy kept current, by the founder's ruling of 2026-10-04, relayed by the overseer.
**Kept:** §7 does not say that no test drives the exit-11 path. That gap is stated once, in test-plan, with its owner.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — D-05 `parse-rejected` gains the `mcp-stdin` parser
**Section:** §6 Log Coverage (log-coverage.md: the `parse-rejected` field row, its per-parser details, a new `mcp-stdin` bullet); §1 Obs Scope Summary (obs-scope-summary.md: the triggers' per-parser parse-failure counts); §12 Obs Decisions Log (this entry is the D-05 extension's record)
**Change:**
- The `parser` list names `mcp-stdin` after `channel-frame`: 10 values, up from 9. The rule is the `parse-rejected.parser` enum of `schemas/diag-line.v1.json`.
- `oversize` gains `mcp-stdin` (a `viola mcp` stdin line past `MAX_FRAME`), the one detail it takes.
- A new bullet: `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` is written at `WARN` into `mcp.ndjson` by `viola_mcp::serve` through `obs_event!`. It is written once per session, after the session ends, only when the stdin reader cut a line past `MAX_FRAME`, and it holds no byte of the cut line.
- The per-parser parse-failure counts add the `viola mcp` stdin at `MAX_FRAME`.
**Why:** the report's Schema / config: the schema's `parser` enum gains `mcp-stdin`, and `detail` already held `oversize`. Its Outcome: the schema admits the line, and the line validates against it. A new closed `parser` value takes a Decisions Log record. That log lives in this file, and §12 of the body points here.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the MCP oversize case's canary check
**Section:** §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: PII Scrubbing, the High classes' canary bullets)
**Change:**
- A sub-bullet follows the home-level canary bullet. The `mcp_tools` E2E case `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` runs on its own fresh home and writes the canary inside its over-long stdin line. It asserts that the canary is in no home-level file of that home.
**Why:** the report's Coverage of new surfaces: the cut writes no byte of the line, and the E2E case looks for a canary in every home-level file. Its Expected amendments: the oversize case starts from its own fresh home.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/
