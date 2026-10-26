### obs-scope-summary#31.3 · values · paths · ## 1. Obs Scope Summary · lines 336–336
- **Required log fields:** `event`, `process`, `instance`, `corr`, `refusal="human-typing"`, `detail` (null / `manual-pause`), `wheel` holder, `exit_code` (10). Typed keys never appear.
### obs-scope-summary#31.4 · values · paths · ## 1. Obs Scope Summary · lines 337–337
- **Source:** tests excerpt, Critical Path 5; arch hint 5; creator must-work "Wheel take-over" and "R2 One wheel"
### obs-scope-summary#32.1 · values · paths · ## 1. Obs Scope Summary · lines 343–343
- **Surfaces involved:** cli (`hook statusline`, shell-out) → persistent store (`budget.json`) → cli (`run`, gate evaluation at start, on `send` and at `turn-ended`) → `events.ndjson` (`budget-gate`)
### obs-scope-summary#32.2 · values · paths · ## 1. Obs Scope Summary · lines 344–344
- **Must-trace spans:** `hook.statusline` › `state.budget_write` › `statusline.shell_out`, then `run.budget_eval`. Log events: `hook-invoked`, `hook-decision` for statusline, and `send-refused` with `budget-paused`.
### obs-scope-summary#32.3 · values · paths · ## 1. Obs Scope Summary · lines 345–345
- **Required log fields:** `event`, `process`, `instance`, `corr`, `window` (`five-hour` / `seven-day`), `used_percentage`, `resets_at`, `paused`, `refusal="budget-paused"`, `exit_code` (11), `shell_exit_status`, `duration_ms`. The `statusline_command` string never appears outside `diagnostics/`.
### obs-scope-summary#32.4 · values · paths · ## 1. Obs Scope Summary · lines 346–346
- **Source:** tests excerpt, Critical Path 6; arch hint 6; creator must-work "Budget governor"; brief §7 policy
### obs-scope-summary#33.1 · values · triggers · ## 1. Obs Scope Summary · lines 352–355
- **Required telemetry signal:**
  - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  - `-32603` is logged as the fixed "internal error".
  - `from` is logged as unauthenticated.
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
  - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate. Four strict-modes refusals leave no line: `viola revive`'s instance check, `viola hook statusline`'s instance check, the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list).
  - `skipped.torn_lines` counts.
### obs-scope-summary#39.1 · values · triggers · ## 1. Obs Scope Summary · lines 404–407
- **Required telemetry signal:**
  - `process-start` / `process-exit` for the `claude` child and for `claude agents --json` / `--version` probes.
  - Parse failures surface as `unknown` plus a counter.
  - C0/C1 controls are escaped in any human terminal rendering of logged rows.
### obs-scope-summary#40.1 · values · triggers · ## 1. Obs Scope Summary · lines 413–413
- **Required telemetry signal:** The logger dependency is tracing-subscriber `>=0.3.20` (0.3.23 pinned by tests). Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
### obs-scope-summary#41.1 · values · triggers · ## 1. Obs Scope Summary · lines 419–422
- **Required telemetry signal:**
  - `corr` = send `cursor` joins `send-issued` / `send-confirmed` / `send-refused` across CLI, MCP and wrapper.
  - `corr` = `dialog_id` joins hook → wrapper → driver.
  - The per-connection JSON-RPC `id` needs a discriminator for a merged-log join (section 2).
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
### obs-scope-summary#46.1 · values · triggers · ## 1. Obs Scope Summary · lines 470–470
- **Required telemetry signal:** No rotation of `events.ndjson` or `diagnostics/*.ndjson` in v1. Any future rotation must keep 0600/0700 permissions and keep `wait after` and `send cursor` offsets valid. The trigger is operational, not compliance: security tier is Minimal and there are no compliance triggers.
### obs-scope-summary#47.1 · values · triggers · ## 1. Obs Scope Summary · lines 476–479
- **Required telemetry signal:**
  - Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  - `send-issued` and `send-refused` are logged events.
  - The tests field and event list is adopted verbatim.
### obs-scope-summary#48.1 · values · triggers · ## 1. Obs Scope Summary · lines 485–488
- **Required telemetry signal:**
  - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
  - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor).
  - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause` and `release`, distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).
### obs-scope-summary#49.1 · values · triggers · ## 1. Obs Scope Summary · lines 494–498
- **Required telemetry signal:**
  - Counters for unknown event kinds, unknown fields and torn lines (`skipped`).
  - The `RefusalReason` `unknown` fallback (exit 14).
  - Higher-`v` rejections (-32602; the code also carries `send`'s invalid params and `release` carrying `from`, so it is not a higher-`v` count alone).
  - Per-parser parse-failure counts: paste validation, channel framing at `MAX_FRAME`, hook stdin, `claude agents --json`, vt100 under `catch_unwind`.
### obs-scope-summary#50.1 · values · triggers · ## 1. Obs Scope Summary · lines 504–507
- **Required telemetry signal:**
  - Events are emitted deterministically, keyed on hook events and `events.ndjson` offsets.
  - No telemetry is derived from screen content: vt100 is used for gate outcomes only.
  - No log-based waits on sleeps.
