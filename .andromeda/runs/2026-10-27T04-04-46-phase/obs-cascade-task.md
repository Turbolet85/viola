# Task — obs · 2026-10-27-bounded-inputs-at-every-boundary

## The chunk
Makes every input crossing into a `viola` process bounded where it enters: a `Read::take(MAX_FRAME)` (or the `MAX_FRAME + 1` over-size form) on every external reader, serde_json at its default depth, closed types for every decision read from outside, `ViolaName::try_new` (or a closed-shape validator) before every path join, and the paste rule on the client and in the wrapper.
The readers it covers: `hook` and `mcp` stdin, channel frames on both sides, the child's PTY output into the vt100 feed, `claude agents --json` and `claude --version` output, `send`'s `--file` / stdin text, the status line command's stdout, and the home and `~/.claude` files (`config.json`, `snapshot.json`, `events.ndjson`, `budget.json`, `ledger/stamps.json`, `statusline-source.json`, `settings.json`). Any reader found without a bound gets one.
It re-verifies the paste rule (`validate_paste_text`: `not-delivered` / `control-character`, `empty-text`, never a strip) at the CLI `send` and MCP `send` clients and in the wrapper before the first wheel read. It also re-verifies that a vt100 screen-model panic degrades readiness to `input-not-ready` and leaves the child's bytes passing through to the human's terminal.
It adds a unit-level witness per property and per boundary, plus an enumeration of the external readers that a reader can check, each listed with its cap, depth and type rule. The candidate claim is `verification-matrix.json#v1-46`, its web clause left out.

## Reached files
- src/run/gate.rs · re-reads its `catch_unwind` site (the vt100 panic degrade to `input-not-ready`)
- crates/viola-agent-claude/src/screen.rs · re-reads its `catch_unwind` site (the screen-model panic degrade and passthrough)
- viola-0.1.0/verification-matrix.json · edits the `v1-46` row

## Needs
- The vt100 feed panic path: `run`'s feed thread wraps each feed and resize in `catch_unwind`, each poisoning episode writes exactly one `parse-rejected{parser:"vt100-feed"}` at WARN, and `run` keeps running. The re-verified degrade must still produce this.
- The readiness-gate `input-not-ready` outcome as a logged signal (the §1 alignment gap (b) and how it was resolved through an additive `detail` or an enum extension). A witness of the degrade reads this signal.
- The zero-unlogged-panics invariant and how G2 counts: a panic forced on purpose in a witness still fires the panic hook's one `event:"panic"` line, so a forced vt100 panic test has to run inside the bounded exemption, the `tests/chaos_feed_panic.rs` home outside `target/e2e-home/`.
- The ban on per-byte or per-chunk spans or events in the PTY pump read loop and the vt100 feed. The size cap on the PTY output into the feed adds no per-chunk line; only gate outcomes or a count on the boundary event are logged.
- `send-refused` for `control-character` and `empty-text` on each deciding side (CLI client, MCP client, wrapper): codes only, never the text, and the paste reaches no line except as `text_bytes`.
- `parse-rejected{parser:"config-json"}`: one line per `config.json` key that gets a bad value (`five_hour`, `seven_day`, `dialog_hold_s`), which falls back to its default. The closed-type config read emits these lines and no others.
- `claude agents --json` parse failure, including an oversize or malformed read: one `parse-rejected{parser:"claude-agents-json"}` and the value `"unknown"`, the rows never logged verbatim. This covers the closed row-status type.
- The `events.ndjson` reader's three skip counts and the rule that an unknown kind or field is skipped and counted, never refused. A closed-type reading of a viola format follows this rule, and no format uses `deny_unknown_fields`.
- The `state-recovered{detail:"snapshot-replayed"|"snapshot-unsupported-v"}` line from `read_snapshot_or_replay` at WARN, one per call. The bounded snapshot read keeps this emission.
- Channel frames on both sides: `channel-request` / `channel-response` never record `params` or result bodies. An over-size or invalid frame or param is answered `-32602` / `-32600` at WARN, `-32603` is exactly `internal error` with `data: null`, and `from` is logged as unauthenticated.
- The `viola.compat.v_rejected` derived metric over-counts `-32602`. Any new invalid-params refusal from a channel `name` / `driver` / `driven` check adds to that over-count until the line tells the causes apart.
- `hook` stdin through `take(MAX_FRAME + 1)`: on oversize or malformed stdin the hook fails open (exit 0, stderr empty, no body), logging only to `hook-<name>.ndjson` with a fail-open `hook-decision` `detail`. `hook statusline` prints nothing on oversize stdin.
- `mcp` stdin and tool arguments: a rejected call is an `isError` with codes only (`invalid-params`, `instance-unreachable`), its log line codes-only with no upstream text, path or type name. `instance-unreachable` gets no line at all.
- Names before path joins: an instance resolves only from `VIOLA_NAME` or an argument that passed `ViolaName::try_new`, never from the working directory. A name the validator refuses resolves no instance, so no role file opens and nothing is written to `diagnostics/`.
- The capability-ledger read inside `run.version_gate`: an unreadable, oversize, malformed or strict-modes-refused `ledger/stamps.json` reads as `cli_verified:false`, and the path and stamp contents never appear in a line.
- The `budget.json` checked read: every failure, oversize included, is no reading and no line. The statusline command's stdout passes through unchanged under its `take(MAX_FRAME)` and is never logged.
- The R8-stripped `CLAUDE*` NEVER-log floor over the `claude --version` and `claude agents --json` spawns the chunk bounds: names and counts only on `process-start`, never values.

