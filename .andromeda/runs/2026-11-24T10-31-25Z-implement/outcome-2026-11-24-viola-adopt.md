# Outcome — 2026-11-24-viola-adopt («`viola adopt`»)

**Outcome:** green — entries 1–13 green on their first firing (entry 10 `recorded`); entry 14 `not run — defer` (gated on
R5); entries 15–22 `leg operator`. The four remove-the-guard controls hold. Smoke skipped (the plan lists no
`role = 'smoke'` entry; the adopted wrapper booted at entries 5 and 9).

Next → the OPERATOR PASS, without R5: entry 15 (pre-push), entry 16 (hygiene, fired as written), the land and the
pre-CI commit, entry 17 (push), entry 18 (the CI read, fired as written), step 11's JUnit read and entry 19 (ci-fold).
Entries 20–22 (R5's build, R5a, R5b) wait on the operator's own word; entry 14 stays deferred. Then the wrap.

## Files changed
- new: `src/cmd/adopt.rs`, `tests/cli_adopt.rs`, `scripts/r5-adopt.sh`
- modified: `src/cmd/mod.rs`, `src/main.rs`, `src/cmd/revive.rs`, `tests/cli_output_plain.rs`,
  `tests/cross_exit_causes.rs`, `tests/contract_error_bodies.rs`, `tests/security_negatives_env_capture.rs`
- out of list, recorded: `tests/support/home.rs`

## Deviations, with their justifications
1. **Step 1, the case row «an id in upper case → exit 2».** The same step requires the session-id value parser
   revive's `--id` uses (`parse_session_id` → `viola_agent_claude::is_session_id`), which takes ASCII hex of either
   case. The two clauses cannot both hold. The shared parser stands (the binding instruction, and no new closed shape);
   the row moved into `adopt_args_take_an_id_a_name_a_parent_and_a_pair` as a pass. A red by the plan's own wording,
   if the wrap reads it so: plan step 1's row against step 1's «the same one revive's `--id` uses».
2. **Step 7, «the fake's receipt holds `--resume` and the id as two arguments».** The fake's `start` receipt records no
   argv. `adopt_resumes_the_session_under_its_name_and_parent` reads the child's own argv from
   `/proc/<child_pid>/cmdline` instead: `--resume` then the id as the next element.
3. **Step 7, «the fake planted with a turn script through its `fake-args` file».** The turn ends through the fake's
   existing `--turn-stop` mode (in `fake-args`), not a script file; no fake mode added.
4. **Step 7, `adopt_spawns_its_own_child_in_its_own_terminal`.** «The wrapper's controlling terminal is the test's
   outer PTY» is read from `/proc/<pid>/stat`: the wrapper leads its own session (the outer PTY the test opened for it,
   tty_nr non-zero), its child's parent is the wrapper, and the child leads a session of its own on another tty.
5. **Step 9, `contract_error_bodies`.** Three rows for `src/cmd/adopt.rs`, not one: the census names writers by
   (file, needle), and adopt holds two needles (`client::refused_document(self.detail());` for `strict-modes-failed`
   and `session-live`, and `client::refused_document("internal-error");`). The internal-error row's witness is a new
   case, `adopt_json_with_no_claude_on_its_path_is_the_internal_error_document` (as revive's own row reads its
   no-claude case). `cross_exit_causes` gained one row (`session-live` with adopt's hint) and one case pair
   `an_adopted_session_open_elsewhere_is_session_live`; `strict-modes-failed` and `already-live` share their existing
   hints, rows already in the table.
6. **Step 12, the rehearsal's bin.** `<base>/bin` holds the fake agent as `claude`, `foot` and `hyprctl` (the plan
   names `claude`; the implement directive `inputs#I4` asks that the PATH hold the fake `hyprctl`, `foot` and `claude`
   first). The script refuses before any start unless `command -v` resolves all three there.
7. **Step 12, the stamp.** The rehearsal's `viola verify` runs from `<base>/verify-cwd`, its trusted root, with the fake
   given `--cli-version 2.1.328 --fixtures … --screens --turn-stop --dialogs --framing --transcript --trusted-root …
   --trust-config <user>/.claude.json` after the dashes, under `HOME=<base>/user` (config seeded `{}`, 0600) and
   `XDG_RUNTIME_DIR=<base>/run`; `fake-args` is written only after the stamp.
8. **Step 12, the wrapper's pid.** Acted on only while `/proc/<pid>/exe` is `<viola>`, exactly as the plan says.
9. **Scope record:** ``- `tests/support/home.rs` · in-intent · serves step 7 · self`` — `Wrapper::adopt`, a
   constructor of the private-field `Wrapper` mirroring `revive_with` (a PTY start, `UNWRAPPED` removed, the fake's
   flags in `fake-args`).

No guard entry (`gate.py guard`) in this plan.

## Controls (`evidence/control-1.txt`)
- `adopt-live` · entry 5 · holds · `adopt-shape` · entry 5 · holds · `adopt-order` · entry 4 · holds · `adopt-parent`
  · entry 4 · holds — `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`

## Readings
- entry 4: `21 tests run: 21 passed, 1349 skipped`; entry 5: `12 tests run: 12 passed, 4347 skipped`
- entry 7: `verify_window_kept_name_reaches_no_stamp_or_record` 6.430 s, `verify_window_env_name_with_the_user_word_reads_fail_and_records_nothing` 8.421 s, `5 tests run: 5 passed, 0 skipped` (`evidence/ci-fold.md`, local half)
- entry 9: `r5a: green resumed=yes lineage=yes pair=haiku/low turn=ended ended=yes` · `r5b: green refused=session-live started=none ended=yes` · `r5: rehearsed — not a live reading`
- entries 11–13: `"ok":true` each; e2e `361 tests run: 361 passed`; integration 686 passed, `cli_adopt` among its suites

## Surfaced / stuck
none.

## Unproven, behind R5 and the forks
- v2-13's gated clause: a REAL plain session resumed by `viola adopt` (R5a) and a REAL live one refused (R5b) — read
  only over the fake agent here. No ref written.
- W1 (no signal to a process viola did not start), W2 (the operator-typed id onto the child's argv), W3 (no MCP tool):
  each on its arm 1, PROVISIONAL on the delegate's word; W4: the `session-live` pair's wording is a planning form, his
  word owed.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola` wrappers, `viola-fake-agent` children, `script` (rehearsal) | this run (tests, entry 9) | terminated — census `none running` at 10:44Z and 10:47Z (`--comm viola-fake-agen --comm script --args projects/viola/target/debug/viola`) |
| the control's clone `~/.cache/andromeda-control/63a8c146` | this run (gate.py control) | removed by the tool |
| `foot` ×5 (pids 5870, 5913, 6053, 8837, 648786) | NOT DERIVED — ppid 1509, the operator's session terminals | left running: not this run's |

Left on disk outside the tree: `$TMPDIR/tmp.U4WKIVmjt0/r5` (the rehearsal base, as the script leaves it, removed by nothing).

## Decisions & corrections
- The host read before the whole layers read CONTENDED (escher alive in a 10:29Z stall); implement stopped per the
  directive (`inputs#I4`). The operator's word resumed it: «The host reads QUIET now (the overseer's hostwatch read
  --last 6 at 2026-11-24T10:44:52Z …). Read hostwatch.py read --from 2026-11-24T10:40:00Z … then entries 10-13 and the
  operator pass WITHOUT any R5 entry» — relayed by the overseer, 2026-11-24; this run's own read 10:40:00Z–10:44:55Z
  read QUIET (5 s stalled).
- The directive's «no new time bound»: a 60 s bound added to R5b's wait in `scripts/r5-adopt.sh` was removed before
  any firing (the plan gives R5b none).
