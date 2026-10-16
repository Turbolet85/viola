# Implement journal — 2026-10-14-resume-and-compact-ledger-rows

Stopped at 2026-10-16T07:00Z on the founder's word, relayed by the operator: the account is at 99% and every
session closes for an account switch. No edit was in progress when the word came, only a read of
`gate-contract.md`. The working tree is preserved. Nothing was committed.

## Plan steps done (P1, code only)
- 1. `LedgerRow::ResumeSession` added as the 22nd row: id `resume-session`, words `--resume <id> reopens the session
  under that id`, relied on every OS (`relied_on` `_ => true`).
- 2. `Probes::resumed`, plus `resume_target`, `session_resumed` (private), `resume_variants` and the `check` arm.
  - Unit cases: the five `check_resume_session_*` cases, plus `resume_target_is_run_b_first_session_id_of_the_shape`
    and `resume_variants_name_run_e_first_session_start_resume_1`.
  - Shape readers moved:
    - the verdict vector is `[true; 22]`;
    - the ids and words list;
    - the relied counts are 22 on Unix and 19 on Windows;
    - the older-stamp oracle gains the twenty-one-row and eighteen-plus-resume stamps;
    - `merge_stamp`'s envelope literal gains `resume-session`.
  - The ledger test fixture's Run B first id is now `RUN_B_ID`, a session-id-shaped literal.
- 3. `LOCAL_COMMANDS` is `[…; 3]` with `("/compact", None)`. Its unit is renamed `…_the_three_known_ones_…`.
  `/compact` was added to the `framing_stem` negatives.
- 4. `classify.rs` gains `/compact` rows in the unconfirmable table and the trailing-newline table (the latter now
  takes a `command` case parameter). `/compactx` and ` /compact` are added to the exact-text table. No product
  change.
- 5. Run E (`resume_run`) in `typed.rs`:
  - `measure` runs five steps.
  - It reads Run B's captures through `super::read_captures` and takes the target from `ledger::resume_target`.
    With no target, no child is spawned.
  - It runs in a `-resume` `TrustedDir`.
  - argv: the run args, then `--plugin-dir <resume plugin>`, then `resume_args(id, false)`.
  - Input is `io::empty()`. It waits for one SessionStart capture, then settles, then kills.
- 6. `verify.rs`:
  - `ProbeDir` gains `resume/` with the spine capture plugin.
  - `Inputs.resume` and `Probes.resumed` are wired.
  - `named_payloads` adds `SessionStart.resume-1.json`.
  - The step-counter unit moves to 22/19. The `verify --help` text names no run count, so it is unchanged.
- 7. The fake agent: `RESUME_VARIANT` and `Agent::launch_variant`. Under `--resume`, the launch fires
  `SessionStart.resume-1` when the set holds it, else `default`. `resume_fields` is unchanged and no option was
  added. New unit case: `fake_resume_launches_on_the_recorded_resume_variant_when_the_set_holds_one`.
- 8. `version_gate.rs`: the older-stamp oracle now has eighteen (unverified on both OSes), eighteen plus resume
  (verifies on Windows only), twenty-one (unverified) and twenty-two (verified). `ResumeSession` joins the
  failing-row loop.
- 9. The per-OS literals:
  - harness `LEDGER_ROWS` is 22/19, and `LIVE_PASS` is 22/19 including the `resume-session` line;
  - `cli_verify.rs`: `STEPS_PASS`, `ROW_IDS` and `STAMPED_PASS` are 22/19;
  - `contract_ledger_probes.rs`: `ROW_IDS` and the stamped line are 22/19.
- 9 and 10, other `cli_verify.rs` oracles. These are measured forecasts, not yet run:
  - the dispatch-error and spawn-pair lists gain a fifth `verify-pty-probe` pair, and the test is renamed
    `…_seven_spawn_pairs`;
  - the record file list gains `SessionStart.resume-1.json`;
  - the run-dir case reads 6 starts passing and 5 failing (`-resume` named);
  - `trusted_run_typed` expects 6 starts;
  - the summaries:

    | Case | Unix | Windows |
    |---|---|---|
    | stop-less set | 5 pass 17 fail | 5 pass 14 fail |
    | no-screens | 7 pass 15 fail | 7 pass 12 fail |
    | no-dialogs | 18 pass 4 fail | 15 pass 4 fail |
    | no-framing | 18 pass 4 fail | 18 pass 1 fail |
    | tag-modal | 20 pass 2 fail | 17 pass 2 fail |

  - The existing stop case gains a foreign `.viola-verify-0-resume` dir.
  - New case `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` (Unix, SIGINT and SIGTERM). Its set
    has no default SessionStart and a session-id-shaped `clear-1` id, so Run E waits deterministically.
    `support::verify::verify_started` gained an `after` parameter.
  - `tests/support/verify.rs`: the synthetic `s-verify-1` id became `SESSION_ID`, a session-id-shaped literal.
    Without it Run E never starts. The file is on research's list.
