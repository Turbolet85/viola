proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 125
    section: '## 1. Obs Scope Summary'
    change: 'the Surface line reads: - **Surface:** ipc-internal (`viola mcp`: stdio MCP server, Tokio allowed; its stdin read in lines of at most `MAX_FRAME`, a longer line refused whole and the session ended as a stdin close does)'
    sidecar: '§1 Telemetry surfaces: the `viola mcp` surface names its stdin bounded at `MAX_FRAME` per line (chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'the report names `obs-plan.md:125` as the site for the bounded stdin (Expected amendments, obs-plan); `viola_mcp::serve` now reads stdin through `CappedLines::new(stdin, MAX_FRAME)` and a cut ends the session (Symbols / APIs)'
    basis: '.andromeda/obs-plan.md:125'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 99
    section: '## 1. Obs Scope Summary'
    change: 'the reason line reads: - **Reason if not fully instrumentable:** The caller is Claude Code, so viola sees only the inbound stdin payload and its own response; on `viola mcp` that payload is read in lines of at most `MAX_FRAME`, a longer line refused whole. Instrument `hook-invoked` / `hook-decision`, MCP tool-call handling and the stdin cut (one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`, no byte of the line). No trace context arrives from Claude Code.'
    sidecar: '§1 scope entities: Claude Code as MCP client, its inbound stdin capped at `MAX_FRAME` per line and the cut instrumented by one `parse-rejected{mcp-stdin}` (chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Symbols / APIs: `serve` hands rmcp `CappedLines` at `MAX_FRAME` and, on a cut, writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!` with no byte of the line; Coverage of new surfaces lists the instrumentation log'
    basis: 'crates/viola-mcp/src/lib.rs:93-102'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 212
    section: '## 1. Obs Scope Summary'
    change: 'the Exporter line gains, at its end: `viola mcp` stdin is a bounded parser surface: `CappedLines` passes a line of at most `MAX_FRAME` bytes, the `\n` counted, and refuses a longer line whole, never truncating or skipping it; the session then ends as a stdin close does (as landed, chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    sidecar: '§1 scope surfaces: the `viola mcp` stdin bound recorded beside its exporter (F2; chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Symbols / APIs (`CappedLines`: a line of exactly the cap passes, a longer one releases no byte and the reader reads as end of stream) and Decisions & corrections (refused whole, never truncated, never skipped)'
    basis: 'crates/viola-mcp/src/stdin.rs:17-29'
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 500
    section: '## 1. Obs Scope Summary'
    change: 'the last trigger bullet reads: - Per-parser parse-failure counts: paste validation, channel framing at `MAX_FRAME`, the `viola mcp` stdin at `MAX_FRAME`, hook stdin, `claude agents --json`, vt100 under `catch_unwind`.'
    sidecar: '§1 triggers: the per-parser parse-failure counts list gains the MCP stdin (chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Counts / qualifiers moved, pattern `parse-counts`: `obs-plan-detail/obs-scope-summary.md:500` changes, the list gaining the MCP stdin'
    basis: '.andromeda/obs-plan-detail/obs-scope-summary.md:500'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 20
    section: '## 6. Log Coverage'
    change: 'the row reads: | `parse-rejected` (D-05) | `parser` (`claude-agents-json|vt100-feed|config-json|ledger-stamps|hook-stdin|channel-frame|mcp-stdin|prompt-submitted|paste-text|state-entry`), `detail`, `count` |'
    sidecar: '§6 log fields: D-05 `parse-rejected.parser` extended by `mcp-stdin`, 9 → 10 values, matching `schemas/diag-line.v1.json` (chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Schema / config: the `parse-rejected.parser` enum gains `mcp-stdin` in that order; Counts moved, pattern `parser-catalog`: `obs-plan-detail/log-coverage.md:20` changes'
    basis: 'schemas/diag-line.v1.json:87'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 34
    section: '## 6. Log Coverage'
    change: 'the `oversize` clause reads: `oversize` (`channel-frame` beyond `MAX_FRAME`, `hook-stdin`, `mcp-stdin` for a `viola mcp` stdin line past `MAX_FRAME`, `vt100-feed` when the bounded feed queue drops a copy — one line per poisoning episode, `claude-agents-json` for an answer past `MAX_FRAME`, `AgentsFailure::Oversize`); the rest of the bullet unchanged'
    sidecar: '§6 log details: `mcp-stdin` takes `oversize` as its one detail (chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Counts moved, pattern `parser-catalog`: `log-coverage.md:34` changes, its per-parser details gaining `oversize` (`mcp-stdin` past `MAX_FRAME`); Schema / config: `detail` already held `oversize`'
    basis: 'schemas/diag-line.v1.json:87'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 34
    section: '## 6. Log Coverage'
    change: 'a bullet follows: - `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` (D-05) is written at `WARN` by `viola_mcp::serve` through `obs_event!` (its private `reject_oversize_line`), once per session, after the session ends, only when the stdin reader cut a line past `MAX_FRAME`; it holds no byte of the cut line (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    sidecar: '§6 log details: the `mcp-stdin` line, its writer, level and timing (F1; chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Symbols / APIs (`serve` writes exactly one such line after the session ends when the cut flag is set, no byte of the line) and Outcome (the obs criterion: the schema admits it and the line validates)'
    basis: 'crates/viola-mcp/src/lib.rs:93-102'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 31
    section: '## 7. Error Capture & Reporting'
    change: 'a bullet follows: - The same test also witnesses the passthrough after the forced feed panic: none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` reaches the outer terminal on any OS, and on Unix the child bytes `e4 b8 ad` reach it; the Windows byte reading of the passthrough is owed to «Windows-only live measurements», since ConPTY re-renders a wide character on a 1-column screen (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    sidecar: '§7 vt100 panic capture: `tests/chaos_feed_panic.rs` gains its passthrough half, the Windows byte reading owed (F6; chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Coverage of new surfaces (the tui passthrough after a feed panic) and Decisions & corrections (no Windows byte oracle; owed by the P5 CARRY on «Windows-only live measurements», PROVISIONAL)'
    basis: 'tests/chaos_feed_panic.rs:109-130'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 37
    section: '## 7. Error Capture & Reporting'
    change: 'a bullet follows: - An MCP stdin line past `MAX_FRAME` is no tool reject: it gives no `isError` result and no rmcp-originated line (the reader signals the cut as end of stream, and `rmcp` is `OFF` in the sink); the session ends with `process-exit` 0 after `initialize` (exit 0, as at a closed stdin), or with `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes, a path no test drives; its one record is the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    sidecar: '§7 MCP errors: the stdin cut, its exits and the undriven pre-initialize path (F3; chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Symbols / APIs (`serve` returns what the session gave: `Ok(())` initialized, `ServeError::Initialize` before), Deviations 1 (driven by no test) and Decisions & corrections (end of stream, never an I/O error, so rmcp writes nothing)'
    basis: 'crates/viola-mcp/src/lib.rs:93-102'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 32
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'a sub-bullet follows the home-level canary bullet: - The `mcp_tools` E2E case `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` starts from its own fresh home, writes the canary inside its over-long stdin line, and asserts the canary in no home-level file of that home (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    sidecar: '§8 verification: the MCP oversize case reads its canary in every home-level file of its fresh home (F4; chunk 2026-10-27-bounded-inputs-at-every-boundary)'
    rationale: 'report Coverage of new surfaces (PII redacted: no byte of the line; the E2E case reads a canary in every home-level file) and Expected amendments (the oversize case starts from its own fresh home)'
    basis: 'crates/viola-e2e/tests/mcp_tools.rs:441-559'
    fact: F4
note: 'the D-05 `parser` extension by `mcp-stdin` still needs its §12 Decisions Log entry in obs-plan-amendments.md (outside these files); F5 (slo-invariants-telemetry-budgets) and the body §10 `cargo deny` failure list gaining serde_json `unbounded_depth` have no picked entity here, left to the new-detail pass; detectors D-obs-instrumentation, D-obs-stack and D-obs-pii read clean (the cut is logged via `obs_event!`, no dependency moved, no byte of the line logged)'