## Standing conventions
- S1 · every new log line goes through `obs_event!` under `#[instrument(skip_all, fields(..))]`, with no raw tracing level macro or `event!` outside `viola_core::obs` and no `err` / `ret` option · §11 Obs Anti-Patterns (Logs, Spans / Traces); .andromeda/obs-plan-detail/obs-anti-patterns.md
- S2 · a new `event` value, field, `parser` value or `detail` code joins the §6 catalog (closed, kebab-case, never a path or pid) and `schemas/diag-line.v1.json`'s default-deny, checked by G4 · .andromeda/obs-plan-detail/log-coverage.md
- S3 · no content (paste text, payload or over-size bytes, row text, command strings) in any home-level line; content goes only to the instance's `detail-<process>.ndjson` inside a chain or drift report · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
- S4 · every new integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it · .andromeda/obs-plan-detail/ci-integration.md
- S5 · a new test meets the zero-unlogged-panics, zero-secret-hit and zero-schema-failure budgets, with no retry-once policy · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
- S6 · no `print!` / `eprintln!` / `dbg!` in product code paths; stdout stays reserved on every role · §11 Obs Anti-Patterns (Logs)

## Not needed
- The error reporting platform (sentry, any reporter, `before_send`) and the OTel SDK or exporter choice: the chunk adds no reporter and no exporter.
- The serde-chain-to-fixed-message dispatch layer and external error-body sanitisation: owned by the next route entry, "Sanitised error surfaces".
- Completing a distinct `detail` code for each exit 21 / exit 1 cause (Founder Direction 4): owned by the route entry "Exit-cause code catalogue".
- The server-identity check lines (`server-verify-failed`), peer credentials and the macOS peer-pid gap: owned by "Unix endpoint and home hardening".
- Windows sideload telemetry (`sideload_fallback`) and Windows-only live readings: owned by "Windows-only live measurements".
- Heartbeat liveness transitions and the >5 s stale flip: the chunk changes no liveness signal.
- The human-wheel scenario's `wheel` events and `pause` / `release` lines: the paste refusal comes before the first wheel read and changes no wheel line.
- The `corr` / `conn` join key and trace-context propagation: the chunk adds no correlation field.
- Service identity resource attributes (`service_name`, `version`): unchanged.
- Audit-log retention and rotation of `events.ndjson` or `diagnostics/`: no rotation is added.
- The `Last-Event-ID` / resume-cursor clause and every frontend or web surface: carried to 0.2.0.
- CI artifact retention days and the pinned upload Action: the chunk adds no CI job or upload.
