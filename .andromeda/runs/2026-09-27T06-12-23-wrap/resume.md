# Wrap resume point — 2026-09-27-instance-state-and-start-order

Paused at the 85 % context alarm after P3 (writes done). Resume in THIS run dir (Setup 2a → resume).

## Done
- Setup: chunk `2026-09-27-instance-state-and-start-order`; base = a892917 (the pre-CI commit 39b8297's parent);
  HEAD ed359cd = origin (0 ahead); code-graph refresh fired in the background (`code-graph-refresh.log`).
- P1: `viola-0.1.0/chunks/…/report.md` written; evolve `report` record appended.
- P2: fan-out (7 docs, 48 proposals; `fanout-results.md`, raw twins), `validation.md`, bodies applied (architecture
  18 lines, security-plan 7+1, obs-plan 3+1, test-plan 21+2), sweep ×2 (`cascade-sweep*.txt`,
  `cascade-sweep.md`), 19 leaf lines re-derived in 9 files, four sidecar entries appended + read back. Escalations
  0 open (Snapshot-writer reversal + TMPDIR carve-out settled by recorded overseer directions). Evolve `reconcile`
  records appended.
- P3: testing.md Session Additions — one new entry + one in-place extension (`curation.md`). The P3 evolve
  checkpoint (`references/evolve/curation.md`) has NOT run yet.

## Remaining (in order)
1. P3 evolve checkpoint (curation playbook).
2. P4: read `.andromeda/cache/.refresh-done` / `.refresh-stale` (+ `code-graph-refresh.log`).
3. P5 route-resolve (markerless tail only):
   - CARRY on "Wrapper channel" (working-route:40): the viola-pty Windows flake —
     `viola-pty tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code`, CI history 0.033 s
     (a892917, job 108526305228) / 0.034 s (054ebe4, job 108516959970) / FAIL 10.031 s lib.rs:1099 (39b8297, job
     108555954166) / 0.022 s (ed359cd, job 108560461939); owner: its /implement; on recurrence force the
     resize-then-key window with a test-only hold. PLUS the plan's route freight: the snapshot `endpoint`, the
     exclusive-bind arbiter for concurrent starts, and the Scenario-1 spans `run.start` / `run.collision_check` /
     `run.pin_copy` / `state.snapshot_write` / `state.heartbeat_start` (obs O1 rejected to this CARRY).
   - CARRY on "Hooks to normalised events" (:43): fill `plugin/hooks/hooks.json` with the absolute pinned-path
     commands; the M6 witness via the fake agent's `command_absolute` receipt; `events.ndjson` line 3 +
     harness readiness events check.
   - CARRY on "Statusline pass-through" (:67): `instances/<name>/settings.json` rewritten each start
     (`replace_private`).
   - CARRY on "Self-healing state" (:65): the crate-level `crates/viola-state/tests/` event-kind round-trip suite
     (test-plan T17/T18 rejected to it).
   - CARRY on "The board: viola list" (:73): the Path 1 `path_` E2E binary (T19/T20 rejected to it).
   - Route NOTE (overseer item 4, not an entry): pre-push's local coverage has no Windows coverage `test` stage and
     cannot reproduce runner-speed timeouts (green twice on trees CI read red: 36296402785).
   - The fix-by-reasoning label: `replace_private_shared` — a fix BY REASONING for the one red of
     `viola-e2e::harness_lifecycle harness_session_boots_reports_logs_and_tears_down` (report Insufficient fixes).
   - Evolve `route-resolve` checkpoint.
4. P6: state.yaml (last_wrap, session_count 15→16, tree_db_refreshed_at) + handoff (incl. Deferred learnings:
   the four recurrences in `curation.md`; the playbook-rule proposal "a locked-decision reversal settled by a
   recorded operator direction → apply, record the direction in the sidecar" for the operator).
5. P7: light gate (`gate.py run --plan … --run-dir {this dir}`, backgrounded, no `--only`; leg entries 22–24 from
   `chunks/…/evidence/operator-pass.md`), drift=0 gate, coverage (`matrix.py flip` — claimed 0 → no-op), `route.py
   flip` + `compact`, commit (`feat(2026-09-27-instance-state-and-start-order): …`), stamp tree.db.commit, push,
   evolve `gates` checkpoint.
