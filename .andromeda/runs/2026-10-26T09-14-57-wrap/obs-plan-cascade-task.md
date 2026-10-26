# Task — obs-plan · 2026-10-26-server-verification-before-any-frame

## The session
Every client that writes a channel frame now checks the serving process first. That covers the six CLI verbs `send`, `wait`, `last`, `answer`, `pause` and `release`, `viola hook` (event and dialog), and `viola mcp`'s four frame tools. The check compares the peer pid and the OS start time against the one `snapshot.json` read, before the first frame.
A mismatch, or a check that cannot be made, is exit 21 `instance-unreachable` on the CLI, a silent fail-open on the hook, and an `isError` `instance-unreachable` on MCP. No frame is sent.
This build is the first to write `server-verify-failed`, as a `process-exit` line in `cli-<name>.ndjson` and as a `hook-decision` line in `hook-<name>.ndjson`. The code was already in `schemas/diag-line.v1.json`, so no event value, schema, config key, exit code or `--json` code was added.
Retired: the server-identity half of the dated gaps for `hook.event`, `send`, `wait` / `last`, `answer` + `hook.dialog`, `pause` / `release` and the MCP frame tools, on Linux and Windows. Their strict-modes half stands until "Home and code-bearing file integrity". macOS stays a recorded gap because it reports no peer pid.
Measured: there are three product `read_snapshot(` callers, not five: `src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142` and `src/cmd/run.rs:379`. The hook's two reads became one. The hook perf rows, which now include the check, read max 2.2–2.4 ms.
Re-pinned, not done: moving the snapshot readers onto `read_snapshot_or_replay` now belongs to "Home and code-bearing file integrity". This is fork 1, held PROVISIONAL until the founder gives his own word.

## Reached files
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/lib.rs
- crates/viola-state/src/liveness.rs
- src/cmd/client.rs
- src/human/hints.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/contract_fake_agent_drift.rs
- tests/security_negatives_server_identity.rs
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- §4 Scenario `wait` / `last` and its detail: the sentence that `strict-modes-failed` / `server-verify-failed` cannot occur on `wait` / `last` until client-side server verification lands is now false for `server-verify-failed` on Linux and Windows. `strict-modes-failed` still cannot occur there. The owner "Server verification before any frame" is closed, so only "Home and code-bearing file integrity" remains.
- §4 E5 detail, the snapshot-readers list: it names five product callers, including `src/cmd/client.rs` and `src/cmd/hook.rs` twice. It should now read three: `src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142` and `src/cmd/run.rs:379`. `src/cmd/client.rs` holds no `read_snapshot(` call. The body's E5 line, that the other snapshot readers write no `state-recovered` line, still holds.
- §4 E5 detail, the owner of the readers' move onto `read_snapshot_or_replay`: it is now "Home and code-bearing file integrity", held PROVISIONAL on fork 1, no longer "Server verification before any frame".
- §6 `detail` code catalog / log-details: the `server-verify-failed` entries under `process-exit` and `hook-decision` are still true, but they are now written by this build. A written-by-this-build qualifier is a candidate.
- §6 log levels: the new `process-exit` exit 21 line and the fail-open `hook-decision` line are both at WARN, which agrees with the table. Re-read only.
- §4 Scenario Confirmed `send`: the CLI `send` now verifies the server inside `send.client` before any `channel.request`. On a mismatch no `channel.request` span opens and no `channel-request` line is written.
- §4 Scenario Dialog → `answer`: CLI `answer` verifies inside `answer.client`. On a mismatch the dialog hook writes `hook-invoked{corr:null}` and then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, with no `hook.dialog` response. This reaches the rule that `hook-invoked` is written only after the `hook.dialog` response (D-07 corr timing).
- §4 Scenario Human takes the wheel: `pause` / `release` now verify inside `pause.client` / `release.client` before their frame.
- §4 Scenario `wait` / `last`: the check runs inside `wait.block` / `last.client`. The client's own exit-13 refusals still come first.
- §1 triggers, Vector 4 (hook stdin): this adds one fail-open cause, a server that is not the recorded wrapper. It is logged to `hook-<name>.ndjson` only, with exit 0, empty streams and no frame.
- §1 triggers, Vector 5 (MCP): a mismatch in the frame tools gives an `isError` with codes only (`instance-unreachable`) and writes no log line, the same as MCP's unreachable path.
- §1 triggers, Founder Direction 4 (a distinct `detail` per exit-21 cause): `server-verify-failed` is now a written exit-21 cause, distinct from `instance-dead`, with no path or pid.
- §1 scope entity `viola-state` (sysinfo liveness): `liveness` now also carries the server verdict, `ServerVerdict` and `verify_server`. `live_endpoint` returns a `LiveEndpoint` struct (endpoint, pid, started_at) from one snapshot read.
- §1 scope entity `viola-channel`: `Client::server_pid` and `TokioClient::server_pid` read the peer pid. They write and log nothing, and on macOS they return none.
- §7 module-boundary errors: `CallError::ServerUnverified` / `Unreachable::ServerUnverified` end in a logged `process-exit{detail:"server-verify-failed", during:"connect"}`, codes only.
- §8 PII: home-level lines still hold no pid, endpoint or path. The new lines hold no pid, start time, path or endpoint, and this is asserted.
- §10 performance budgets: the hook rows' timed path now includes the server check, at max 2.2–2.4 ms over 30 runs, up from 1.23–1.6 ms before the check landed. The spine bound of 1.0 s still holds. The budget value is unchanged.
- §10 zero-unlogged-panics and the §9 G2 / G4 scope (indirect): the new test binary `security_negatives_server_identity` and the impostor rows in `hook_fail_open` add homes and lines that G2, G4 and the secret scan cover.
- Any obs text that counts `hook_fail_open` cases (indirect, through the matrix): `hook_fails_open_silently_within_the_spine_bound` gains `case::impostor` (12 rows off macOS). `hook_dialog_tier_fails_open_silently` gains `question_impostor` and `permission_impostor`.
- Any obs text naming the dated exceptions as unverified frame writers (indirect, through the retired gaps): their server-identity half is closed on Linux and Windows. Their strict-modes half and the macOS gap stand.

