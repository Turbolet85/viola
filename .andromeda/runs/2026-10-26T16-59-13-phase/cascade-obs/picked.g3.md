### obs-scope-summary#29.1 · values · paths · ## 1. Obs Scope Summary · lines 316–316
- **Surfaces involved:** cli or ipc-internal (mcp) → ipc-internal (channel) → cli (`run`) + persistent store (`events.ndjson` tail)
### obs-scope-summary#29.2 · values · paths · ## 1. Obs Scope Summary · lines 317–317
- **Must-trace spans:** `wait.block` (client) › `channel.request` › `run.wait_dispatch`. Log events: `channel-request` and `channel-response` with the outcome.
### obs-scope-summary#29.3 · values · paths · ## 1. Obs Scope Summary · lines 318–318
- **Required log fields:** `event`, `process`, `instance`, `corr` (JSON-RPC `id`), `after` (byte offset), `timeout_ms`, `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
### obs-scope-summary#29.4 · values · paths · ## 1. Obs Scope Summary · lines 319–319
- **Source:** tests excerpt, Critical Path 3; arch hint 3; design excerpt, CLI `viola wait` (span with outcome attribute); creator must-work "Waking without polling"
### obs-scope-summary#30.1 · values · paths · ## 1. Obs Scope Summary · lines 325–325
- **Surfaces involved:** cli (`hook` PreToolUse / PermissionRequest) → ipc-internal (`hook.dialog`) → cli (`run`) ← cli or ipc-internal (mcp) `answer`
### obs-scope-summary#30.2 · values · paths · ## 1. Obs Scope Summary · lines 326–326
- **Must-trace spans:** `hook.handle` › `channel.request(hook.dialog)` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`; on the driver side, `answer.client` › `channel.request(answer)`. Log events: `hook-invoked`, `dialog-raised`, `dialog-answered`, `hook-decision` (decision null on deadline expiry).
### obs-scope-summary#30.3 · values · paths · ## 1. Obs Scope Summary · lines 327–327
- **Required log fields:** `event`, `process`, `instance`, `corr` (`dialog_id`), `dialog_kind`, `refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12, `server-verify-failed` on Linux and Windows: exit 21 on CLI `answer` with `during:"connect"`, fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint), `deadline_hit`, `duration_ms`. The question, answer and tool `input` text never appear.
### obs-scope-summary#30.4 · values · paths · ## 1. Obs Scope Summary · lines 328–328
- **Source:** tests excerpt, Critical Path 4 and Critical Path 7 (unverified CLI gate folded in as the answer-withheld branch); arch hint 4; creator must-work "Dialog answering"
### obs-scope-summary#31.1 · values · paths · ## 1. Obs Scope Summary · lines 334–334
- **Surfaces involved:** cli (`run`, key input) + `events.ndjson` (`wheel`) → cli or ipc-internal (mcp) `send` refused → cli `release` → ipc-internal
### obs-scope-summary#31.2 · values · paths · ## 1. Obs Scope Summary · lines 335–335
- **Must-trace spans:** `run.wheel_transition` › `channel.request(send)` refused › `pause.client` / `release.client` › `channel.request(pause|release)`. Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`. `release-from-driver` is logged as its own event (Founder Direction 4), admitted by `schemas/diag-line.v1.json`.
### obs-scope-summary#34.1 · values · triggers · ## 1. Obs Scope Summary · lines 361–363
- **Required telemetry signal:**
  - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  - `send-refused` with detail `control-character` or `empty-text` is emitted by each deciding side, client and wrapper, without the text (tests excerpt, Coverage Triggers, Vector 2).
### obs-scope-summary#35.1 · values · triggers · ## 1. Obs Scope Summary · lines 369–373
- **Required telemetry signal:**
  - Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`. As landed a strict-modes failure is not logged: `viola hook statusline` is the one arm that runs an instance check, and a refused check leaves no line (chunk 2026-10-10-budget-governor; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer, the operator's answer at its planning superseded 2026-10-13).
  - A server-verify mismatch (Linux and Windows) is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` — and no pid, start time, path or endpoint in the line; macOS compares no peer pid (the recorded gap).
  - Exit is 0 and stderr stays empty.
  - Drift reports go to `diagnostics/` only.
