# Task — obs-plan · 2026-10-27-bounded-inputs-at-every-boundary

## The session
Bounded every external reader, closed the gaps it found, and witnessed each boundary in a new census test (`tests/contract_bounded_readers.rs`: 40 `BOUNDARIES` rows, 2 `EXCEPTIONS` rows, a census of every product reader).
`viola mcp`'s stdin now passes through the new crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME`. A line past the cap passes no byte and ends the session as a stdin close does. `viola_mcp::serve` then writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!`.
`schemas/diag-line.v1.json` adds `mcp-stdin` to the `parse-rejected.parser` enum, taking the catalog from 9 to 10 values. `detail` already held `oversize`.
`deny.toml` gains a `[[bans.features]]` table that bans serde_json `unbounded_depth`, and `scripts/deny-probes.sh` gains its probe row. The verdict moves from 13/13 to 14/14.
`replace_private_shared`'s fallback compare now reads at most `bytes.len() + 1` and adds no log line. `tests/chaos_feed_panic.rs` now also asserts the human passthrough on the outer terminal after a forced vt100 feed panic.
No IPC method, MCP tool, env var, CLI flag, exit code, error variant, config key or dependency was added. Nothing was retired, and no spec master was edited.

## Reached files
- crates/viola-mcp/src/stdin.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-state/src/fs.rs
- schemas/diag-line.v1.json
- deny.toml
- scripts/deny-probes.sh
- tests/contract_bounded_readers.rs
- tests/chaos_feed_panic.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/verification-matrix.json

## Touches
- §6 Log Coverage, the `parse-rejected` `parser` list (log-coverage.md, log-fields group, the parser list near line 20): it moves from 9 to 10 values with `mcp-stdin` added, matching the schema at line 87.
- §6 Log Coverage, the per-parser details (log-coverage.md, log-details group, near line 34): `mcp-stdin`'s one detail is `oversize`, meaning a stdin line past `MAX_FRAME`.
- §1 Obs Scope Summary, coverage triggers (obs-scope-summary.md, triggers group, near line 500): the per-parser parse-failure counts list gains the MCP stdin.
- §1 Telemetry surfaces, the ipc-internal `viola mcp` surface and its exporter line (obs-plan.md body near lines 125–127, and its scope-surfaces detail group): this is where the bounded stdin goes. A line past the cap is refused whole and ends the session.
- §1 entities `viola-mcp` and Claude Code as MCP client (body near lines 69 and 77, and the scope-entities detail groups): the inbound MCP stdin payload is now capped at `MAX_FRAME` per line.
- §12 Obs Decisions Log (`obs-plan-amendments.md`): the `parser` enum extension by `mcp-stdin` needs a Decisions Log entry, and the obs detector writes it.
- §8 default-deny and §9 G4 schema conformance: `schemas/diag-line.v1.json` now admits `parse-rejected{parser:"mcp-stdin"}`. The new line validates against it, which `mcp_tools` asserts. G4 over `mcp.ndjson` covers the line.
- §6 log levels mapping, `parse-rejected` at `warn`: re-read and unmoved. The new line is WARN.
- §6 per-module log levels, `rmcp` `OFF`: re-read and unmoved. The reader signals the cut as end of stream rather than as an I/O error, so rmcp writes nothing of its own, and its `tracing::error!` is OFF in the sink anyway.
- §3 harness contract (`mcp` writes only MCP frames to stdout) and §11 Logs (no telemetry to stdout): re-read and unmoved. On a cut, stdout holds only the frames answered before it and stderr stays empty.
- §7 `mcp` per-role behaviour and `process-exit`: an oversize cut on an initialized session ends with `process-exit` 0 and exit 0, as a closed stdin does. A cut before `initialize` completes returns `ServeError::Initialize`, exit 11. No test drives the pre-initialize cut.
- §1 Log file location (home-level `mcp.ndjson` never carries user content) and §8 Home-level lines path-free: re-read and unmoved. The `parse-rejected` line holds no byte of the cut line.
- §8 Verification (the content canary never reaches a home-level `diagnostics/*.ndjson`): the new `mcp_tools` oversize case starts from its own fresh home and reads a canary in every home-level file.
- §8 Scrubbing libraries (veil `toggle` banned by a cargo-deny `[[bans.features]]` table, body near line 560) and §11 PII Scrubbing (the veil ban, body near line 732): `deny.toml` now holds a second `[[bans.features]]` table, for serde_json `unbounded_depth`.
- §10 build / deploy failure conditions, the `cargo deny` list (Tokio, C-crate, veil `toggle`, banned obs crates; body near line 669): a serde_json `unbounded_depth` feature now also fails `cargo deny`. Any obs text that counts the deny probes moves from 13/13 to 14/14. The report's sweep found no such site in obs-plan.
- §7 vt100 panic capture (error-capture-reporting.md near lines 18 and 31; body near line 535): re-read and unmoved, since it already reads `panicked` for a caught panic and `oversize` for a dropped copy. `tests/chaos_feed_panic.rs` now asserts that none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, on every OS. On Unix it also asserts that the child's bytes `e4 b8 ad` pass through.
- §9 Pipeline integration, the one `tests/chaos_feed_panic.rs` home outside `target/e2e-home/` (body near line 600): the test gained its passthrough half, and its home exemption is unmoved.
- §10 zero unlogged panics and §9 G2: re-read and unmoved. `g2-zero-panics.sh` reads `g2: clean` after the chunk, and the oversize cut panics nothing.
- §11 Logs (only `obs_event!` writes a line): re-read and unmoved. The new `reject_oversize_line` writes through `obs_event!`.
- `viola_state::fs::replace_private_shared`'s bounded compare read: re-read with no new event, field or detail code. Wherever obs text names the pinned copy's replace, the line set is unchanged.

