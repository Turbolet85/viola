# Implement run journal — 2026-10-26-server-verification-before-any-frame

Long runs, journalled before each wait (the operator's ask at invocation).

- 2026-10-26 08:34Z · P2 block run 1 — `gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --run-dir .andromeda/runs/2026-10-26T08-22-16-implement --marker 2026-10-26-server-verification-before-any-frame`, backgrounded. 20 non-leg entries (the three `operator` entries 21–23 not fired by plain `run`); the longest bounds are entry 19 `run --perf` (3600 s) and entry 20 `pre-push` (7200 s); P5 read them at 16.95 s and 87.61 s. Before it: the three widened binaries and the new unit cases green by direct nextest (59 and 46 run), and the red-before-green control recorded in the chunk's `evidence/red-before-green.md`.
  Result (2026-10-26 08:37:11Z, entry 21's log mtime): `entries 24 · green 19 · red 1 (21) · recorded 1 · timeout 0 · not-run 3` — entry 21
  `pre-push` red on `contract_fake_agent_drift::fake_agent_agents_replay_matches_every_recorded_agents_set`
  (ETXTBSY). Folded on the overseer's answer (`inputs#I5`), fixed, 40-run loop 0 red (`evidence/etxtbsy-drift-test.md`);
  the captured-span cases for the must-trace check added (`evidence/red-before-green.md`, second reading).
- 2026-10-26 08:42Z · P2 block run 2 — the same full-block call, backgrounded, over the fixed tree.
  Result (2026-10-26 08:44:41Z, entry 21's log mtime): `entries 24 · green 18 · red 2 (13,21)` — 13 the guard's
  `outside 1 files` (`tests/contract_fake_agent_drift.rs`, accepted as named, `inputs#I5`); 21 `pre-push` red on
  `contract_windows_mutation_scope::windows_gated_sources_equal_the_workflow_scope_both_ways` («unlisted:
  src/cmd/client.rs»): this run's own cause — the new `refused_inside` test module was gated
  `#[cfg(all(test, not(target_os = "macos")))]`, so its `cfg!(windows)` sat before the file's first column-0
  `#[cfg(test)]` and read as a product Windows gate. Fixed by stacking `#[cfg(test)]` and
  `#[cfg(not(target_os = "macos"))]`; the contract binary and the span cases green directly (11 run, 11 passed).
- 2026-10-26 08:45Z · P2 block run 3 — the same full-block call, backgrounded.
  Result: `entries 24 · green 19 · red 1 (13) · recorded 1 · timeout 0 · not-run 3` — 13 the guard's accepted
  outside line alone; pre-push green at `linux-tests` (88.21 s); perf hook maxima 2.2–2.4 ms.
- 2026-10-26 08:48Z · P3 smoke by hand — `agent-run.sh boot --session p-svf-p3 --instance builder` (output to a
  scratch file, never piped), `status`, then the session's own `viola last builder --json` and `viola wait builder
  --timeout-ms 500 --json` against the real wrapper, then `cleanup`. Done: both exit 0, cleanup clean
  (`evidence/smoke-p3.md`). P4: `scope: clean`, census none of this run's; outcome written.
- 2026-10-26 08:50Z · operator pass step 1 (`.claude/rules/ci.md` §The operator pass), on the operator's word
  (`inputs#I6`): the native gate on the uncommitted tree — `python -X utf8 {tools_dir}/gate.py run --plan
  viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --entry 21` (no run dir: the pass runs
  under no skill; its line goes into `evidence/operator-pass.md`).
  Step 1 green (87.67 s); step 2 `hygiene: clean`, re-read clean. Next: the pre-CI commit, the push (entry 23), then
  the CI read (entry 24, `--wait 2400`, backgrounded) — journalled after the push lands (ci.md 2026-10-13).
