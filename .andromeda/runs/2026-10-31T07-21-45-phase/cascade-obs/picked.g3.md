### obs-scope-summary#26.1 · values · logs · ## 1. Obs Scope Summary · lines 285–293
- **Log file location:** Tests require `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`. That differs from two upstream statements:
  - arch Obs-Relevant Conventions (Diagnostics location): `<viola home>/instances/<ViolaName>/diagnostics/`;
  - security Vector 2 and Data Classifications: only `instances/<name>/diagnostics/` may hold user content.

  Obs aligns to tests (tests is upstream) on file basenames and on the `logs` wrapper `{"src":"diag","file":<basename>,"record":<line>}`. The directory root (home-level or instance-level) must be reconciled in Phase 3/5 with a Decisions Log entry. If `mcp.ndjson` sits at home level, it must never carry user content.

  `mcp` always writes `<home>/diagnostics/mcp.ndjson`, as tests assume (D-09; stderr JSON only when the file cannot be opened; landed at chunk 2026-10-17-mcp-server-for-drivers).

  Every file is 0600 inside a 0700 directory (Windows: home DACL), and the tests secret-scan test enforces it. The product holds it at open too: `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before anything is created or narrowed — a symlink refused, a folder held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows — and a refusal is a failed open (no role file opened; a detail line dropped). Retention is the lifetime of the session home (tests excerpt, `logs`); there is no rotation in v1 (security Vector 7).
### obs-scope-summary#27.2 · values · paths · ## 1. Obs Scope Summary · lines 300–300
- **Must-trace spans:** `run.start` › `state.snapshot_recover` › `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `run.budget_eval` › `state.snapshot_write` › `state.heartbeat_start` › `pty.spawn`. Log events: `process-start` (run), `process-start` for the child, and `process-exit` on failure. The product event order to verify in `events.ndjson` is `wheel{cause:"start"}` → `budget-gate` → `session-start`.
### obs-scope-summary#27.3 · values · paths · ## 1. Obs Scope Summary · lines 301–301
- **Required log fields:** `event`, `process="run"`, `instance`, `corr=null`, `cli_version`, `cli_verified`, `exit_code`. Exit-1 causes use a distinct `detail` code: already-live, squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, and for a start made by `viola revive` also no-session, session-live and cwd-missing. A revive whose instance check refuses writes no line: the check precedes the log (chunk 2026-10-10-budget-governor). A `run` or a revived start whose home-tree check (`check_home_tree`, through `checked_exe`) refuses writes no line either, so that exit-1 cause has no `detail` code: the check precedes the wrapper log, and the output is the home pair (a home-level refusal) or the instance pair (an instance-level one), revive `--json` the `strict-modes-failed` document. No path or pid appears in the detail.
### obs-scope-summary#28.2 · values · paths · ## 1. Obs Scope Summary · lines 309–309
- **Must-trace spans:** `send.client` (cli/mcp) › `channel.request` › `run.readiness_gate` (vt100) › `pty.paste_write` › `run.confirm_window` (not opened for an unconfirmable send). Log events: `channel-request`, then `send-issued`, then `hook-invoked` (UserPromptSubmit, or SessionStart for a `/clear` confirmed by its new session or a `/compact` confirmed by its same session (verified CLI); none awaited for an unconfirmable send), then `send-confirmed` or `send-refused`, then `channel-response`.
### obs-scope-summary#28.3 · values · paths · ## 1. Obs Scope Summary · lines 310–310
- **Required log fields:** `event`, `process`, `instance`, `corr` (the JSON-RPC `id` on channel lines; the send `cursor` on send lines), `from` (self-reported, not an identity), `confirmed`, `refusal`, `detail` (`input-not-ready` / `no-prompt-submitted` / `control-character` / `empty-text` / `turn-running`), `duration_ms`. Sent text never appears.
### obs-scope-summary#29.2 · values · paths · ## 1. Obs Scope Summary · lines 318–318
- **Must-trace spans:** `wait.block` (client) › `channel.request` › `run.wait_dispatch`. Log events: `channel-request` and `channel-response` with the outcome.
### obs-scope-summary#29.3 · values · paths · ## 1. Obs Scope Summary · lines 319–319
- **Required log fields:** `event`, `process`, `instance`, `corr` (JSON-RPC `id`), `after` (byte offset), `timeout_ms`, `outcome` (woken kind / `timed_out` / `instance-unreachable`), `exit_code` (21 when the wrapper is killed mid-wait), `duration_ms`. The `last_assistant_message` content never appears.
### obs-scope-summary#30.3 · values · paths · ## 1. Obs Scope Summary · lines 328–328
- **Required log fields:** `event`, `process`, `instance`, `corr` (`dialog_id`), `dialog_kind`, `refusal` / `detail` (`unknown-dialog` exit 13, `unverified-cli` exit 12, `server-verify-failed` on Linux and Windows: exit 21 on CLI `answer` with `during:"connect"`, fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint), `deadline_hit`, `duration_ms`. The question, answer and tool `input` text never appear. A refused instance check writes no line and so carries none of these fields: CLI `answer` (`refuse_untrusted`, before any frame) exits 21 with the instance pair, or the `instance-unreachable` document under `--json`; the dialog hook exits 0 at once with empty streams, no frame and no line.
### obs-scope-summary#31.2 · values · paths · ## 1. Obs Scope Summary · lines 336–336
- **Must-trace spans:** `run.wheel_transition` › `channel.request(send)` refused › `pause.client` / `release.client` › `channel.request(pause|release)`. Log events: `send-refused` and `channel-request` / `channel-response` for `pause` and `release`. `release-from-driver` is logged as its own event (Founder Direction 4), admitted by `schemas/diag-line.v1.json`.
### obs-scope-summary#32.2 · values · paths · ## 1. Obs Scope Summary · lines 345–345
- **Must-trace spans:** `hook.statusline` › `state.budget_write` › `statusline.shell_out`, then `run.budget_eval`. Log events: `hook-invoked`, `hook-decision` for statusline, and `send-refused` with `budget-paused`.
### obs-scope-summary#32.3 · values · paths · ## 1. Obs Scope Summary · lines 346–346
- **Required log fields:** `event`, `process`, `instance`, `corr`, `window` (`five-hour` / `seven-day`), `used_percentage`, `resets_at`, `paused`, `refusal="budget-paused"`, `exit_code` (11), `shell_exit_status`, `duration_ms`. The `statusline_command` string never appears outside `diagnostics/`.
### obs-scope-summary#33.1 · values · triggers · ## 1. Obs Scope Summary · lines 353–356
- **Required telemetry signal:**
  - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  - `-32603` is logged as the fixed "internal error".
  - `from` is logged as unauthenticated.