### obs-scope-summary#36.1 · values · triggers · ## 1. Obs Scope Summary · lines 379–379
- **Required telemetry signal:** Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names.
### obs-scope-summary#37.1 · values · triggers · ## 1. Obs Scope Summary · lines 385–389
- **Required telemetry signal:**
  - A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
  - The R8 strip is logged as a count or variable-name list only, never values. Phase 3 must confirm names are admissible.
  - An anyhow chain containing a serde source becomes a fixed message before reaching stderr.
  - No env, flag or config may disable redaction.
### obs-scope-summary#38.1 · values · triggers · ## 1. Obs Scope Summary · lines 395–398
- **Required telemetry signal:**
  - `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), enforced by the tests secret-scan test.
  - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate. Five strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check, `viola link` / `viola unlink`'s instance check of the driven instance (made before any role file opens; exit 21), the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list).
  - `skipped.torn_lines` counts.
### obs-scope-summary#39.1 · values · triggers · ## 1. Obs Scope Summary · lines 404–407
- **Required telemetry signal:**
  - `process-start` / `process-exit` for the `claude` child and for `claude agents --json` / `--version` probes.
  - Parse failures surface as `unknown` plus a counter.
  - C0/C1 controls are escaped in any human terminal rendering of logged rows.
### obs-scope-summary#42.1 · values · triggers · ## 1. Obs Scope Summary · lines 428–432
- **Required telemetry signal:**
  - `duration_ms` on `hook-decision`, or on `process-exit` for `process="hook"`, per hook event. The budget is `session-end` `max < 1.0 s`, and spine hooks and `pre-tool-use` on an unverified CLI must stay under the spine deadline (provisionally 1.0 s).
  - The heartbeat staleness flip happens at >5 s (4.9 s live / 5.1 s stale or gone).

  Aggregation into p95/p99 is done by the harness or `jq` over the ndjson, not by an in-process metrics pipeline (no listener, no exporter).
### obs-scope-summary#43.1 · values · triggers · ## 1. Obs Scope Summary · lines 438–445
- **Required telemetry signal:** Fault telemetry per class:
  - torn ndjson line healed, with file and offset;
  - corrupt or unsupported-`v` snapshot → replay recovery;
  - child exit detected on the process handle without EOF (ConPTY);
  - endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
  - stale heartbeat vs live pid.

  These need `event` values or an additive `detail` field (section 3, gap b).
### obs-scope-summary#44.1 · values · triggers · ## 1. Obs Scope Summary · lines 451–458
- **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  - permission application: Windows DACL vs Unix 0600/0700;
  - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  - named pipe vs Unix socket in channel logs;
  - ConPTY vs openpty in `pty` spans.
  - the server check's peer-pid read: Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there: the recorded gap, owned by "Unix endpoint and home hardening").

  No OS-specific exporter.
### obs-scope-summary#45.1 · values · triggers · ## 1. Obs Scope Summary · lines 464–464
- **Required telemetry signal:** A zero-unlogged-panics invariant for all four process roles. Each panic produces exactly one single-line `event:"panic"` record and no multi-line backtrace, and `hook` still exits 0. No formal latency or availability SLO beyond the tests perf gates.
### obs-scope-summary#47.1 · values · triggers · ## 1. Obs Scope Summary · lines 476–479
- **Required telemetry signal:**
  - Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  - `send-issued` and `send-refused` are logged events.
  - The tests field and event list is adopted verbatim.
### obs-scope-summary#48.1 · values · triggers · ## 1. Obs Scope Summary · lines 485–488
- **Required telemetry signal:**
  - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
  - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor). `viola link` / `viola unlink`'s strict-mode fail (their instance check of the driven instance) carries no logged `detail` either (no line): its output is the fixed instance-state `unable` / `hint:` pair on stderr, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, exit 21.
  - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for `link` / `unlink` the driven instance's file), distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).
### obs-scope-summary#49.1 · values · triggers · ## 1. Obs Scope Summary · lines 494–498
- **Required telemetry signal:**
  - Counters for unknown event kinds, unknown fields and torn lines (`skipped`).
  - The `RefusalReason` `unknown` fallback (exit 14).
  - Higher-`v` rejections (-32602; the code also carries `send`'s invalid params, `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`, so it is not a higher-`v` count alone).
  - Per-parser parse-failure counts: paste validation, channel framing at `MAX_FRAME`, hook stdin, `claude agents --json`, vt100 under `catch_unwind`.
