# Apply — obs-plan · part 4 of 6

## Verdicts
- fanout-obs-plan--cascade-g3.md 5 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:29+#1
- fanout-obs-plan--cascade-g3.md 6 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:20#1
- fanout-obs-plan--cascade-g3.md 7 · folded into cascade-compare-obs-plan.md 9
- fanout-obs-plan--cascade-g3.md 8 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:30#1
- fanout-obs-plan--cascade-g3.md 9 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:39#1
- fanout-obs-plan--cascade-g3.md 10 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:42#1
- fanout-obs-plan--cascade-g3.md 11 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:44#1
- fanout-obs-plan--cascade-g3.md 12 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:54#1
- fanout-obs-plan--cascade-g3.md 13 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:54#2
- fanout-obs-plan--cascade-g3.md 14 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:55#1
- fanout-obs-plan--cascade-g3.md 15 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:56#1
- fanout-obs-plan--cascade-g3.md 16 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:69#1
- fanout-obs-plan--cascade-g3.md 17 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:86#1, .andromeda/obs-plan-detail/span-trace-coverage.md:86+#1
- fanout-obs-plan--cascade-g3.md 18 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:102#1
- fanout-obs-plan--cascade-g3.md 20 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:113#1
- fanout-obs-plan--cascade-g3.md 21 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:122+#1
- fanout-obs-plan--cascade-g3.md 22 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:124+#1
- fanout-obs-plan--cascade-g3.md 23 · folded into fanout-obs-plan--cascade-g3.md 21
- fanout-obs-plan--cascade-g3.md 24 · folded into fanout-obs-plan--cascade-g3.md 25
- fanout-obs-plan--cascade-g3.md 25 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:192#1
- fanout-obs-plan--cascade-g3.md 26 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:217+#1
- cascade-compare-obs-plan.md 8 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:28#1
- cascade-compare-obs-plan.md 9 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:31#1

