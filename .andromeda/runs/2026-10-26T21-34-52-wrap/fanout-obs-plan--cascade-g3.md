proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 20
    section: '## 2. Telemetry Strategy'
    change: 'Extend the bounded exemptions after «or a refused instance check on `viola link` / `viola unlink`, made before their role file opens» with: «, a refused or absent instance on `viola hook`s event and dialog paths (a `VIOLA_DIR` naming no existing instance directory included), a refused instance check on the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, through `client::refuse_untrusted`), and a refused home tree on `viola run`, `viola revive`s start or `viola verify` (`check_home_tree`), each made before its role file opens; a `diagnostics/` path refused by `check_diagnostics` is a failed step-4 role-file open»; the rest unchanged.'
    sidecar: 'telemetry-strategy panic exemptions: the new pre-role-file strict-modes checks (hook event/dialog, six frame verbs, run / revive start and verify home tree) join the bounded list (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    rationale: 'Report §Changes Symbols: refuse_untrusted opens nothing under the home and writes no line (client.rs 20-41); hook.rs canonical / trusted run before read_diagnostics_level / viola_obs_init; run.rs 166-172, revive.rs 125-135 and verify.rs 64-70 refuse before their log; obs.rs check_diagnostics is a failed open; founder no-line ruling inputs#I3 Q2.'
    basis: .andromeda/obs-plan-detail/telemetry-strategy.md:20
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 30
    section: '## 2. Telemetry Strategy'
    change: 'Extend the «strict-modes failure» item: after «`viola hook statusline`s refused instance check leaves no trace, asserted by absence (chunk 2026-10-10-budget-governor)» add «; so does the event and dialog paths refused or absent instance (`canonical` then `trusted`, before `read_diagnostics_level` and obs init; a `VIOLA_DIR` naming no existing instance directory included): exit 0 at once, no `config.json` read, no role file, no stdin read, no line, no frame, no SessionEnd append (chunk 2026-10-26-home-and-code-bearing-file-integrity)»; the server-verify item unchanged.'
    sidecar: 'telemetry-strategy Vector 4: the hook event and dialog paths refused instance check leaves no trace, beside statusline.'
    rationale: 'Report §Changes Symbols src/cmd/hook.rs: canonical (75-81) and trusted (83-86) shared by both arms; a refused or absent instance returns exit 0 at once with no line; Retired (1) and (6); Deviations 5.'
    basis: .andromeda/obs-plan-detail/telemetry-strategy.md:30
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 33
    section: '## 2. Telemetry Strategy'
    change: 'Append to the Vector 7 mapping: «, §4 run-start `process-start.sideload_fallback` `strict-modes-failed` (a `conpty/` companion its owner/DACL check refuses, `PinError::StrictModes`; the child on the inbox ConPTY, exit unchanged); the strict-modes checks made before every read or open (the six frame verbs, the hook event and dialog paths, the `viola mcp` frame tools, `verify`, `run` / `revive`s start, the `diagnostics/` opens) write no line by the founder ruling (§10 exemptions)».'
    sidecar: 'telemetry-strategy Vector 7: sideload_fallback strict-modes-failed and the no-line strict-modes refusals mapped.'
    rationale: 'Report §Changes Schema / config (diag-line.v1.json line 70 gains strict-modes-failed), Symbols pin.rs pin_companions and run.rs:510, Coverage of new surfaces (instrumentation n/a, no line, inputs#I3 Q2); Counts 7, 20.'
    basis: .andromeda/obs-plan-detail/telemetry-strategy.md:33
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 43
    section: '## 2. Telemetry Strategy'
    change: 'Append to the Founder Direction 4 mapping: «; a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`s start and `verify`, exit 1) writes no line and so carries no `detail` code, by the founder own answer (chunk 2026-10-26-home-and-code-bearing-file-integrity, `inputs#I3` Q2); its machine-readable form is the exit code, and under `--json` the closed `instance-unreachable` document (frame verbs) or `strict-modes-failed` (revive)».'
    sidecar: 'telemetry-strategy FD4 row: the no-line strict-modes refusals carry no detail code, the founder ruling named.'
    rationale: 'Report §Changes Symbols client.rs refuse_untrusted (Some(21), refused_document instance-unreachable), run.rs refuse_tree and verify.rs refuse_home (exit 1, no log); revive --json keeps strict-modes-failed; Reverted / negative facts: logging strict-modes-failed after a refused check was rejected by the founder (inputs#I3 Q2).'
    basis: .andromeda/obs-plan-detail/telemetry-strategy.md:43
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 18
    section: '## 4. Span / Trace Coverage'
    change: 'In the cli row, after the `viola link` / `viola unlink` sentence add: «The six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) first run `client::refuse_untrusted` (the instance check of the named instance, the same check `link` / `unlink` share) before any role file opens; a refused check (exit 21, the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`) opens no process log and writes no line. `viola verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log; a refused tree (exit 1, the home pair) opens no process log and writes no line.»'
    sidecar: 'span-trace-coverage cli row: the six frame verbs and verify refuse a widened home before any role file, with no line.'
    rationale: 'Report §Changes Symbols client.rs 20-41 (called first by send, wait, last, answer, pause, release; link_call moved onto it unchanged) and verify.rs 64-70 / 95-99; founder inputs#I3 Q2 and Q3; Counts 8, 20.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:18
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 20
    section: '## 4. Span / Trace Coverage'
    change: 'In the `viola hook` row, widen the exception to «`hook-invoked` / `hook-decision` (except the hidden capture arms, `hook <event> --capture` and `hook statusline --capture`: no obs init, no lines, §4 Edge flows; and except a refused or absent instance on the event and dialog paths: `canonical` then `trusted` run before `read_diagnostics_level` and obs init, and a refused check, no `VIOLA_NAME` or a `VIOLA_DIR` naming no existing instance directory returns exit 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append)»; the span chain, `catch_unwind` and the sink fallback unchanged.'
    sidecar: 'span-trace-coverage hook row: a refused or absent instance writes no hook-invoked / hook-decision line.'
    rationale: 'Report §Changes Symbols src/cmd/hook.rs (canonical 75-81, trusted 83-86, before read_diagnostics_level / viola_obs_init; capture arms ahead unchanged); Deviations 5; Retired (1), (6).'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:20
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 22
    section: '## 4. Span / Trace Coverage'
    change: 'In the viola-mcp row, after «Codes-only `send-refused` / `channel-response` on rejects.» add: «A frame tool (`send`, `wait`, `last`, `answer`) runs `check_instance` on its instance before `live_endpoint`; a refusal returns `instance-unreachable` with `detail: null`, no frame and no line.»'
    sidecar: 'span-trace-coverage mcp row: a frame tool refused by check_instance writes no line.'
    rationale: 'Report §Changes Symbols viola-mcp tools.rs frame (337-339): check_instance before live_endpoint, a refusal result::unreachable(), no frame, no line; Retired (8).'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:22
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 30
    section: '## 4. Span / Trace Coverage'
    change: 'Replace «(`read_snapshot`, then `connect_by`, then `verify_server`)» with «(`read_snapshot_or_replay`, then `connect_by`, then `verify_server`)» and add after «before any frame»: «; only a snapshot read whole proceeds: a replayed or absent snapshot (a replay carries no `endpoint`, `pid` or `started_at`) is `NotSent::Unreachable`, the hook failing open with `hook-decision{detail:"channel-unreachable"}` and a SessionEnd taking the direct `append_session_end` as before».'
    sidecar: 'span-trace-coverage hook server check: connect reads read_snapshot_or_replay; replayed or absent is unreachable, fail open.'
    rationale: 'Report §Changes Symbols src/cmd/hook.rs: hook connect (224-226) reads read_snapshot_or_replay, Replayed / Absent → NotSent::Unreachable (fail open, channel-unreachable, SessionEnd direct append as before); Counts 5 names span-trace-coverage.md:30 stale; Retired: read_snapshot as a product reader 3 → 0.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 39
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Add after the must-trace chain: «The start one snapshot read (`read_snapshot_or_replay`, handed to `run.collision_check`) puts `state.snapshot_recover` under `run.start` before `run.collision_check`; on the hook path it sits under `hook.handle` (hook `connect`s read). Two span-order oracles list it (chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'span-trace-coverage run start: state.snapshot_recover placed under run.start before run.collision_check, and under hook.handle.'
    rationale: 'Report Deviations 4 (consequence: state.snapshot_recover now under run.start before run.collision_check and under hook.handle; two span-order oracles list it); Counts 6.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:39
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 42
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Extend the `run.collision_check` attribute line: «; over a replayed snapshot (no pid or start time) liveness is judged from the heartbeat alone: `beat_age` at or under `STALE_AFTER` is `already-live`, otherwise `free`; an error or an absent snapshot is `free`».'
    sidecar: 'span-trace-coverage run.collision_check: a replayed snapshot is judged from the beat alone.'
    rationale: 'Report §Changes Symbols src/cmd/run.rs: collision_check takes the caller one read (Option<&Recovered>, 425-432), Replayed → liveness from the beat alone; unit collision_check_over_a_replayed_snapshot_reads_the_beat_alone (1173-1224).'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:42
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 44
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'In the `run.conpty_sideload` attribute, the outcome list becomes `loaded|hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built` (`strict-modes-failed`: a `conpty/` dir or companion its owner/DACL check refuses, `PinError::StrictModes`, nothing written); «never a refusal and never an exit change» unchanged.'
    sidecar: 'span-trace-coverage run.conpty_sideload outcome gains strict-modes-failed.'
    rationale: 'Report §Changes Symbols src/cmd/run.rs: sideload_outcome maps PinError::StrictModes to the outcome strict-modes-failed (run.rs:510); pin.rs pin_companions runs check_present before the held open; Retired (3).'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:44
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 54
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Add to the failure bullet: «`run` and `revive`s start run `check_home_tree` (through `checked_exe` and `refuse_tree`) before `open_wrapper_log`: the home (read rule), the instance set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned exe, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`. A refusal exits 1 with the home pair (home level) or the instance pair (instance level): no log, no line, no child, so no `detail`; `revive` under `--json` keeps `strict-modes-failed`.»'
    sidecar: 'span-trace-coverage run start: the home-tree check before the log, its refusal exit 1 with no line.'
    rationale: 'Report §Changes Symbols src/cmd/run.rs checked_exe (189-200), refuse_tree (202-215), run 166-172 and revive.rs 125-135 before open_wrapper_log; founder inputs#I3 Q2.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:54
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 54
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Add to the revive sentence: «the preflight makes one snapshot read and hands it to both its collision check and its recorded-directory reading: one replay, one `state-recovered` line per revive».'
    sidecar: 'span-trace-coverage revive preflight: one snapshot read for both readings, one state-recovered line per revive.'
    rationale: 'Report §Changes Symbols src/cmd/run.rs (revive preflight reads once and hands that read to both); Deviations 4 (cli_revive «one replay, one line»).'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:54
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 55
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'In the success bullet, `sideload_fallback` becomes (`hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`, D-36); the rest unchanged.'
    sidecar: 'span-trace-coverage process-start sideload_fallback 4 → 5 words (+strict-modes-failed).'
    rationale: 'Report §Changes Schema / config: diag-line.v1.json line 70 sideload_fallback enum gains strict-modes-failed; Counts 7 names span-trace-coverage.md:55 stale.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:55
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 56
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'In the schema bullet, `sideload_fallback` is closed in the `process-start` properties to `hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`; the rest unchanged.'
    sidecar: 'span-trace-coverage diag-line schema: the sideload_fallback enum lists strict-modes-failed.'
    rationale: 'Report §Changes Schema / config: diag-line.v1.json line 70 now hash-mismatch, strict-modes-failed, unreadable, load-failed, not-built; Counts 7 names span-trace-coverage.md:56 stale.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:56
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 69
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Extend the server-check sub-bullet: «Before it, the instance check refuses a widened instance with no frame and no line: CLI `send` runs `client::refuse_untrusted` first, before any role file opens (exit 21, the instance pair or `instance-unreachable` under `--json`); MCP `send` runs `check_instance` before `live_endpoint` (`instance-unreachable`, `detail: null`). The one snapshot read is `live_endpoint`s `read_snapshot_or_replay`: a replayed snapshot carries no `endpoint`, `pid` or `started_at`, so the path ends as unreachable.»'
    sidecar: 'span-trace-coverage send path: the instance check precedes the server check; live_endpoint reads the replay-aware snapshot.'
    rationale: 'Report §Changes Symbols client.rs refuse_untrusted (send.rs:190-192), tools.rs 337-339, liveness.rs live_endpoint 92-94 (only Recovered::Snapshot proceeds); Retired (4), (8).'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:69
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 86
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'After the `send.client` `process-exit` sentence add: «A `send` refused by its instance check (`client::refuse_untrusted`, exit 21, the instance pair or under `--json` the `instance-unreachable` document) writes no `process-exit` and no line: it is refused before any role file opens (the founder own answer, `inputs#I3` Q2).» and read «no live snapshot» as including a replayed one.'
    sidecar: 'span-trace-coverage send cleanup: the strict-modes exit 21 writes no process-exit.'
    rationale: 'Report §Changes Symbols client.rs 20-41 (opens nothing under the home, writes no line; send.rs:190-192) and liveness.rs 92-94 (Replayed → None); founder inputs#I3 Q2.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:86
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 102
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Replace «`strict-modes-failed` cannot occur on them, the strict-modes half owned by the route entry "Home and code-bearing file integrity".» with «`wait` / `last` first run the instance check (`client::refuse_untrusted`) before any role file opens: a refused check exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, with no frame and no line; MCP `wait` / `last` return `instance-unreachable` with no line.»'
    sidecar: 'span-trace-coverage wait / last: the retired strict-modes gap replaced by the built instance check (no line).'
    rationale: 'Report §Changes Retired (5) CLI wait / last now run the strict-modes check before their first read; Symbols client.rs refuse_untrusted (wait.rs 43-45, last.rs 30-32); Counts 11 names span-trace-coverage.md:102 stale.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:102
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan.md
    near: 361
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Replace «`strict-modes-failed` cannot occur on them, its check owed to the route entry "Home and code-bearing file integrity"» with «a refused instance check ends a CLI `wait` / `last` before any role file opens (exit 21, the instance pair or the `instance-unreachable` document, no line), and MCP returns `instance-unreachable` with no line».'
    sidecar: 'obs-plan §4 wait / last broad line: the retired strict-modes gap replaced by the built check.'
    rationale: 'Report §Changes Retired (5); Counts 11 names obs-plan.md:361 stale.'
    basis: .andromeda/obs-plan.md:361
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 113
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'Extend the server-check sub-bullet: «Before both, the instance check: the dialog hook runs `canonical` then `trusted` before obs init, and a refused or absent instance exits 0 at once (no line, no frame, no decision: fail open); CLI `answer` runs `client::refuse_untrusted` before any role file opens (exit 21, no line), and MCP `answer` runs `check_instance` before `live_endpoint` (`instance-unreachable`, no line).»'
    sidecar: 'span-trace-coverage dialog path: the instance check precedes the server check on the hook and on answer.'
    rationale: 'Report §Changes Symbols hook.rs canonical / trusted, client.rs refuse_untrusted (answer.rs 94-96), tools.rs 337-339; Retired (6), (8).'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:113
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 122
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'After the failed-server-check bullet add: «A refused or absent instance (the instance check before obs init) is no such path: the hook exits 0 with empty streams and writes no `hook-invoked`, no `hook-decision` and no line, and sends no frame.»'
    sidecar: 'span-trace-coverage dialog log fields: a refused instance writes no hook-invoked / hook-decision.'
    rationale: 'Report §Changes Symbols src/cmd/hook.rs: a refused or absent instance returns exit 0 at once, no role file, no line, no frame; Outcome (security) viola hook event/dialog refused, no line.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:122
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 124
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'After the server-verification bullet add: «a refused instance check on CLI `answer` (`client::refuse_untrusted`, before any role file opens): exit 21, the instance pair or under `--json` the `instance-unreachable` document, no `channel-request`, no `process-exit` and no line; MCP `answer` refused by `check_instance` returns `instance-unreachable`, `detail: null`, and writes no line.»'
    sidecar: 'span-trace-coverage answer log fields: the strict-modes refusal writes no line on CLI or MCP.'
    rationale: 'Report §Changes Symbols client.rs 20-41 (answer.rs 94-96), tools.rs 337-339; founder inputs#I3 Q2 and Q3.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:124
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 126
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'Extend the last sentence: «Fail-open before `dialog_id` gives `hook-invoked{corr:null}` + `hook-decision{corr:null, detail}` (Section 6 fail-open codes), except a refused or absent instance, which writes neither.»'
    sidecar: 'span-trace-coverage dialog cleanup: the refused-instance fail-open writes no line.'
    rationale: 'Report §Changes Symbols src/cmd/hook.rs: the refused or absent instance returns before obs init with no line.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:126
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 191
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'After the second-caller sentence add: «Since chunk 2026-10-26-home-and-code-bearing-file-integrity four more product readers take `read_snapshot_or_replay` (2 → 6): `run`s start for its collision check (a start over an unreadable snapshot writes one `state-recovered{detail:"snapshot-replayed"}` line in `run-<name>.ndjson`), hook `connect` (under `hook.handle`; a replayed snapshot is unreachable), statusline `serve` (a replay carries no `statusline_command`, so nothing runs or prints) and `viola_state::liveness::live_endpoint` (the CLI frame verbs and `viola mcp`; a replay is no live endpoint), each writing its one line per call.»'
    sidecar: 'span-trace-coverage E5: the replay product readers 2 → 6 (run collision check, hook connect, statusline serve, live_endpoint).'
    rationale: 'Report §Changes Symbols liveness.rs 92-94, hook.rs 224-226, statusline.rs 128-132, run.rs 425-432; Counts 5 (replay product readers 2 → 6) and 6; tests/chaos_snapshot_replay.rs 78-83 reads run-builder.ndjson.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:191
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 192
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Replace the first sentence («The three other snapshot readers … still call `read_snapshot` and write no `state-recovered` line.») with «No product reader under `src/` calls `read_snapshot` any more (3 → 0, chunk 2026-10-26-home-and-code-bearing-file-integrity): `run`s collision check, hook `connect` and statusline `serve` read through `read_snapshot_or_replay`, as do the CLI frame verbs and `viola mcp` through `live_endpoint`.»; the `session_chain` and link-half sentences unchanged.'
    sidecar: 'span-trace-coverage E5: read_snapshot retired as a product reader under src/ (3 → 0).'
    rationale: 'Report §Changes Retired: read_snapshot as a product reader under src/ (3 → 0); Symbols remaining-caller facts (hook.rs:204, statusline.rs:142, run.rs:383 at the base; the only HEAD hit inside mod tests); Counts 5 names span-trace-coverage.md:192 stale.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:192
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 217
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'After the `plugin install` bullet add: «`verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log: a refused tree exits 1 with the home pair on stderr, no step line, no summary, no log opened and no line. `update_stamps` runs `check_stamps` before `ledger/` is created and again under the lock, reading and writing nothing on a refusal; its `StampsError::StrictModes` maps to the same home pair.»'
    sidecar: 'span-trace-coverage verify: the home-tree check before the cli log and update_stamps refusing before reading, both with no line.'
    rationale: 'Report §Changes Symbols verify.rs 64-70, 95-99, 288-296 and stamps.rs StampsError (37-44); Deviations 1; Retired (2); founder inputs#I3 Q2.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:217
note: The same new no-line sites and the panic-exemption widening owe matching edits in other categories (§10 bounded exemption list, §1 Vector 7 and Founder Direction 4 triggers, log-coverage sideload_fallback row), which this group does not propose.
