### obs-scope-summary#2.4 · pending · tier · ## 1. Obs Scope Summary · lines 20–20 · stale
- `hook-invoked` `corr` timing relative to `dialog_id` assignment.
### obs-scope-summary#6.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 52–52 · add-to
- **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
### obs-scope-summary#12.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 107–107 · add-to
- **Reason if not fully instrumentable:** External CLI. Instrument the spawn/exit and the parse outcome. A parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
### obs-scope-summary#14.3 · values · scope-entities · ## 1. Obs Scope Summary · lines 123–123 · add-to
- **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use), not the OS internals.
### obs-scope-summary#21.5 · values · scope-surfaces · ## 1. Obs Scope Summary · lines 200–203 · add-to
- **Notes:**
  - JSON-RPC ids are "integers, monotonic per connection" (arch Standard Contracts, JSON-RPC frame shapes). `corr` for `channel-request` / `channel-response` is therefore not unique across connections or processes, and a merged-log join needs `process`, `instance` and a connection discriminator. This is a gap for Phase 3; the tests contract allows additive fields.
  - `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).
### obs-scope-summary#25.0 · mirror · log-fields · ## 1. Obs Scope Summary · lines 258–278 · contradict
```markdown
- **Format:** JSON-per-line. There are two streams:
  - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
  - **Process logs:** emitted by tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, writing to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`. Files are 0600, dirs 0700, with one `write` per line.
- **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
    - `process-start`, `process-exit`, `panic`: `corr` = null
  - `process` (`run|hook|mcp|cli`) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).

  obs-plan may add fields but must not rename or remove these. The harness greps on them.
- **Constraints:**
  - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  - No line may contain a `Cookie` header or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
```
### obs-scope-summary#30.3 · values · paths · ## 1. Obs Scope Summary · lines 327–327 · contradict
- **Required log fields:** `event`, `process`, `instance`, `corr` (`dialog_id`), `dialog_kind`, `refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12), `deadline_hit`, `duration_ms`. The question, answer and tool `input` text never appear.
### obs-scope-summary#34.1 · values · triggers · ## 1. Obs Scope Summary · lines 361–363 · stale
- **Required telemetry signal:**
  - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  - `send-refused` with detail `control-character` or `empty-text` is emitted by each deciding side, client and wrapper, without the text (tests excerpt, Coverage Triggers, Vector 2).
### obs-scope-summary#36.1 · values · triggers · ## 1. Obs Scope Summary · lines 378–378 · stale
- **Required telemetry signal:** Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names.
### obs-scope-summary#41.1 · values · triggers · ## 1. Obs Scope Summary · lines 418–421 · contradict
- **Required telemetry signal:**
  - `corr` = send `cursor` joins `send-issued` / `send-confirmed` / `send-refused` across CLI, MCP and wrapper.
  - `corr` = `dialog_id` joins hook → wrapper → driver.
  - The per-connection JSON-RPC `id` needs a discriminator for a merged-log join (section 2).
### obs-scope-summary#42.1 · values · triggers · ## 1. Obs Scope Summary · lines 427–431 · stale
- **Required telemetry signal:**
  - `duration_ms` on `hook-decision`, or on `process-exit` for `process="hook"`, per hook event. The budget is `session-end` `max < 1.0 s`, and spine hooks and `pre-tool-use` on an unverified CLI must stay under the spine deadline (provisionally 1.0 s).
  - The heartbeat staleness flip happens at >5 s (4.9 s live / 5.1 s stale or gone).

  Aggregation into p95/p99 is done by the harness or `jq` over the ndjson, not by an in-process metrics pipeline (no listener, no exporter).
### obs-scope-summary#43.1 · values · triggers · ## 1. Obs Scope Summary · lines 437–444 · stale
- **Required telemetry signal:** Fault telemetry per class:
  - torn ndjson line healed, with file and offset;
  - corrupt or unsupported-`v` snapshot → replay recovery;
  - child exit detected on the process handle without EOF (ConPTY);
  - endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
  - stale heartbeat vs live pid.

  These need `event` values or an additive `detail` field (section 3, gap b).
### obs-scope-summary#44.1 · values · triggers · ## 1. Obs Scope Summary · lines 450–456 · add-to
- **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  - permission application: Windows DACL vs Unix 0600/0700;
  - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  - named pipe vs Unix socket in channel logs;
  - ConPTY vs openpty in `pty` spans.

  No OS-specific exporter.
### obs-scope-summary#48.1 · values · triggers · ## 1. Obs Scope Summary · lines 483–485 · stale
- **Required telemetry signal:**
  - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
  - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor).
### telemetry-strategy#1.3 · values · strategy · ## 2. Telemetry Strategy · lines 12–12 · stale
| Errors | Panics (a one-line `event:"panic"` record), typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit` | No external reporter: security says "no outbound network calls". A hand-written `std::panic::set_hook`, plus `std::panic::catch_unwind` at the main-thread catch site of every role, around `hook` dispatch, in the `run` PTY pump and handle-wait threads, and around the vt100 feed (Section 7) |
### telemetry-strategy#2.1 · values · strategy · ## 2. Telemetry Strategy · lines 20–20 · stale
- Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance. The bounded exemptions (the pre-init window, `cli` without an instance, `hook` without `VIOLA_NAME`, a failed step-4 role-file open, the no-obs-init hidden capture arms (`hook <event> --capture` and `hook statusline --capture`), and a refused instance check on either `viola revive` arm (start or `--list`) or on `viola hook statusline`, or a refused `viola list` scan, which opens none either) are listed in Section 10. The payload (one escaped JSON string) and the backtrace (a one-line JSON array of frames) are written only to the instance-scoped detail file (Section 7).
### telemetry-strategy#3.3 · values · triggers · ## 2. Telemetry Strategy · lines 30–30 · add-to
| logging-sensitive Vector 4 (hook stdin) | §3 Logging stack (hook sink), §4 Scenario 4, §6 `hook-decision` `detail` codes. The "missing `VIOLA_NAME`" item is **overridden**: no file and no line, asserted by absence (D-16, §4 E1). The "strict-modes failure" item writes no line as landed either: `viola hook statusline`'s refused instance check leaves no trace, asserted by absence (chunk 2026-10-10-budget-governor) |
### telemetry-strategy#3.9 · values · triggers · ## 2. Telemetry Strategy · lines 36–36 · add-to
| cross-surface-trace-propagation | §3 Trace context propagation (`corr` + `conn`), §4 Scenarios 2/4 |
### telemetry-strategy#3.13 · values · triggers · ## 2. Telemetry Strategy · lines 40–40 · stale
| error-budget-SLO | §7, §10 zero-unlogged-panics |
### telemetry-strategy#3.16 · values · triggers · ## 2. Telemetry Strategy · lines 43–43 · add-to
| creator-explicit (Founder Direction 4) | §6 detail code catalog, §12 D-01 |
