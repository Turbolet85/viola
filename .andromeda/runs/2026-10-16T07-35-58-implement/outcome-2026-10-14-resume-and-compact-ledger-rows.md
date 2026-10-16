# Implement outcome — 2026-10-14-resume-and-compact-ledger-rows (re-entry after the plan revision)

## Outcome: green + smoke

This run re-entered after the plan revision (`.andromeda/runs/2026-10-16T07-29-18-phase/`, on the operator's word
`inputs#I4`). The stopped run is `.andromeda/runs/2026-10-16T06-46-40-implement/`; its `journal.md` and
`outcome-2026-10-14-resume-and-compact-ledger-rows.md` carry steps 1–15, both live rounds and the deviations, which
this report repeats below. This run wrote no code.

## Gates

This run's firing was one call: `gate.py run … --only 1,15,16,17,18,19,20,21,22,23` (run dir
`.andromeda/runs/2026-10-16T07-35-58-implement/`). Verbatim summary: `entries 26 · green 10 · red 0 · recorded 0 ·
timeout 0 · not-run 16`.

| Entry | What | Reading |
|---|---|---|
| 1 | fmt | green, 0.91 s |
| 15 | preservation guard (revised: `tests/support/home.rs` excluded) | **green**, 0.01 s |
| 16 | `cargo deny check` | green, 0.75 s |
| 17–22 | smoke (cleanup, boot, status), G2, G4, cleanup | green; boot 4.51 s, cleanup green on `processes_gone` and `endpoint_gone` |
| 23 | `pre-push` | green, 73.82 s |

The stopped run read these, and the plan's Revision names them done. None were re-fired here:

| Entry | Reading |
|---|---|
| 2–6 | green (unit 2090 tests; `check_resume_session_` `passed 5`) |
| 7–8 | round 1: `round: COMPLETE`, `stamped 2.1.287  22 pass  0 fail` |
| 9–11 | round 2: `round: COMPLETE`, `local-live passed 1 failed 0` |
| 12 | census, run by hand: no survivor |
| 13 | fixture guard: green |
| 14 | integration: green, 196/196 |

- **Live starts:** 12 of 12, all in the stopped run. None in this run, because the cap is spent.
  - Evidence: `evidence/round-071816Z.txt`, `evidence/round-071942Z.txt` and `evidence/live-round.md`.
- **Deferrals:** none. There was a Rust delta, so every Rust gate ran, either in the stopped run or here.
- **Tree:** unchanged between the stopped run's last edit (`tests/support/home.rs`) and this run's firing. This run
  wrote no source.
- **Matrix:** `matrix.py show --chunk` reads `claimed … 0`, so no ref was written. `v1-34` stays unclaimed.
- **P3:** the smoke entries ran as P2 gates (17–22). They are recorded here and were not re-driven.
- **Scope read:**
  - `scope: clean — changed 16 · listed 16 · recorded 0 … · absorbed 0 · excluded 84`.
  - After the revision, research lists `tests/support/home.rs`, so the read prints `record: listed — tests/support/home.rs`.

## Files changed (the chunk, all on research's lists after the revision)
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
- `tests/support/home.rs`

New:
- `fixtures/claude/2.1.287/SessionStart.resume-1.json` (recorded live, 578 B)
- `viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/evidence/` (`live-round.md` and two round listings)
- `scope-record.md` (one line, `tests/support/home.rs` · companion · serves step 9 · self)

## Deviations from plan, with justifications (from the stopped run, unchanged)
- **The test set id.** `tests/support/verify.rs`'s synthetic session id `s-verify-1` became `SESSION_ID`, a literal of
  the session-id shape. Without it Run E never starts. `verify_started` gained an `after` parameter.
- **Run-count readers outside step 9's named sites moved, measured:**
  - stop-less set 5/17 (Windows 5/14);
  - no-screens 7/15 (Windows 7/12);
  - `trusted_run_typed` starts 6;
  - the run-dir case: 6 starts passing and 5 failing;
  - the record file list: +`SessionStart.resume-1.json`;
  - the dispatch-error spawn list: +1 pair;
  - the `cli_fake_agent` resume and fork cases read the `resume-1` base.
- **Caught by gates:**
  - a second `[true; 21]` verdict vector in `ledger.rs`;
  - the env and hook receipt counts, 5/37 → 6/38;
  - `dialogless_home`'s 17/14 → 18/15.

  The revision folded the last one into step 9.
- **Summary forecasts, measured:**
  - no-dialogs 18/4 (Windows 15/4);
  - no-framing 18/4 (Windows 18/1);
  - tag modal 20/2 (Windows 17/2).

  The Windows halves are read by CI.
- **The new stop case** holds Run E open on purpose: its set has no default SessionStart and a shaped `clear-1` id.
- **Byte equality held** in the drift contract's resume replay, which was step 10's condition.

## Surfaced (for the wrap's reconcile, unchanged)
1. **obs-plan wording vs the verify PTY path** (pre-existing).
   - The verify PTY runs' `finish` logs the pump's exit code for a killed child, but the rule says a child killed at
     the stop logs no `child_exit_status`.
   - The new stop case asserts only `duration_ms`, as Run B's stop case already does.
2. **Step 12 reading.** `verify_window_without_screens_fails_every_interactive_row` went 34.42 s → 42.98 s locally.
   - The cause is Run E's settle, which waits out `GATE_MAX_WAIT` with no screen, by design.
   - The CI kill is 60 s, and the mutants profile's is 45 s, so the local margin is about 2 s. No kill moved.
3. **Live readings** (`evidence/live-round.md`).
   - `--resume <id>` found Run B's session from Run E's own `-resume` project dir.
   - Run E added no transcript file in either round.

Entry 15's red was surfaced in the stopped run and is resolved by the revision. Entry 15 is green now.

## Process hygiene

| Process | Started by | Final state |
|---|---|---|
| the smoke session `p-rcl-smoke` (supervisor, wrapper, fake agent) | this run (entries 17–22) | terminated: cleanup read `processes_gone` and `endpoint_gone` true |
| `pre-push`'s test processes | this run (entry 23) | terminated: at 07:38:01Z a `/proc/<pid>/exe` scan found no process under the repository root |
| the live rounds' starts | a leg, in the stopped run | terminated (see that run's outcome) |

## Decisions & corrections
- **The operator's word** (the invocation arguments, 2026-10-16):
  - re-enter after the revision and run fmt and block entry 15 onward;
  - no live start;
  - then the operator pass with the `ci.py conclusion` read;
  - stop ready for the wrap.
- **Guard refusal:** the Bash guard refused one `cd` into the skills dir (it would have moved the cwd). The call was
  re-issued once with an absolute path.

## Next
The operator pass. In the plan, entry 24 is hygiene, fired as written. Then comes the pre-CI commit, then entry 25,
the push through `--operator 25`, then entry 26, `ci.py conclusion`, fired as written. After that, the wrap.
