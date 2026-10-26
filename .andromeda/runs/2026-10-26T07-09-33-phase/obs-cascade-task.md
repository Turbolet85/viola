# Task — obs · 2026-10-26-server-verification-before-any-frame

## The chunk
Builds client-side server verification before the first frame in every client that writes to a wrapper's endpoint: CLI `send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`'s `hook.event` (every event) and `hook.dialog` (`pre-tool-use` · `permission-request`); `viola mcp`'s `send` / `wait` / `last` / `answer` tools through the Tokio client.
The check: the connected server's process id (Unix `peer_creds`, Windows the pipe's server process id) and its sysinfo start time match the `pid` and `started_at` the instance's `snapshot.json` records; macOS reads euid + dir only, a recorded gap.
On a mismatch or a check that cannot be made: CLI verbs exit 21 (instance unreachable), MCP tools return `instance-unreachable`, `viola hook` fails open (exit 0, nothing on stdout or stderr, no frame written).
It closes the dated exceptions one, four, five, six, seven and eight (a master amendment at the wrap); whether the strict-modes half comes with it is held for P4, and an unreadable or newer `snapshot.json` keeps reading as absent (exit 21 / fail open) under the founder's hold.
It re-reads at P3 the five `read_snapshot` callers (the client, `run`'s collision check, the hook twice, the statusline arm) and the Windows exiting-wrapper pipe window that skips `SessionEnd`'s direct-append fallback.

## Reached files
- src/cmd/client.rs · edits the clients' shared pre-check before the first frame
- src/cmd/send.rs · edits the pre-check before the `send` frame
- src/cmd/wait.rs · edits the pre-check before the `wait` frame
- src/cmd/last.rs · edits the pre-check before the `last` frame
- src/cmd/answer.rs · edits the pre-check before the `answer` frame
- src/cmd/pause.rs · edits the pre-check before the `pause` frame
- src/cmd/release.rs · edits the pre-check before the `release` frame
- src/cmd/hook.rs · edits the `hook.event` and `hook.dialog` clients' endpoint read and their fail-open-on-mismatch path
- src/cmd/hook/statusline.rs · re-reads its `read_snapshot` call (one of the five snapshot readers)
- src/cmd/run.rs · re-reads its collision check's `read_snapshot` call
- crates/viola-mcp/src/tools.rs · edits the `send` / `wait` / `last` / `answer` tools' pre-check before their frames
- crates/viola-channel/src/client.rs · edits the sync client's connect (the server identity read)
- crates/viola-channel/src/tokio_client.rs · edits the Tokio client's connect (the server identity read)
- crates/viola-channel/src/endpoint.rs · edits the endpoint's server pid read (Unix `peer_creds`, Windows the pipe's server process id)
- crates/viola-state/src/liveness.rs · edits `live_endpoint`, the liveness-only pre-check the verification joins
- crates/viola-state/src/snapshot.rs · re-reads `read_snapshot`'s absent-on-unreadable-or-newer outcome
- crates/viola-state/src/strict.rs · re-reads the existing strict-modes checks
- .claude/rules/security.md · edits the dated exceptions list (the first and fourth to eighth removed at the wrap)