- 10. Fixture contracts:
  - `contract_fixture_hygiene.rs`: new `claude_resume_variants_are_walked_for_every_resume_set`.
  - `contract_fake_agent_drift.rs`: new `fake_agent_resume_replay_matches_every_recorded_resume_set`, which runs
    the fake agent under `--resume <the variant's id>` with an EOF after the first hook.
  - `cli_fake_agent.rs`: the existing resume and fork cases read the `resume-1` base, and there is a new
    ten-key-name case.
  - These go red until step 13 lands the fixture, by design.
- 11. `cli_send.rs`: new `send_compact_on_a_verified_cli_is_unconfirmable_at_once` (`stamped_home`,
  `--local-command-mode`).

## Files changed (all on research's lists; `scope-record.md` not needed so far, 0 lines)
- `crates/viola-agent-claude/src/ledger.rs`
- `src/run/send/classify.rs`
- `src/cmd/verify/typed.rs`
- `src/cmd/verify.rs`
- `src/bin/viola-fake-agent.rs`
- `src/run/version_gate.rs`
- `crates/viola-e2e/src/harness/run.rs`
- `tests/cli_verify.rs`
- `tests/contract_ledger_probes.rs`
- `tests/contract_fixture_hygiene.rs`
- `tests/contract_fake_agent_drift.rs`
- `tests/cli_fake_agent.rs`
- `tests/cli_send.rs`
- `tests/support/verify.rs`

Not yet created: `fixtures/claude/2.1.287/SessionStart.resume-1.json` (step 13) and `evidence/live-round.md`
(step 15).

## Run / not run
- Run:
  - `rustfmt --edition 2024` on each edited file;
  - `cargo check -q --workspace --all-targets --features viola/fake-agent`, exit 0 (07:00Z).
  - Evolve records were appended: the new-session orientation record and the P1 `code` checkpoint (3 records).
- NOT run, nothing at all from the plan's `[[gate]]` block: no `gate.py run`, no fmt, clippy or lint-probes gate,
  no unit or integration suite, no smoke, no pre-push. No Windows-target clippy check
  (`--target x86_64-pc-windows-msvc`, `CARGO_TARGET_DIR=target/wincheck`) either.
- Live claude starts used: **0 of 12**. No record round and no `--local-live` round were started.
- Process census: none started by this run.

## Open items to read at re-entry
- Step 12 risk (forecast, not measured): in the `verify_window_` class Run E's settle has no literal, so it waits
  out `GATE_MAX_WAIT` once more. That is about 35 s → 43.5 s.
  - Against the mutants profile's 45 s kill this is tight, and the CI profile kill is 60 s.
  - `.config/nextest.toml` is outside the preservation guard, so it cannot move here. Record the reading and
    surface it.
- `tests/cli_revive.rs` and `tests/chaos_revive.rs` may read the fake agent's resume payload. Once the variant
  exists the base moves to `resume-1`, with 10 keys instead of 5. Read them under the integration entry.
- Several contract and fake-agent cases go red until the record round lands `SessionStart.resume-1.json`, by
  design: the cli_fake_agent resume cases, the hygiene resume case and the drift resume case.
- The `cli_fake_agent` ten-key literal and the drift byte-equality are both forecasts, to be read against the
  recorded file. If byte equality fails, stop and report (plan step 10).

## Re-entry point
`/andromeda-implement` on this chunk, **Phase 2**:
1. Reuse this run dir or open a new one. Read gate-contract.md §Tool, §Firing forms, §What the tool reads,
   §Outcome words, §Exit grammar, §Where the logs live, §Call table and §Live round.
2. Fire the non-live gates first, in the background: `gate.py run --plan …/plan.md --run-dir … --marker …`. That
   means fmt, clippy, G1, lint-probes, `run --unit`, and the `check_resume_session_` filter (expect `passed":5`).
   Fix to green.
3. Then the live round (`--live-legs`). Ledger each start with `date -u` before it is made, and journal the run
   here before waiting on it. The order is the record entry (6 starts), census, probe, `--local-live` (6 starts),
   census. A red round is reported and never re-fired.