## New facts
- F1 · span-trace-coverage · The six CLI frame verbs verify the wrapper through `cmd::client::connect` (peer pid plus start time against the one snapshot read, `ServerVerdict`) inside their dispatch spans `send.client`, `wait.block`, `last.client`, `answer.client`, `pause.client` and `release.client`, before any frame. A mismatch opens no `channel.request` span and writes no `channel-request` line.
- F2 · log-coverage · On a mismatch or an unmakeable check, this build writes `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`.
- F3 · log-coverage · On a mismatch, `viola hook` writes `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson`. On the dialog path the order is `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`.
- F4 · span-trace-coverage · The hook's event and dialog paths share one `connect` (`read_snapshot`, then `connect_by`, then `verify_server`). A SessionEnd whose check fails takes the direct `append_session_end`.
- F5 · span-trace-coverage · `viola mcp`'s `send`, `wait`, `last` and `answer` tools verify after `TokioClient::connect`, reading the start time on `spawn_blocking`, inside `mcp.tool_call`. A mismatch returns `instance-unreachable` with no frame and no log line.
- F6 · span-trace-coverage · There are seven captured-span cases, one per CLI verb plus one hook. Each asserts that the server refusal happens inside its scenario's root span, through the shared test helper `cmd::client::refused_inside`.
- F7 · pii-scrubbing-compliance · `tests/security_negatives_server_identity.rs` scans every output and the `cli-<name>.ndjson` line with `digit_runs`, asserting that no pid, start time, path or endpoint appears in a server-verification refusal.
- F8 · obs-scope-summary · On macOS the peer pid is not reported (`SERVER_PID_REPORTED` false), so the verdict is `Matches` with no comparison: the recorded gap. The server-identity negatives compile out on macOS, and the macOS cases are owed to "Unix endpoint and home hardening".

## Untouched
- The span naming convention and the `<area>` list: no span name or area was added or renamed.
- The closed `event` enum and `schemas/diag-line.v1.json` / `schemas/diag-detail.v1.json`: no event value, field or schema edit.
- Sink configuration, `diagnostics/` file locations, 0600/0700 modes and the absence of rotation: not changed.
- Panic hooks, catch sites, the `internal-error` exit path and the G3 panic-strategy rule: not reached.
- §9 CI step order, gate commands G1–G4, artifact uploads and the secret-scan step: no `scripts/` or `.github/` file changed.
- §4 Scenario `viola run` start sequence: `run`'s collision check still reads `read_snapshot` and verifies no server. The chunk did not change it.
- §4 Scenario Budget governor, the `hook statusline` shell-out and `budget.json`: not reached.
- E1 (unwrapped hook no-op), E2 (R8 strip) and E3 (implicit link): not changed.
- E5 replay semantics: `read_snapshot_or_replay` and its `state-recovered` lines are unchanged, and no reader moved onto it.
- The capability ledger read in `run.version_gate` and the Vector 7 rule that a strict-modes failure has a line only on the ledger-stamps read: the strict-modes half was not built.
- Supply-chain and Tokio-free rules: no dependency added or bumped, and `viola-channel`'s normal graph stays tokio-free.
- §5 metric naming and label cardinality: no metric or grouping key added.
- §7 error-reporter platform choice: unchanged.