### obs-scope-summary#34.1 · values · triggers · ## 1. Obs Scope Summary · lines 362–364
- **Required telemetry signal:**
  - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
  - `send-refused` with detail `control-character` or `empty-text` is emitted by each deciding side, client and wrapper, without the text (tests excerpt, Coverage Triggers, Vector 2).
### obs-scope-summary#35.1 · values · triggers · ## 1. Obs Scope Summary · lines 370–374
- **Required telemetry signal:**
  - Fail-open paths are logged to `hook-<name>.ndjson` only: oversize stdin, malformed JSON, channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`. As landed a strict-modes failure is not logged: every `viola hook` arm but the two `--capture` arms — `hook statusline`, the event paths and the dialog paths — runs an instance check before obs init, and a refused check leaves no line (chunk 2026-10-10-budget-governor; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer, the operator's answer at its planning superseded 2026-10-13).
  - A server-verify mismatch (Linux and Windows) is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` — and no pid, start time, path or endpoint in the line; macOS compares no peer pid (the recorded gap).
  - Exit is 0 and stderr stays empty.
  - Drift reports go to `diagnostics/` only.
### obs-scope-summary#36.1 · values · triggers · ## 1. Obs Scope Summary · lines 380–380
- **Required telemetry signal:** Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names. One rejected call writes no line: a frame tool (`send`, `wait`, `last`, `answer`) whose instance check (`check_instance`, before `live_endpoint`) refuses returns `instance-unreachable` with `detail: null` and writes no frame.
### obs-scope-summary#37.1 · values · triggers · ## 1. Obs Scope Summary · lines 386–390
- **Required telemetry signal:**
  - A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
  - The R8 strip is logged as a count or variable-name list only, never values. Phase 3 must confirm names are admissible.
  - An anyhow chain containing a serde source becomes a fixed message before reaching stderr.
  - No env, flag or config may disable redaction.