4. Then the fixture guard, the integration entry (step 12's after-reading), preservation, `cargo deny`, the
   smoke, G2, G4 and pre-push.
5. Then step 15 `evidence/live-round.md`, the matrix refs (none expected: `v1-34` stays unclaimed), the P3 smoke
   and the P4 report. After that the operator pass (hygiene, push, `ci.py conclusion`), then the wrap.

## Resumed (the founder's word: finish implement, stop before the wrap)
- 2026-10-16T07:15Z: resumed in this run dir. The dry-run listing reads 26 entries: 7–12 are the round, 24–26 are
  operator entries.
- 07:16Z: firing entries 1–6 (`--only 1,2,3,4,5,6`) in the background. No live start.
- Before 07:17:03Z (a time written here first as "07:22Z" without reading the clock; corrected against `date -u`
  07:17:03Z): entry 5 red, one failure: `ledger::tests::check_typed_rows_fail_without_a_measurement`, a second verdict-vector literal still at 21 members. The literal was fixed and entries 1,2,5,6 re-fired.
- 07:17:03Z pre-readings: target/e2e-home is a link onto tmpfs; ~/.claude/projects holds 76 dirs and 643 .jsonl transcripts (list kept in the session scratchpad); 16 processes named claude already running (not this tree's); the 2.1.287 binary is present.
- Windows-target clippy (CARGO_TARGET_DIR=target/wincheck, --target x86_64-pc-windows-msvc, -D warnings): exit 0, no output.
- LIVE START LEDGER, round 1 (record), date -u 2026-10-16T07:17:12Z, before firing: starts 1-6 of 12 = the print probe and Runs A-E of one viola verify --record against 2.1.287 (gate entry 8, after round step 7). Fired once, never re-fired.
- 07:17Z: firing gate.py run --live-legs --entry 8 in the background, waiting on its completion notice.
- 07:18:16Z round 1 read: round: COMPLETE, entry 7 green, entry 8 green 52.67 s, stamped 2.1.287  22 pass  0 fail; resume-session pass live; SessionStart.resume-1.json copied (578 B, byte-equal to the recording, ten key names, id = Run B's first id, not the clear id). Starts used: 6 of 12.
- 07:18:40Z transcripts: 643 -> 647 (+4: Run B x2, Run C, Run D); Run E added no transcript file.
- LIVE START LEDGER, round 2 (run --local-live), date -u 2026-10-16T07:18:40Z, before firing: starts 7-12 of 12 = the print probe and Runs A-E of the harness's one viola verify against 2.1.287 on PATH (gate entry 11, after round steps 9 and 10). Fired once, never re-fired.
- 07:18Z: firing gate.py run --live-legs --entry 11 in the background, waiting on its completion notice.
- 07:19:42Z round 2 read: round: COMPLETE, entries 9, 10 green, entry 11 green 48.43 s, local-live passed 1 failed 0. Starts used: 12 of 12. The cap is spent; no further live start.
- 07:19:50Z census 12 run by hand (entry 12 fires only in a whole round): no survivor; no .viola-verify-* or /tmp/viola-verify-<suffix> dir left. Transcripts 647 -> 651 (+4: Run B x2, C, D); Run E added none in either round.
- evidence/round-071816Z.txt and evidence/round-071942Z.txt written.
- 07:20Z: firing entries 13-22 (--only) in the background.
- entry 14 (integration) red, 3 of 196: (a) the new stop case asserted no child_exit_status on Run E's killed exit, but the PTY path logs the pump's exit code for a killed child (pre-existing; Run B's stop case never asserted it): assertion dropped, duration kept; (b) verify_runs_the_probe_under_the_identity_strip counts env receipts 5 and hooks 37, now 6 and 38 with Run E (one more start, its launch SessionStart). Re-firing 14.
- entry 14 re-fired: green, 196/196, 48.81 s (Summary 47.56 s).
- step 12 reading (P5 before-reading = the phase run's 14.log, 190 tests, Summary 38.61 s): verify_window_without_screens_fails_every_interactive_row 34.42 s -> 42.98 s (+8.56 s, Run E's settle with no screen waits out GATE_MAX_WAIT, by design); CI verify_window_ kill 60 s, mutants-profile kill 45 s (margin about 2 s locally); every other test <= 13.14 s under its 20 s kill. No kill moved (plan step 12).
- Windows-target clippy re-check not needed for the test-only edits since (cli_verify.rs bodies only).
- firing entry 23 (pre-push) in the background.
- 07:24:24Z entry 23 (pre-push) red: coverage 2551/2552, viola::cli_answer path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human: tests/support/home.rs dialogless_home pins the dialogless stamp at 17/14 pass (now 18/15 with the resume row). Not in research's lists and not in the integration filter: fixed as a companion, scope-record.md line written at the edit. The preservation guard (entry 15) does not exclude tests/support/home.rs, so it is expected to read red: a plan-guard gap to surface. Re-firing 1, 15, 23.

## Stop point (2026-10-16T07:28:01Z)
- Gates: 1-6 green; round 1 (7, 8) and round 2 (9, 10, 11) COMPLETE, 12 of 12 live starts used, none re-fired; census 12 run by hand, no survivor; 13, 14, 16-22, 23 green (pre-push: coverage 2552/2552, playwright 1/1, gate no breach). Entry 15 (preservation guard) red, only on tests/support/home.rs, a companion recorded in scope-record.md; scope read clean.
- Asked the operator; answer (the overseer, founder-delegated, inputs#I3, relay-1.md): re-plan the guard; stop here with no commit or push; /andromeda-phase's plan revision comes next.
- Not done: the operator pass (hygiene, pre-CI commit, push, ci.py conclusion). No commit, no push. The working tree holds the whole chunk.
- Re-entry: after the plan revision adds tests/support/home.rs to entry 15's exclusions, re-fire entry 15 (and fmt) through gate.py, then the operator pass (24 hygiene as written, the pre-CI commit, 25 push through --operator, 26 ci.py conclusion as written), then the wrap. No live start is needed: the cap is spent and the rounds are recorded in evidence/.