## Needs
- The new mismatch outcome on CLI verbs: `process-exit` with exit 21 at WARN and a closed `detail` code that tells a failed server verification apart from the liveness causes (`server-verify-failed`, and `strict-modes-failed` if the strict-modes half comes with it); no path, pid or start time in the code (Founder Direction 4).
- The `wait` / `last` scenario's note that `strict-modes-failed` / `server-verify-failed` cannot occur until client-side server verification lands: this chunk is the entry that owns it, so the note and the scenario's failure lines change.
- The confirmed `send` scenario's client side (cli `send` or mcp `send` → channel): a send refused before any frame by a failed verification, logged codes-only with no sent text.
- The dialog → `answer` scenario: the `hook.dialog` client failing open on a mismatch, and a dialog hook's `hook-invoked` written only after the `hook.dialog` response, which on a refused verification never comes.
- The wheel scenario's `pause` / `release` clients: the same exit-21 outcome before their frames.
- The hook's fail-open path: a `hook-decision` with a fail-open `detail` at WARN, lines to `hook-<name>.ndjson` only, exit 0 with stdout and stderr empty, no `hook.event` or `hook.dialog` frame written (Vector 4 trigger).
- `SessionEnd`'s direct-append fallback in the hook: whether a verification mismatch reads as not delivered (so the fallback runs) is the CARRY's product consequence, and the hook's line must say which outcome was taken.
- MCP rejected tool calls: `instance-unreachable` as an `isError` code only, mirrored by a codes-only line in `mcp.ndjson` with no path, upstream text or type name (Vector 5 trigger).
- `channel-request` / `channel-response` on the client side: no `channel-request` line for a frame never written; `corr` + `conn` on every channel call that is made; `params` and result bodies never recorded.
- The server pid, the snapshot's `pid` and `started_at`: `pid` is a join key, never a grouping key or a `detail` value; the `endpoint` value is never logged on any line.
- sysinfo and interprocess are targets held `OFF` in the sink: a failed start-time read or peer-credential read is captured at viola's own seam as a viola event, never through those crates' logs.
- `viola-channel` and `viola-state` instrumentation stays sync; Tokio only in the `viola-mcp` path and `viola-channel`'s `tokio` feature, with no new obs dependency in the sync graph (Vector 9 trigger).
- A new typed error for a failed verification (in `viola-channel` or `viola-state`) has a fixed-message `Display`, crosses the crate seam logged once at WARN/ERROR with a closed `detail` / `error_code`, and carries its chain only to `detail-<process>.ndjson`.
- Strict-modes failure telemetry (Vector 7 trigger): today a strict-modes failure has a line only on `run`'s `ledger-stamps` read; if P4 takes the strict-modes half into the clients, that rule and the line catalog change with it.
- E5 snapshot recovery: `state-recovered` lines come only from `read_snapshot_or_replay`; the five readers stay on `read_snapshot` under the founder's hold, so the clients write no `state-recovered` line unless his word moves them.
- Client spans for the verification: `<area>.<operation>` under the existing areas (`send`, `wait`, `last`, `answer`, `pause`, `release`, `hook`, `mcp`, `channel`), static names, no instance name or path in a span name.
- The macOS recorded gap (euid + dir only): the same file sink, line schema and fields on every OS, no OS-specific field for the gap (multi-platform-exporter-compat trigger).
- The hook perf budget: the added peer-credential and sysinfo start-time read stays inside the spine hooks' and `pre-tool-use`'s `max` under the spine deadline and `session-end` under 1.0 s, gated by `gate --require perf`.
- The exit-21 `detail` code catalog and the `hook-decision` fail-open `detail` values: each new code added to the closed catalog and admitted by `schemas/diag-line.v1.json` (G4).

## Standing conventions
- S1 · every new log line goes through `obs_event!` with explicit typed fields; no raw `tracing` level macro or `event!` outside `viola_core::obs` · §11 Obs Anti-Patterns › Logs (.andromeda/obs-plan-detail/obs-anti-patterns.md)
- S2 · every new span is `#[instrument(skip_all, name = ..., fields(...))]` with an explicit allow-list, never `err` / `ret` · §4 Span / Trace Coverage
- S3 · every new field or `detail` value is named in the §6 catalog and admitted by `schemas/diag-line.v1.json` (default-deny, G4) · .andromeda/obs-plan-detail/log-coverage.md
- S4 · `detail` codes are closed kebab-case and carry no path or pid · §6 Log Coverage (.andromeda/obs-plan-detail/log-coverage.md)
- S5 · log levels follow the §6 mapping (exit 21 and fail-open `hook-decision` at WARN, refusals at INFO) · §6 Log Coverage
- S6 · home-level lines stay path-free and content-free; content and chains go only to `instances/<name>/diagnostics/detail-<process>.ndjson` · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
- S7 · no telemetry to stdout on any role, and nothing on the hook's stderr · §11 Obs Anti-Patterns › Logs
- S8 · typed errors carry fixed-message `Display`; no serde-sourced chain reaches stderr, `--json`, MCP `isError` or channel `error.data` · .andromeda/obs-plan-detail/error-capture-reporting.md
- S9 · every new integration-test home lives under `target/e2e-home/` so G2, G4 and the secret scan cover it · .andromeda/obs-plan-detail/ci-integration.md

## Not needed
- The `viola run` start sequence scenario's version gate, sideload, `--settings` argument and child spawn: the server side is unchanged, and `run`'s collision check is only re-read.
- The budget governor scenario (statusline shell-out, `budget.json`, `STATUSLINE_DEADLINE`): the statusline arm writes no frame, and its snapshot read is only re-read.
- E3 `link` / `unlink`: held for Epoch 6's own entry; nothing here serves them.
- E2 R8 strip fields on `process-start{subject:"claude-child"}`: no child is spawned by a client.
- Torn-line healing and the `events.ndjson` appender: no append path changes.
- The capability ledger's `ledger/stamps.json` read in `run.version_gate` and `verify` / `plugin install` logging: not clients of a wrapper's endpoint.
- `viola list`'s board and the bounded `claude agents --json` read and its `parse-rejected{parser:"claude-agents-json"}`.
- The panic hook, its catch sites and the zero-unlogged-panics invariant's machinery: unchanged.
- Derived metrics (`viola.<area>.<measure>`), `viola.compat.v_rejected` and metric label rules: no metric is added.
- CI step order, artifact uploads, retention, nightly fuzz and the Windows mutants workflow: no CI job is added.
- OTel SDK, OTLP exporter, sentry and veil `toggle`: no exporter or reporter is touched.
- The heartbeat tick and liveness transition lines: the heartbeat writer is unchanged.
- Windows pipe listener admission (DACL, remote rejection, first-instance): landed with 2026-10-25-windows-endpoint-admission.
