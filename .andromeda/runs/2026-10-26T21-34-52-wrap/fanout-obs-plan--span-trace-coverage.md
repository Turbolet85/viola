proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 192
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Replace the opening sentence («The three other snapshot readers … still call `read_snapshot` and write no `state-recovered` line.») with: «Since chunk 2026-10-26-home-and-code-bearing-file-integrity no product reader under `src/` calls `read_snapshot` (3 → 0; the rule: `read_snapshot(` under `src/`, outside test modules): `run`s collision check, `src/cmd/hook.rs`s `connect` (its event and dialog paths sharing the one read) and `src/cmd/hook/statusline.rs`s `serve` read through `read_snapshot_or_replay`, and so does `viola_state::liveness::live_endpoint` (the CLI frame verbs and `viola mcp`s frame tools), each writing the replay its one `state-recovered` line. `live_endpoint` and the hook `connect` proceed only on the snapshot itself (a replayed or absent one carries no `endpoint`, `pid` or `started_at`: no endpoint, the hook `channel-unreachable`), `serve` runs and prints nothing over a replayed snapshot (it carries no `statusline_command`), and the collision check reads a replayed snapshot its liveness from the beat alone.» Keep the rest of the group (`session_chain`, the link half) unchanged.'
    sidecar: 'span-trace-coverage E5: the three readers left on `read_snapshot` moved onto the replay with `live_endpoint` (read_snapshot product callers 3 → 0), chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Retired names `read_snapshot` as a product reader under `src/` (3 → 0); Count 5 lists span-trace-coverage.md:192 stale; Symbols (liveness, hook.rs, statusline.rs, run.rs) state each reader on `read_snapshot_or_replay`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:32
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 30
    section: '## 4. Span / Trace Coverage'
    change: 'Replace «(`read_snapshot`, then `connect_by`, then `verify_server`)» with «(`read_snapshot_or_replay`, its `state.snapshot_recover` under `hook.handle`, then `connect_by`, then `verify_server`)», and add: «A `Replayed` or `Absent` read is `NotSent::Unreachable`: no frame, the hook fails open with `channel-unreachable`, and a SessionEnd takes the direct `append_session_end` as before.»'
    sidecar: 'span-trace-coverage hook server check: the shared `connect` reads through `read_snapshot_or_replay`, a replayed or absent snapshot unreachable, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols src/cmd/hook.rs (connect 224-226 reads `read_snapshot_or_replay`; Replayed / Absent → `NotSent::Unreachable`); Count 5 lists span-trace-coverage.md:30 stale; Deviation 4 places `state.snapshot_recover` under `hook.handle`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 191
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Append after the second product caller: «Since chunk 2026-10-26-home-and-code-bearing-file-integrity four more product readers write it (replay product readers 2 → 6): `run`s collision check, its `state.snapshot_recover` under `run.start` before `run.collision_check`; the hook `connect` (event and dialog paths), under `hook.handle`; `hook statusline`s `serve`; and `viola_state::liveness::live_endpoint` (the CLI frame verbs and `viola mcp`s frame tools), one line per call. `revive`s preflight reads once and hands that one read to both its collision check and its recorded-directory reading, so a revive over an unreadable snapshot still writes one line.»'
    sidecar: 'span-trace-coverage E5: four new `state-recovered` emitters and the `state.snapshot_recover` placement under `run.start` / `hook.handle`, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Count 6 (span-trace-coverage.md:191 an emitter-list row re-read for the four new readers), Count 5 (replay product readers 2 → 6), Deviation 4 (one read in revive, the span placement).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:33
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 102
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Replace «`wait` / `last`, like `send`, take the liveness-only pre-check and then the server check; `strict-modes-failed` cannot occur on them, the strict-modes half owned by the route entry "Home and code-bearing file integrity".» with «`wait` / `last`, like `send`, run the instance strict-modes check first (`client::refuse_untrusted`: a refusal exits 21 with the instance pair, or under `--json` the `instance-unreachable` document, opens no process log and writes no line, so no `process-exit` and no `strict-modes-failed` detail), then the liveness-only pre-check and the server check.»'
    sidecar: 'span-trace-coverage wait/last: the dated strict-modes gap retired; the check runs first and a refusal writes no line, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Retired gap (5) CLI `wait` / `last`; Count 11 lists span-trace-coverage.md:102 stale; Symbols client.rs `refuse_untrusted` (wait 43-45, last 30-32); the founder rejected logging `strict-modes-failed` after a refused check (Reverted, `inputs#I3` Q2).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:38
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 55
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'The `sideload_fallback` list reads `hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`; add: «`strict-modes-failed` is a companion its owner/DACL check refuses (`pin_companions` runs `strict::check_present` on `bin/<key>/conpty/` and on each companion that exists before the held open; `PinError::StrictModes`, nothing written), the child left on the inbox ConPTY with the exit unchanged.»'
    sidecar: 'span-trace-coverage run start: `sideload_fallback` gains `strict-modes-failed` (4 → 5 words), the conpty owner/DACL leg, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Count 7 (sideload_fallback 4 → 5 words; span-trace-coverage.md:55 stale), Schema / config (diag-line.v1.json line 70), Symbols pin (`PinError::StrictModes`, `check_present`) and run.rs:510 (`sideload_outcome` maps it to `strict-modes-failed`).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:34
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 56
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Replace «closed in the `process-start` properties to `hash-mismatch|unreadable|load-failed|not-built`» with «closed in the `process-start` properties to `hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`».'
    sidecar: 'span-trace-coverage run start: the schema closed `sideload_fallback` enum restated at five words, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Schema / config: `schemas/diag-line.v1.json` line 70, the `process-start.sideload_fallback` enum gains `strict-modes-failed`; Count 7 lists span-trace-coverage.md:56 stale.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:24
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 44
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'The `run.conpty_sideload` outcome list reads `loaded|hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built` (`sideload_outcome` maps `PinError::StrictModes` to `strict-modes-failed`); the rest of the row unchanged.'
    sidecar: 'span-trace-coverage run start: `run.conpty_sideload` outcome gains `strict-modes-failed`, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols run.rs: `sideload_outcome` maps `PinError::StrictModes` to the outcome `strict-modes-failed` (run.rs:510); the same five-word outcome as Count 7, enumerated here as the span attribute.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
    dependent-of: D-obs-instrumentation
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 39
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Must-trace chain reads `run.start` (INTERNAL, root) › `state.snapshot_recover` › `run.collision_check` › … (the snapshot read the collision check takes, chunk 2026-10-26-home-and-code-bearing-file-integrity); add: «A fourth start step carries no span and no line: the home-tree check (`checked_exe`: the exe bytes read once, `pin_key`, `strict::check_home_tree` with the instance name and key), made before the wrapper log opens.»'
    sidecar: 'span-trace-coverage run start: `state.snapshot_recover` before `run.collision_check`; the home-tree check a no-span, no-line step before the log, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Deviation 4 (`state.snapshot_recover` now sits under `run.start` before `run.collision_check`; two span-order oracles list it); Symbols run.rs `checked_exe` (189-200) called before `open_wrapper_log`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:97
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 42
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Add to `run.collision_check`: «it takes the caller its one read (`Option<&Recovered>`): a replayed snapshot is judged by the beat alone (`beat_age ≤ STALE_AFTER` → `already-live`, else `free`); an error or an absent snapshot → `free`.»'
    sidecar: 'span-trace-coverage run start: `run.collision_check` outcome over a replayed snapshot read from the beat alone, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols run.rs: `collision_check` now takes the caller its ONE read (425-432), `Replayed` → liveness from the beat alone, error or `Absent` → free; unit `collision_check_over_a_replayed_snapshot_reads_the_beat_alone`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 54
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'Add: «Before its log opens, `run` (and `revive`s start, after its instance check) runs the home-tree check (`check_home_tree`: the home, `instances/<name>`, `ledger/`, `bin/`, `plugin/`, `bin/<key>/` and the pinned copy, `plugin/<key>/`); a refusal exits 1 with the home pair (`HomeLevel`) or the instance pair (`InstanceLevel`) on stderr, opens no log, writes no line and starts no child, so it has no `process-exit` and no `detail` (`revive --json` keeps `strict-modes-failed`).»'
    sidecar: 'span-trace-coverage run start: the home-tree refusal before the log (exit 1, no line), chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols run.rs (`checked_exe` / `refuse_tree`; `run` 166-172 and revive.rs 125-135 call them before `open_wrapper_log`: exit 1, no log opened, no line, no child); Count 20 lists span-trace-coverage.md:54 an exemption-list row re-read for the new site.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:47
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 18
    section: '## 4. Span / Trace Coverage'
    change: 'In the cli (short-lived verbs) row add: «`send`, `wait`, `last`, `answer`, `pause` and `release` run the instance strict-modes check first (`client::refuse_untrusted`, `check_instance` on the home and `instances/<name>`; `send` after its text read), the check `link` / `unlink` share: a refused check (exit 21, the instance pair, or under `--json` the `instance-unreachable` document) opens no process log and writes no line (the founder its own answer, `inputs#I3` Q2).»'
    sidecar: 'span-trace-coverage cli row: the six frame verbs join the no-line strict-modes refusal, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols client.rs `refuse_untrusted` (20-41) called first by the six verbs; Count 20 lists span-trace-coverage.md:18 re-read for the six verbs; Coverage of new surfaces: instrumentation n/a, no line by the founder word.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 28
    section: '## 4. Span / Trace Coverage'
    change: 'Replace «after its own client-side refusals (for `link` / `unlink`, their instance check of the driven instance, then the liveness pre-check)» with «after its own client-side refusals (for every one of the eight, the instance strict-modes check `client::refuse_untrusted` first, then the liveness pre-check)».'
    sidecar: 'span-trace-coverage server check: the instance check precedes the liveness pre-check on all eight frame verbs, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols client.rs: `refuse_untrusted` called first by send, wait, last, answer, pause, release, and `link_call` moved onto it; Count 2 (check_instance set and call-site files moved).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 20
    section: '## 4. Span / Trace Coverage'
    change: 'In the cli (`viola hook`) row add: «The event and dialog paths run the instance check (`canonical`, then `trusted`) before obs init: a refused or absent instance returns exit 0 at once with no `config.json` read, no role file, no stdin read, no line, no frame and no SessionEnd append; a `VIOLA_DIR` naming an instance directory that does not exist writes nothing. The two capture arms stay ahead of it.»'
    sidecar: 'span-trace-coverage hook row: the event and dialog paths check the instance before obs init and write nothing on a refusal, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols src/cmd/hook.rs (`canonical` 75-81, `trusted` 83-86 before `read_diagnostics_level` / `viola_obs_init`), Count 23 (a `VIOLA_DIR` naming no directory writes nothing), Retired gap (1).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 31
    section: '## 4. Span / Trace Coverage'
    change: 'Add: «Before `live_endpoint`, each frame tool runs `check_instance(home, instance_dir)` inside `mcp.tool_call`; a refusal returns `instance-unreachable` with `detail: null`, no frame and no line.»'
    sidecar: 'span-trace-coverage mcp: the frame tools check the instance before the liveness read, a refusal no line, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols viola-mcp tools.rs `frame` (337-339): `check_instance` before `live_endpoint`, a refusal `result::unreachable()`, no frame, no line; Retired gap (8).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 216
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Add: «Before its `cli` log opens, `verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/`; a refusal exits 1 with the home pair, no step line, no summary and no line; a `StampsError::StrictModes` from `update_stamps` (checked before `ledger/` is created and again under the lock) maps to the same pair.»'
    sidecar: 'span-trace-coverage verify: the home-tree check before the cli log and the stamps strict-modes refusal, no line, chunk 2026-10-26-home-and-code-bearing-file-integrity.'
    rationale: 'Report Symbols verify.rs (64-70 `check_home_tree` before the `cli` log, `refuse_home` 95-99, `measure` 288-296) and stamps (`StampsError`); Count 20 lists span-trace-coverage.md:217 among the exemption rows re-read; Retired gap (2).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19
note: 'Body obs-plan.md carries the same stale facts for other fanouts: :361 (wait/last «strict-modes-failed cannot occur»), :394 («the other snapshot readers write no state-recovered line»), :215 (Vector 7: a strict-modes failure now also has the sideload_fallback line), :311 (human::refuse per-file counts); obs.rs check_diagnostics (a refused diagnostics open ends at sink) belongs to log-coverage, not proposed here.'