## New facts
- F1 · log-coverage · `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` is written at WARN by `viola_mcp::serve` through `obs_event!`, once per session, after the session ends, when the stdin reader cut a line past `MAX_FRAME`. The line holds no byte of the cut line.
- F2 · obs-scope-summary · `viola mcp`'s stdin is a bounded parser surface. `CappedLines` passes a line of at most `MAX_FRAME` bytes, counting the `\n`, and refuses a longer line whole, never truncating or skipping it. The session then ends as a stdin close does.
- F3 · error-capture-reporting · An MCP stdin line past `MAX_FRAME` gives no `isError` result and no rmcp-originated line. The session ends with `process-exit` 0 after `initialize`, or `ServeError::Initialize` (exit 11) before it, and the pre-initialize cut is driven by no test.
- F4 · pii-scrubbing-compliance · The `mcp_tools` E2E case `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` writes a canary in an over-long line and asserts it in no home-level file of its fresh home.
- F5 · slo-invariants-telemetry-budgets · `cargo deny` also fails on serde_json's `unbounded_depth` feature, through a `deny.toml` `[[bans.features]]` table proved by the `serde_json-unbounded-depth` row of `scripts/deny-probes.sh`, so the verdict reads `14/14 banned, control clean`.
- F6 · error-capture-reporting · After a forced vt100 feed panic, `tests/chaos_feed_panic.rs` asserts that no obs literal (`panic`, `parse-rejected`, `vt100`, `unable`, `hint:`) reaches the outer terminal on any OS. The Windows byte reading of the passthrough is owed to «Windows-only live measurements».

## Untouched
- §4 Span / Trace Coverage scenarios (`viola run` start sequence, confirmed `send`, `wait` / `last`, dialog → `answer`, wheel / `release`, budget governor): no span, span name or scenario step changed. The `mcp` frame-tool lines are true as written.
- §5 Metric Coverage: the derived counter keyed `parser` carries no list of values, so the new value needs no metric edit, and the label-cardinality rule is unmoved.
- §2 Telemetry Strategy and its triggers: the `mcp` lines there describe the frame tools and the `list` tool, which this session did not change.
- §7 MCP rejects (`isError` codes only, and `instance-unreachable` logged by no line): no frame tool's instance check, liveness pre-check or server check changed.
- §4 E5 snapshot replay, torn-append healing and heartbeat liveness: no state-recovery path, reader or line changed.
- §9 CI step order, artifact names, retention, G1 and G3: no CI step, workflow or gate command changed. The new E2E binary runs under the existing `run --e2e`.
- §9 G2's one exemption (the `fake-agent` seam's panic at `src/cmd/hook/seam.rs`): unchanged.
- The channel's own `channel-frame` parser line and close-on-oversize rule: the MCP cut leans on it as precedent but leaves it unchanged.
- §7 Platform pick and §11 Error Reporting (no sentry, no reporter, anyhow at the dispatch edge only): no error reporter, error type or chain mapping changed.
- §1 Logging stack and service identity (tracing-subscriber 0.3.23, rmcp 3.4.1 pins): no dependency was added or bumped, and `Cargo.lock` is untouched. rmcp's features stay `server` and `transport-io`.
- §10 performance budgets (hyperfine `max` on the hook rows): no hook path or perf row changed.
- The Path 1 CI-timeout WATCH (1/3 green): it lives on the working route, not in obs-plan, and no case of this chunk touches `run`'s start sequence.
