### obs-scope-summary#29.3 · values · paths · ## 1. Obs Scope Summary · lines 318–318
- **Required log fields:** `event`, `process`, `instance`, `corr` (JSON-RPC `id`), `after` (byte offset), `timeout_ms`, `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
### obs-scope-summary#30.1 · values · paths · ## 1. Obs Scope Summary · lines 325–325
- **Surfaces involved:** cli (`hook` PreToolUse / PermissionRequest) → ipc-internal (`hook.dialog`) → cli (`run`) ← cli or ipc-internal (mcp) `answer`
### obs-scope-summary#30.2 · values · paths · ## 1. Obs Scope Summary · lines 326–326
- **Must-trace spans:** `hook.handle` › `channel.request(hook.dialog)` › `run.dialog_register` › `run.dialog_await` › `hook.decision_emit`; on the driver side, `answer.client` › `channel.request(answer)`. Log events: `hook-invoked`, `dialog-raised`, `dialog-answered`, `hook-decision` (decision null on deadline expiry).
### obs-scope-summary#30.3 · values · paths · ## 1. Obs Scope Summary · lines 327–327
- **Required log fields:** `event`, `process`, `instance`, `corr` (`dialog_id`), `dialog_kind`, `refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12), `deadline_hit`, `duration_ms`. The question, answer and tool `input` text never appear.
### obs-scope-summary#31.1 · values · paths · ## 1. Obs Scope Summary · lines 334–334
- **Surfaces involved:** cli (`run`, key input) + `events.ndjson` (`wheel`) → cli or ipc-internal (mcp) `send` refused → cli `release` → ipc-internal
### obs-scope-summary#31.2 · values · paths · ## 1. Obs Scope Summary · lines 335–335
- **Must-trace spans:** `run.wheel_transition` › `channel.request(send)` refused › `pause.client` / `release.client` › `channel.request(pause|release)`. Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`. `release-from-driver` is logged as its own event (Founder Direction 4), admitted by `schemas/diag-line.v1.json`.
### obs-scope-summary#33.1 · values · triggers · ## 1. Obs Scope Summary · lines 352–355
- **Required telemetry signal:**
  - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  - `-32603` is logged as the fixed "internal error".
  - `from` is logged as unauthenticated.
### obs-scope-summary#34.1 · values · triggers · ## 1. Obs Scope Summary · lines 361–363
- **Required telemetry signal:**
  - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  - `send-refused` with detail `control-character` or `empty-text` is emitted by each deciding side, client and wrapper, without the text (tests excerpt, Coverage Triggers, Vector 2).
### obs-scope-summary#35.1 · values · triggers · ## 1. Obs Scope Summary · lines 369–372
- **Required telemetry signal:**
  - Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`. As landed a strict-modes failure is not logged: `viola hook statusline` is the one arm that runs an instance check, and a refused check leaves no line (chunk 2026-10-10-budget-governor; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer, the operator's answer at its planning superseded 2026-10-13).
  - Exit is 0 and stderr stays empty.
  - Drift reports go to `diagnostics/` only.
### obs-scope-summary#36.1 · values · triggers · ## 1. Obs Scope Summary · lines 378–378
- **Required telemetry signal:** Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names.
### obs-scope-summary#37.1 · values · triggers · ## 1. Obs Scope Summary · lines 384–388
- **Required telemetry signal:**
  - A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
  - The R8 strip is logged as a count or variable-name list only, never values. Phase 3 must confirm names are admissible.
  - An anyhow chain containing a serde source becomes a fixed message before reaching stderr.
  - No env, flag or config may disable redaction.
### obs-scope-summary#38.1 · values · triggers · ## 1. Obs Scope Summary · lines 394–397
- **Required telemetry signal:**
  - `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), enforced by the tests secret-scan test.
  - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate. Four strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check, the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list).
  - `skipped.torn_lines` counts.
### obs-scope-summary#40.1 · values · triggers · ## 1. Obs Scope Summary · lines 412–412
- **Required telemetry signal:** The logger dependency is tracing-subscriber `>=0.3.20` (0.3.23 pinned by tests). Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
### obs-scope-summary#41.1 · values · triggers · ## 1. Obs Scope Summary · lines 418–421
- **Required telemetry signal:**
  - `corr` = send `cursor` joins `send-issued` / `send-confirmed` / `send-refused` across CLI, MCP and wrapper.
  - `corr` = `dialog_id` joins hook → wrapper → driver.
  - The per-connection JSON-RPC `id` needs a discriminator for a merged-log join (section 2).
### obs-scope-summary#42.1 · values · triggers · ## 1. Obs Scope Summary · lines 427–431
- **Required telemetry signal:**
  - `duration_ms` on `hook-decision`, or on `process-exit` for `process="hook"`, per hook event. The budget is `session-end` `max < 1.0 s`, and spine hooks and `pre-tool-use` on an unverified CLI must stay under the spine deadline (provisionally 1.0 s).
  - The heartbeat staleness flip happens at >5 s (4.9 s live / 5.1 s stale or gone).

  Aggregation into p95/p99 is done by the harness or `jq` over the ndjson, not by an in-process metrics pipeline (no listener, no exporter).
### obs-scope-summary#43.1 · values · triggers · ## 1. Obs Scope Summary · lines 437–444
- **Required telemetry signal:** Fault telemetry per class:
  - torn ndjson line healed, with file and offset;
  - corrupt or unsupported-`v` snapshot → replay recovery;
  - child exit detected on the process handle without EOF (ConPTY);
  - endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
  - stale heartbeat vs live pid.

  These need `event` values or an additive `detail` field (section 3, gap b).
### obs-scope-summary#44.1 · values · triggers · ## 1. Obs Scope Summary · lines 450–456
- **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  - permission application: Windows DACL vs Unix 0600/0700;
  - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  - named pipe vs Unix socket in channel logs;
  - ConPTY vs openpty in `pty` spans.

  No OS-specific exporter.
### obs-scope-summary#47.1 · values · triggers · ## 1. Obs Scope Summary · lines 474–477
- **Required telemetry signal:**
  - Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  - `send-issued` and `send-refused` are logged events.
  - The tests field and event list is adopted verbatim.
### obs-scope-summary#48.1 · values · triggers · ## 1. Obs Scope Summary · lines 483–485
- **Required telemetry signal:**
  - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
  - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor).
### obs-scope-summary#50.1 · values · triggers · ## 1. Obs Scope Summary · lines 501–504
- **Required telemetry signal:**
  - Events are emitted deterministically, keyed on hook events and `events.ndjson` offsets.
  - No telemetry is derived from screen content: vt100 is used for gate outcomes only.
  - No log-based waits on sleeps.