## Edits
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:20
  old: «(except the hidden capture arms, `hook <event> --capture` and `hook statusline --capture`: no obs init, no lines, §4 Edge flows)»
  new: «(except the hidden capture arms, `hook <event> --capture` and `hook statusline --capture`: no obs init, no lines, §4 Edge flows; and except a refused or absent instance on the event and dialog paths: the instance check, `canonical` then `trusted`, runs before `read_diagnostics_level` and obs init, and a refused check, or an instance that does not resolve (no valid `VIOLA_NAME`, or a `VIOLA_DIR` naming no existing instance directory), exits 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:28
  old: «(for `link` / `unlink`, their instance check of the driven instance, then the liveness pre-check)»
  new: «(for each of the eight, the instance check `client::refuse_untrusted` first, of the driven instance for `link` / `unlink`, then the liveness pre-check)»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:29
  anchor: «- The refusal's place is asserted by captured-span cases, one per CLI frame verb (`request_refuses_an_unverified_server_inside_its_dispatch_span` in each verb's module) and one for the hook (`hook_refuses_an_unverified_server_inside_hook_handle`), each through the shared test-only helper `cmd::client::refused_inside` (`assert_refused_inside`): the refusal happens inside its scenario's root span, and each case reads red with the check removed.»
  add: «- **cli (short-lived verbs), the instance check:** the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) run `client::refuse_untrusted`, the instance check `link` / `unlink` share, on their named instance before any role file opens; a refused check (exit 21, the instance pair on stderr, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`) opens no process log and writes no line.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:30
  old: «inside `hook.handle` (`read_snapshot`, then `connect_by`, then `verify_server`) before any frame.»
  new: «inside `hook.handle` (`read_snapshot_or_replay`, its span `state.snapshot_recover` under `hook.handle`, then `connect_by`, then `verify_server`) before any frame. Only a snapshot read whole goes on: a replayed snapshot (it carries no `endpoint`, `pid` or `started_at`) or an absent one is unreachable, so no frame is sent and the hook fails open with `hook-decision{detail:"channel-unreachable"}`, a SessionEnd taking the direct `append_session_end`.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:31
  old: «the frame tools `send`, `wait`, `last` and `answer` verify the wrapper after `TokioClient::connect`»
  new: «the frame tools `send`, `wait`, `last` and `answer` first run `check_instance` on their instance, before `live_endpoint` (a refusal returns `instance-unreachable` with `detail: null`, no frame and no line), then verify the wrapper after `TokioClient::connect`»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:39
  old: «`run.start` (INTERNAL, root) › `run.collision_check`»
  new: «`run.start` (INTERNAL, root) › `state.snapshot_recover` (the start's one snapshot read, `read_snapshot_or_replay`, which the collision check takes) › `run.collision_check`»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:42
  old: «(taken over, its `events.ndjson` appended to);»
  new: «(taken over, its `events.ndjson` appended to); over a replayed snapshot, which carries no pid or start time, the heartbeat alone decides (a beat no older than `STALE_AFTER` is `already-live`, an older one `free`), and an absent snapshot or a failed read is `free`;»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:44
  old: «`outcome` (`loaded|hash-mismatch|unreadable|load-failed|not-built`)»
  new: «`outcome` (`loaded|hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`; `strict-modes-failed` when the owner/DACL check refuses the `conpty/` dir or a companion, `PinError::StrictModes`, nothing written)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:54
  old: «(Founder Direction 4; no path or pid).»
  new: «(Founder Direction 4; no path or pid). Before that log opens, the start (`run`'s, and `viola revive`'s) runs the home-tree check `check_home_tree` (through `checked_exe` and `refuse_tree`) on the home (to the read rule), the instance's set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned exe, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`: a refusal exits 1 with the home pair or the instance pair on stderr (`viola revive --json`: its `strict-modes-failed` document), opens no log, writes no line and starts no child, so it carries no `detail`.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:54
  old: «then a preflight of its own whose refusals add»
  new: «then a preflight of its own, which makes one snapshot read and hands it to both its collision check and its recorded-directory reading (over an unreadable snapshot one replay, so one `state-recovered` line), whose refusals add»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:55
  old: «`sideload_fallback` (`hash-mismatch|unreadable|load-failed|not-built`, D-36)»
  new: «`sideload_fallback` (`hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`, D-36)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:56
  old: «closed in the `process-start` properties to `hash-mismatch|unreadable|load-failed|not-built`»
  new: «closed in the `process-start` properties to `hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:69
  old: «The server check runs before `channel.request`, inside `send.client` (CLI) or `mcp.tool_call` (MCP): the peer pid and its start time against the one snapshot read.»
  new: «The instance check comes first: CLI `send` runs `client::refuse_untrusted` after its text read and before any role file opens, and MCP `send` runs `check_instance` before `live_endpoint`; a refusal sends no frame and writes no line. The server check runs before `channel.request`, inside `send.client` (CLI) or `mcp.tool_call` (MCP): the peer pid and its start time against the one snapshot read, `live_endpoint`'s `read_snapshot_or_replay` (a replayed snapshot carries no `endpoint`, `pid` or `started_at`, so the path ends as unreachable).»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:86
  old: «(no live snapshot, or the connect failed)»
  new: «(no live snapshot, a replayed one included, or the connect failed)»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:86
  anchor: «- **Cleanup:** `run.confirm_window` closes on the confirmation its send waits for, or on window expiry. For an ordinary send that is the matching `prompt-submitted{origin:"driver"}`. For a listed local command with a new-session post-condition on a verified CLI (`/clear`) it is the `session-start` whose `cause` is `clear` and whose `agent_session_id` is a string that differs from the id the send slot remembers; that send waits for its post-condition only, and no `prompt-submitted` claims it. For a listed local command with a compacted-session post-condition on a verified CLI (`/compact`) it is the `session-start` whose `cause` is `compact` and whose `agent_session_id` is a string equal to the id the send slot remembered before that line (nothing remembered claims nothing), no `prompt-submitted` claiming it either; its window is `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL), every other confirmed send's `CONFIRM_WINDOW_FALLBACK` (10 s). Expiry is `send-refused{detail:"no-prompt-submitted"}` for all three (no new detail). An unconfirmable send (a listed command with no post-condition, or any listed command on an unverified CLI) opens no `run.confirm_window`. `channel-response` is emitted from a `Drop` guard in `channel.dispatch`, so an early return or error still logs it. `send.client` closes with `process-exit{subject:"self", exit_code}` (0 / 10 / 11 / 12 / 13 / 14 / 20 / 21, or 1 `internal-error` on a reply with neither a parseable `result` nor an `error.code`); exit 20 carries `detail:"wrapper-fault"`, and exit 21 `detail:"instance-dead"` with `during` `connect` (no live snapshot, or the connect failed) or `call` (the connection lost after it), or `detail:"server-verify-failed"` with `during:"connect"` (the serving process is not the recorded wrapper, or the check cannot be made; no frame sent; Linux and Windows, macOS on the recorded gap).»
  add: «  - A `send` its instance check refuses (`client::refuse_untrusted`: exit 21, the instance pair, or under `--json` the `instance-unreachable` document) is refused before any role file opens: no `send.client` span, no `process-exit` and no line.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:102
  old: «take the liveness-only pre-check and then the server check; `strict-modes-failed` cannot occur on them, the strict-modes half owned by the route entry "Home and code-bearing file integrity".»
  new: «first take the instance check (`client::refuse_untrusted`, before any role file opens), then the liveness-only pre-check and then the server check; a refused instance check exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and sends no frame and writes no line, and MCP `wait` / `last` refused by `check_instance` return `instance-unreachable` with no line.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:113
  old: «The server check comes before each frame of this path:»
  new: «The instance check comes first on both sides: the dialog hook runs `canonical`, then `trusted`, before obs init, and a refused or absent instance exits 0 at once with no line, no frame and no decision (fail open); CLI `answer` runs `client::refuse_untrusted` before any role file opens, and MCP `answer` runs `check_instance` before `live_endpoint`, each refusal with no frame and no line. The server check comes before each frame of this path:»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:122
  anchor: «  - A failed server check is one of those fail-open paths before `dialog_id`: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, with no `hook.dialog` frame and so no `dialog-raised`.»
  add: «  - A refused or absent instance (the instance check, `canonical` then `trusted`, made before obs init) is none of the fail-open paths before `dialog_id`: the hook exits 0 with empty streams, writes no `hook-invoked`, no `hook-decision` and no line, and sends no frame.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:124
  anchor: «  - a server-verification refusal on CLI `answer`, made before any frame: no `channel-request` / `channel-response` line, `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN; MCP `answer` writes no line.»
  add: «  - a refused instance check on CLI `answer` (`client::refuse_untrusted`, before any role file opens): exit 21 with the instance pair, or under `--json` the `instance-unreachable` document, no `channel-request`, no `process-exit` and no line; MCP `answer` refused by `check_instance` returns `instance-unreachable` with `detail: null` and writes no line.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:192
  old: «The three other snapshot readers (`run`'s collision check in `src/cmd/run.rs`, `src/cmd/hook.rs` once, its event and dialog paths sharing the one read, and `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules, as measured at viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md §Counts; the CLI frame verbs and `viola mcp` read through `viola_state::liveness::live_endpoint`, outside the rule's `src/`) still call `read_snapshot` and write no `state-recovered` line.»
  new: «Its other product callers are `run`'s start, whose collision check takes the start's one read (a start over an unreadable snapshot writes one `state-recovered{detail:"snapshot-replayed"}` line in `run-<name>.ndjson`), `src/cmd/hook.rs`'s `connect`, shared by the event and dialog paths (under `hook.handle`; a replayed snapshot is unreachable), `src/cmd/hook/statusline.rs`'s `serve` (a replayed snapshot carries no `statusline_command`, so nothing runs or prints) and `viola_state::liveness::live_endpoint`, read by the CLI frame verbs and `viola mcp`'s frame tools (a replayed snapshot is no live endpoint), each writing its one line per call; no product reader calls `read_snapshot` (the rule: `read_snapshot(` under `src/`, outside test modules, as measured at viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md §Changes).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:217
  anchor: «  - `plugin install` writes that pair only after its strict-modes tree check has passed (`process-exit` exit 0, no detail); a refused check opens no log and writes no line (exit 21: the stderr pair, or under `--json` the `instance-unreachable` document). Its outcome is the exit code (0 / 1 / 2 / 21) and, under `--json`, `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`; no path in either document or any line.»
  add: «  - `verify` runs the home-tree check `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log opens: a refused tree exits 1 with the home pair on stderr, no step line and no summary, and opens no log and writes no line. `update_stamps` runs `check_stamps` before `ledger/` is created and again under the lock, and on a refusal reads and writes nothing; `verify` maps that refusal (`StampsError::StrictModes`) to the same home pair.»

## Patterns
[[pattern]]
id = "obs4-01"
fixed = "for `link` / `unlink`, their instance check of the driven instance, then the liveness pre-check"

[[pattern]]
id = "obs4-02"
fixed = "(`read_snapshot`, then `connect_by`"

[[pattern]]
id = "obs4-03"
fixed = "loaded|hash-mismatch|unreadable|load-failed"

[[pattern]]
id = "obs4-04"
fixed = "`sideload_fallback` (`hash-mismatch|unreadable|load-failed|not-built`"

[[pattern]]
id = "obs4-05"
fixed = "properties to `hash-mismatch|unreadable|load-failed|not-built`"

[[pattern]]
id = "obs4-06"
fixed = "`strict-modes-failed` cannot occur on them"

[[pattern]]
id = "obs4-07"
fixed = "still call `read_snapshot` and write no `state-recovered` line"