### obs-scope-summary#38.1 · values · triggers · ## 1. Obs Scope Summary · lines 396–400
- **Required telemetry signal:**
  - `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), enforced by the tests secret-scan test.
  - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line in two places: the `ledger-stamps` read in `run`'s version gate, and on Windows x64 a refused `conpty/` companion (`PinError::StrictModes`), which `process-start` carries as `sideload_fallback` `strict-modes-failed`, the exit unchanged; a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`) leaves only verify's own `process-exit` (with an instance; exit 1, no `detail`). Every other strict-modes refusal leaves no line: `viola revive`'s instance check, `run`'s and a revived start's home-tree check (`check_home_tree`, exit 1, before the wrapper log opens), `viola verify`'s home-tree check (exit 1), the instance check of every `viola hook` arm but the two `--capture` arms (exit 0), the instance check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`refuse_untrusted`, exit 21), `viola link` / `viola unlink`'s instance check of the driven instance (made before any role file opens; exit 21), the `viola mcp` frame tools' instance check (`instance-unreachable`), the `diagnostics/` opens (`check_diagnostics`: a refusal is a failed open), the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list).
  - A pinned-exe hash mismatch is also reached when a start loses the pinned copy's no-clobber placement to a concurrent first start and the copy that stands re-hashes different (`place_absent`'s `PinError::HashMismatch`, `run.pin_copy` `pinned-hash-mismatch`, the file left as found); that path adds no line, field or `detail` code.
  - `skipped.torn_lines` counts.
### obs-scope-summary#39.1 · values · triggers · ## 1. Obs Scope Summary · lines 406–409
- **Required telemetry signal:**
  - `process-start` / `process-exit` for the `claude` child and for `claude agents --json` / `--version` probes.
  - Parse failures surface as `unknown` plus a counter.
  - C0/C1 controls are escaped in any human terminal rendering of logged rows.
### obs-scope-summary#41.1 · values · triggers · ## 1. Obs Scope Summary · lines 421–424
- **Required telemetry signal:**
  - `corr` = send `cursor` joins `send-issued` / `send-confirmed` / `send-refused` across CLI, MCP and wrapper.
  - `corr` = `dialog_id` joins hook → wrapper → driver.
  - The per-connection JSON-RPC `id` needs a discriminator for a merged-log join (section 2).
### obs-scope-summary#43.1 · values · triggers · ## 1. Obs Scope Summary · lines 440–447
- **Required telemetry signal:** Fault telemetry per class:
  - torn ndjson line healed, with file and offset;
  - corrupt or unsupported-`v` snapshot → replay recovery;
  - child exit detected on the process handle without EOF (ConPTY);
  - endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
  - stale heartbeat vs live pid.

  These need `event` values or an additive `detail` field (section 3, gap b).
### obs-scope-summary#44.1 · values · triggers · ## 1. Obs Scope Summary · lines 453–460
- **Required telemetry signal:** The same file sink and line schema on every OS. Platform differences must still yield identical fields:
  - permission application: Windows DACL vs Unix 0600/0700;
  - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
  - named pipe vs Unix socket in channel logs;
  - ConPTY vs openpty in `pty` spans.
  - the server check's peer-pid read: Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there: the recorded gap, owned by "Unix endpoint and home hardening").

  No OS-specific exporter.
### obs-scope-summary#47.1 · values · triggers · ## 1. Obs Scope Summary · lines 478–481
- **Required telemetry signal:**
  - Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
  - `send-issued` and `send-refused` are logged events.
  - The tests field and event list is adopted verbatim.
### obs-scope-summary#48.1 · values · triggers · ## 1. Obs Scope Summary · lines 487–490
- **Required telemetry signal:**
  - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
  - Each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, `.cmd`/`.bat` child, and `viola revive`'s no logged session and missing recorded directory) gets a distinct `detail` code with no path or pid. `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor). `viola link` / `viola unlink`'s strict-mode fail (their instance check of the driven instance) carries no logged `detail` either (no line): its output is the fixed instance-state `unable` / `hint:` pair on stderr, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, exit 21.
  - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for `link` / `unlink` the driven instance's file), distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).
### obs-scope-summary#49.1 · values · triggers · ## 1. Obs Scope Summary · lines 496–500
- **Required telemetry signal:**
  - Counters for unknown event kinds, unknown fields and torn lines (`skipped`).
  - The `RefusalReason` `unknown` fallback (exit 14).
  - Higher-`v` rejections (-32602; the code also carries `send`'s invalid params, `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`, so it is not a higher-`v` count alone).
  - Per-parser parse-failure counts: paste validation, channel framing at `MAX_FRAME`, the `viola mcp` stdin at `MAX_FRAME`, hook stdin, `claude agents --json`, vt100 under `catch_unwind`.
