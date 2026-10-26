## 2026-10-26-server-verification-before-any-frame — `server-verify-failed` written: the CLI exit-21 cause and the hook fail-open code
**Section:** §6 Log Coverage (log-coverage.md: the `process-exit` exit-21 line; the `hook-decision` fail-open codes) · §1 Obs Scope Summary (obs-scope-summary.md: the Dialog → `answer` path's required log fields; the Vector 4 trigger's fail-open signal; the Founder Direction 4 trigger's exit-21 / exit-1 causes) · §2 Telemetry Strategy (telemetry-strategy.md: the Vector 4 row)
**Change:**
- §6 exit 21: `server-verify-failed` is now written by the six CLI frame verbs on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows): `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, with no pid, start time, path or endpoint (was a code in the list only).
- §6 fail-open codes: `viola hook` writes `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson`; on the dialog path `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`.
- §1 Dialog → `answer` required fields: `refusal` / `detail` add `server-verify-failed` beside `unknown-dialog` (exit 13) and `unverified-cli` (exit 12): exit 21 on CLI `answer` with `during:"connect"`, fail-open on the dialog hook with `corr` null and `decision_emitted:false`.
- §1 Vector 4 trigger, a new item: a server-verify mismatch is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent, the dialog hook's two lines in that order, no pid, start time, path or endpoint; macOS compares no peer pid (the recorded gap).
- §1 Founder Direction 4 trigger, a new item: the server-verify fail is written by `send`, `wait`, `last`, `answer`, `pause` and `release`, distinct from `instance-dead`; macOS writes none.
- §2 Vector 4 row: the "server-verify mismatch" item writes one `hook-decision{detail:"server-verify-failed"}` at WARN on Linux and Windows (macOS the recorded gap).
**Why:** `server-verify-failed` was already a member of `diag-line.v1.json`'s `process-exit` and `hook-decision` `detail` enums, and this build is the first that writes it; no line carries a pid, start time, path or endpoint (asserted), and no config key, `--json` code, exit code or `RefusalReason` is added. macOS reports no peer pid and stays on the recorded gap.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the `wait` outcome count leaves out a refusal made before any frame
**Section:** §5 Metric Coverage (metric-coverage.md: a new values item ahead of the canonical quantile snippet)
**Change:** a new item: `viola.wait.outcome` counts only the `wait` calls answered by a `channel-response`, and a `wait` refused before any frame writes none. The CLI's server-verification refusal (Linux and Windows) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, counted by the Counter row's `detail` grouping; MCP's `wait` tool writes no line for it.
**Why:** `wait` now verifies the server inside `wait.block` before its frame; a refusal sends no frame, so it opens no `channel.request` span and writes no `channel-response` line (the operator's must-trace check, live, confirmed), and MCP writes no line for the mismatch, as on its unreachable path.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server check's call sites and its per-OS peer-pid read
**Section:** §1 Obs Scope Summary (obs-scope-summary.md: the OS-APIs entity's reason; the multi-platform trigger's OS differences)
**Change:**
- OS-APIs entity: the instrumented call sites add the server check's peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line (was bind, connect, lock, rename, kill-fallback use).
- Multi-platform trigger, a new OS difference: the server check's peer-pid read, Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there: the recorded gap, owned by "Unix endpoint and home hardening"). Still no OS-specific exporter.
**Why:** `Client::server_pid` reads `peer_creds().pid()` (interprocess 2.4.4): Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the client end, macOS none; `server_verdict` returns `Matches` with no comparison when no peer pid is reported, and `verify_server` reads the start time through `process_start_time`. The OS-APIs reason is a verbatim upstream copy, kept current.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-identity negatives' no-number scan
**Section:** §8 PII Scrubbing & Compliance (pii-scrubbing-compliance.md: the verification list)
**Change:** a new item: a server-verification refusal carries no pid, start time, path or endpoint; `tests/security_negatives_server_identity.rs` scans every output and the `cli-<name>.ndjson` line with `digit_runs` (whole numbers only, itself tested by `digit_runs_reads_whole_numbers_only`), on Linux and Windows (the file compiles out on macOS).
**Why:** the chunk's new test binary `security_negatives_server_identity` (`#![cfg(not(target_os = "macos"))]`) asserts the obs criterion — no pid, start time, path or endpoint in any output or line — met, `digit_runs` scanning every output.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server check inside the timed hook path
**Section:** §10 SLO Invariants & Telemetry Budgets (slo-invariants-telemetry-budgets.md: the spine hooks / `pre-tool-use` budget row)
**Change:** the Budget assertion cell adds: the timed path holds the server check a frame-writing hook makes before its first frame (the connected endpoint's peer pid and its OS start time against the one `snapshot.json` read; macOS the recorded gap, no comparison), bounded by the same gate, and a failed check fails open (`hook-decision{detail:"server-verify-failed"}`, exit 0). The `max <` spine-deadline assertion (provisionally 1.0 s) is unchanged.
**Why:** `run --perf` reads every hook row `max < SPINE_DEADLINE` with zero panics and the check in the timed path (hook rows max 2.2–2.4 ms over 30 runs each, the bound 1 s); `src/cmd/hook.rs`'s one shared `connect` (`read_snapshot`, `connect_by`, `verify_server`) serves the event and dialog paths.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the pre-frame server check's place in the span tree
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: the surface table's cli row; a new spans values block; the must-trace lists of the Scenarios Confirmed `send`, `wait` / `last`, Dialog → `answer` and Human takes the wheel)
**Change:**
- cli row: the dispatch spans name `last.client` (was absent beside `send.client`, `wait.block`, `answer.client`, `pause.client`, `release.client`).
- New block: the six CLI frame verbs verify the wrapper through `cmd::client::connect` inside their dispatch span, after their own client-side refusals and before any frame (the peer pid and its OS start time against the one snapshot read, a closed `ServerVerdict` `Matches|Mismatch|Unverifiable`); `viola hook`'s event and dialog paths share one `connect` inside `hook.handle` (`read_snapshot`, `connect_by`, `verify_server`), a SessionEnd whose check fails taking the direct `append_session_end`; `viola mcp`'s `send`, `wait`, `last` and `answer` verify after `TokioClient::connect` inside `mcp.tool_call` (`server_verified`, the start time read on `tokio::task::spawn_blocking`). A mismatch or a check that cannot be made opens no `channel.request` and writes no `channel-request` line. Linux and Windows; macOS on the recorded gap.
- The refusal's place is asserted by captured-span cases, one per CLI frame verb (`request_refuses_an_unverified_server_inside_its_dispatch_span`) and one for the hook (`hook_refuses_an_unverified_server_inside_hook_handle`), through the test-only `cmd::client::refused_inside` (`assert_refused_inside`), each red with the check removed.
- Each scenario's must-trace list places the check before its `channel.request`: inside `send.client` / `mcp.tool_call`, `wait.block` / `last.client`, `hook.handle` (before `hook.dialog`) and `answer.client`, `pause.client` / `release.client`.
**Why:** each verb's `request` now goes through `client::connect` inside its dispatch span; the seven captured-span cases and their helper were added beyond the plan on the operator's check, live, each read red with the check removed; that must-trace check confirmed no `channel.request` span or `channel-request` line on a refusal, since no frame is sent.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-verification refusal's lines in the scenarios
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: Scenario Confirmed `send` log fields and Cleanup; Scenario `wait` / `last` Cleanup; Scenario Dialog → `answer` log fields; Scenario Human takes the wheel log fields)
**Change:**
- Confirmed `send`: a client-side server-verification refusal, after the client's own refusals and before any frame, writes no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN, MCP `send` writes no line. Cleanup: exit 21 adds `detail:"server-verify-failed"` with `during:"connect"` (no frame sent) beside `instance-dead` (`connect` | `call`).
- `wait` / `last` Cleanup: `wait.block` also closes on a server-verification refusal made before any frame, the client's `process-exit` its outcome line (was the response or the connection error only).
- Dialog → `answer`: a failed server check is a fail-open path before `dialog_id`, `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, with no `hook.dialog` frame and so no `dialog-raised`; CLI `answer`'s refusal writes no channel line and the same exit-21 `process-exit` at WARN; MCP `answer` writes no line.
- Human takes the wheel: a `pause` / `release` refusal writes no channel line, the client's exit-21 `process-exit` at WARN.
**Why:** this build first writes `server-verify-failed` on `process-exit` and `hook-decision`, the dialog path's two lines in that order; MCP writes no line for the mismatch; no frame is sent, so neither side writes a channel line.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — `wait` / `last`: «`server-verify-failed` cannot occur» retired
**Section:** §4 Span / Trace Coverage (obs-plan.md: Scenario `wait` / `last`, the broad line; span-trace-coverage.md: the same scenario's required log fields)
**Change:**
- Was «`strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands», owned by "Server verification before any frame" and "Home and code-bearing file integrity" (the second claim of «2026-10-04-wait-and-last — wait channel lines scoped; wait/last exit 21 is instance-dead only»); now on Linux and Windows a serving process that is not the recorded wrapper, or a check that cannot be made, ends a CLI `wait` / `last` before any frame with one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN inside the dispatch span and no `channel-request` line, and MCP's returns `instance-unreachable` with no line; macOS compares no peer pid (the recorded gap).
- `wait` / `last`, like `send`, take the liveness-only pre-check and then the server check; `strict-modes-failed` cannot occur on them, its check owed to "Home and code-bearing file integrity".
**Why:** the sentence is disproved for `server-verify-failed` on Linux and Windows — `tests/security_negatives_server_identity.rs` reads it on `wait` and `last` — and the server-identity half of their dated gap is retired; the strict-modes half stands.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — E5: the other snapshot readers, five to three
**Section:** §4 Span / Trace Coverage (span-trace-coverage.md: Edge flows E5, the other snapshot readers)
**Change:** was five other snapshot readers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`), per «2026-10-10-statusline-pass-through — the statusline arm's lines as landed, its check before obs init, two start steps with no span»; now three: `run`'s collision check in `src/cmd/run.rs`, `src/cmd/hook.rs` once (its event and dialog paths sharing the one read) and `src/cmd/hook/statusline.rs` — the rule `read_snapshot(` under `src/`, outside test modules, as measured at the chunk's report; the CLI frame verbs and `viola mcp` read through `viola_state::liveness::live_endpoint`, outside the rule's `src/`.
**Why:** the count measured three product callers: `src/cmd/client.rs` has held no `read_snapshot(` call since chunk 2026-10-17-mcp-server-for-drivers moved the CLI read into `live_endpoint`, and this chunk joined `src/cmd/hook.rs`'s two reads in one `connect`; the plan's «four» is superseded by the measured three — a count carried from a plan is re-measured at the wrap, never copied.
**Kept:** the readers still call `read_snapshot` and write no `state-recovered` line: fork 1's option 1, the founder's own ruling, given by dialog at his wake 2026-10-26, relayed by the overseer; their move onto `read_snapshot_or_replay` is not this entry's.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — MCP's `instance-unreachable` reject writes no line
**Section:** §7 Error Capture & Reporting (obs-plan.md: Error classes captured, the MCP rejects bullet)
**Change:** was "MCP rejects are `isError` codes only, mirrored by codes-only log lines"; now a logged reject's line is codes-only, and `instance-unreachable` — the liveness pre-check's, or a frame tool's server-verification mismatch on Linux and Windows — is logged by no line, its outcome kept inside `mcp.tool_call`, the `isError` code its only record.
**Why:** `frame` verifies after `TokioClient::connect` through `server_verified`, and a mismatch returns `result::unreachable()` with no frame and no new log line, as the unreachable path does (no line by plan: the closed `event` enum); the old wording read as if every MCP reject had a line.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/
