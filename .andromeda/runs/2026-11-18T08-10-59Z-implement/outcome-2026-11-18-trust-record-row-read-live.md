# Outcome — 2026-11-18-trust-record-row-read-live (implement run 2026-11-18T08-10-59Z, plan rev 8, Revision 3)

**Outcome: surfaced.** Re-entry at step 21. Steps 21–24 are built and green on the fake, and the `rec-hiddengate`
control holds. **S0R-3**, the last live verify of the cap, was ledgered at 2026-11-18T08:27:05Z on the R6 ledger
(`--cap 1 --spare 2`), and the round read COMPLETE. It ran with `keys: none` and `stop: none` (verify exited by
itself). It was not refused for the token window. Step 26's two lines:
- `failing:` names floor ids, so the **floor is BROKEN**;
- `kept: b,c,d,f`, so it reads **GREEN as a diagnostic reading**, never verification.

The four kept first screens are **one modal, a `browser-tools-prompt`**: 2 options, the 1st selected, no owner act,
no compiled literal of ours matches it, no folder named, no plugin. A mode is named on B, C and F, not on D. It
selects **the ORDER** of the three differences, not the trust path. Its mend is the founder's
(`founder-forks.md` in this run dir). Implement **stopped before the operator pass**. **The cap is exhausted**, and no
further live start was made. No timer or scheduled wake was set.

Next → the founder's answer to the fork in `.andromeda/runs/2026-11-18T08-10-59Z-implement/founder-forks.md`, under a
new cap; then `/andromeda-phase` (a revision) or `/andromeda-wrap-session`. A surfaced run writes no report: the wrap
authors it.

## Files changed this run (8, all in research's lists; scope: clean — changed 19 · listed 19 · recorded 0)
- `crates/viola-agent-claude/src/ledger.rs` — `TypedRun::first_rows: [Option<Vec<String>>; 6]`. It holds the rows of
  each run's first-reading settle in memory only: no `measured` key and no row reads it.
- `crates/viola-agent-claude/src/ledger/test_cases.rs` — `clean_typed()` gains `first_rows`.
- `src/cmd/verify/typed/run.rs` — `Run.first_rows` is set in `keep_first` beside the reading. The new
  `pub(super) struct First { reading, rows }` is what `end_by_kill`, `end_by_ctrl_c` and `finish` return now.
- `src/cmd/verify/typed/runs.rs` — `keep(typed, i, First)` stores both. `dialog_run`, `plan_run` and `resume_run`
  return `First`.
- `src/cmd/verify/typed.rs` — C, D and E go through `keep`; the module comment names the held rows.
- `src/cmd/verify.rs` — the new hidden flag `--record-first-screens` (`hide = true`). On a failed verify under
  `--record` with that flag, `record_first_screens` and `first_screen_files` write `FirstScreen.<x>.json`
  (`{"v":1,"run","screen","cols":80,"rows"}`) through `write_recording`. They take each run whose word is not `box`
  (and, for Run A, not `trust`). The rows stay whole through `ledger::scrub`, then every row and seam goes through
  `ledger::screen_fault`, and an unclean set is refused whole (`refuse_recording`). `home_and_user` is a helper. The
  unit cases: `first_screen_files_take_each_failed_run_and_no_box` (7 cases),
  `first_screen_files_write_a_poisoned_runs_rows_null`,
  `first_screen_scrub_keeps_a_temp_path_and_rewrites_the_home` and `verify_help_names_no_first_screens_option`.
- `.gitignore` — `fixtures/claude/*/FirstScreen.*.json`.
- `tests/support/verify.rs` — `verify_started` takes `before` (handed to `verify_args` and `user_tie`). The new
  `pub struct Place { cwd, tmp, name }` replaces three parameters; clippy's `too_many_arguments` fired at eight.
- `tests/cli_verify.rs`:
  - the four `verify_started` calls pass `&[]` and a `Place`;
  - `verify_record_writes_its_trust_record_to_the_kept_config_dir_only` adds that no `FirstScreen.` file is written;
  - new: the helper `failed_record_verify`,
    `verify_record_on_a_failed_run_writes_no_screen_without_the_rig_option`,
    `verify_record_first_screens_on_a_failed_run_writes_its_first_screen_only` (its 24-row oracle copied from the
    first run) and `verify_stopped_with_record_writes_no_first_screen`.
- `crates/viola-e2e/src/rig/isolate.rs` — `Scratch::record_dir` (`<root>/record`) and
  `Scratch::keep_first_screens(kept)`. The latter copies each `FirstScreen.<x>.json` (read through `MAX_FRAME`,
  parsed) into the run's kept folder through `write_private` (0600) and returns `b,c,…` · `none` · `unread`.
- `crates/viola-e2e/src/rig/mod.rs` — the `V` argv gains `--record <scratch>/record --record-first-screens`. Before
  the teardown, `kept:` is printed from `keep_first_screens(<out>/<run>)`.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — the V rehearsal gains `kept: none`. New:
  `rig_rehearsal_v_keeps_a_failed_runs_first_screen_locally` (fake told `{config}/elsewhere.json`: `verify-exit 1`,
  `kept: f`, only `FirstScreen.f.json` in the kept folder, `run` `f`, `screen` `trust`, torn down whole).

Steps 1–20's files from runs 01-02-09Z, 01-39-23Z and 06-24-01Z stand uncommitted beside these.

## Gates (trail `.andromeda/runs/2026-11-18T08-10-59Z-implement/gate-2026-11-18-trust-record-row-read-live.json`)
- First firing: `entries 19 · green 6 · red 1 (2)`. The red was clippy (`gate.py show --n 2`): `too_many_arguments`
  on `verify_started` (8/7). After that was fixed, a local clippy also read `collapsible_if` twice in the new
  `verify.rs` code. Fix: `Place`, and two let-chains.
- Second firing: `entries 19 · green 7 · red 0 · recorded 0 · timeout 0 · not-run 12`. Entry 5 ran 80 cases and
  passed them all (its log's `nextest-integration`).
- Control (`evidence/control-4.txt`): `control rec-hiddengate · entry 5 · holds — in place green, removed red · exit
  0 ✗ (exit 1)`, then `control: 1 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (`--timeout 900`).
- Scope `clean`, then hygiene (refused 1, respelled on the word, then `clean`), then the land (tx 2627), all before
  the start, as step 25 asks.
- Live round (`evidence/round-082702Z.txt`): `round: COMPLETE · legs fired 1/1 · entries 19 · green 5 · red 0`.
  - The install was read by path: `2.1.325 (Claude Code)`.
  - The census read 0 before and after.
  - The build was `f654a8b51b07f334`, the same as the summary's `viola-build`.
  - Entry 11's 35 atoms all held, the artifact `live-starts.ndjson` fresh.
- No `gate.py guard` entry. No deferral. No matrix ref: the chunk owns no coverage row (0 rows), and no capability is
  claimed.
- Smoke (P3): skipped. The plan's smoke entries 13–15 are operator legs, and the directive stops this run before the
  operator pass.

## What was surfaced (`evidence/live-starts.md` §S0R-3)
- **The reading:** `verify-exit 1 · 7/23 · trust-record fail`, the same 16 failing ids as S0R and S0R-2.
  - A: `trust` at 459 ms.
  - B, C, D, F: `none` at 8 500 ms, each `lit=rule@1 adj=no first=rule col=80`, with facts equal to S0R-2's but for
    `last`.
  - E: null (no resume target).
  - `kept: b,c,d,f`.
- **The screen, in closed words:** `browser-tools-prompt` on all four runs.
  - 2 options; the 1st (keep the tools off) is selected.
  - Owner act: no. Compiled literal: no.
  - Folder: none. Plugin: no.
  - Mode: yes on B, C and F; no on D (D's body differs only in its mode sentence).
- **What raises it:** the ORDER. The screen is identical whatever the run's folder (B in the seeded root's
  subfolder, F in a temp dir), and S0C's plain child under the same config never met it, so something verify runs
  before B arms it under the private config. Not measured: whether that is the print probe or Run A. The trust path is
  not selected.
- **The gap:** on a host where the CLI detects a browser extension, 2.1.325 raises this modal at an interactive start
  after earlier starts. No compiled modal literal knows it (security `decision/input-box-readiness-from-literals`;
  architecture `decision/screen-signatures-compiled`). So verify reads `none` and fails 16 rows, and `run`'s gate
  would wait out its bound on it. The mend is the founder's; the fork is in `founder-forks.md`.

## Deviations from the plan, with justifications
- `tests/support/verify.rs`: the plan says `verify_started` «takes the before-args it hands `verify_args` (its three
  callers pass `&[]`)». With the new stop case there are four callers. The eighth parameter tripped clippy's
  `too_many_arguments`, so `cwd`, `tmp` and `name` became one `Place` (every caller passed the same `"verifier"`);
  this was not silenced with an allow. `before` also reaches `user_tie`, so a `--record` start keeps the real `HOME`
  as `verify` does.
- `first_screen_files_take_each_failed_run_and_no_box` adds two rows beyond the plan's list: F's `trust`, which
  writes, and E's `exited`. The poisoned run became a case of its own, `first_screen_files_write_a_poisoned_runs_rows_null`.
- The split-home seam case uses a home of `/data/alice`. A `/home/<user>` home split mid-name reads `absolute-path`
  on the row alone before the seam, so it could not show `home-path` at the seam.
- A hygiene row outside this run's own files, on the operator's word (inputs#I15; the overseer's technical answer,
  2026-11-18T08:26:29Z): in `.andromeda/runs/2026-11-18T07-42-29Z-phase/fold-review.py:30`, the rig's
  synthetic scratch path, spelled with the uid's digits, became `/tmp/viola-rig-<uid>/r/cwd`; nothing else in that file changed. In this run's `relay-1.md` the question line
  was spelled the same way; the verbatim copy is inputs#I15.
- Scope record: none. `gate.py scope` read `clean` with 0 recorded.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig live S0R-3`, its `viola verify` and its `claude` 2.1.325 children | a leg (entry 11) | terminated: the rig's summary read `census: 0`; entry 12's census read `census: 0`; the round reported no survivors |
| gate entries 1–7, 8–10 and 12 (cargo, nextest, harness, rig census) | this run | terminated (each entry's tree censused by the tool) |
| control clone `~/.cache/andromeda-control/c1c3a01c` and its firings | this run (`gate.py control`) | terminated; `clone removed` |
| local `cargo check` / `clippy` / `nextest` runs in steps 21–24 | this run | exited (foreground calls) |
| `gate.py census --comm viola-live-rig --comm viola-fake-agent --comm cargo-nextest --args projects/viola/target/debug/viola --args viola-rig- --args andromeda-control` | — | `census: none running` |

## Decisions & corrections
- The directive was followed:
  - ONE live start, S0R-3, on the R6 ledger, ledgered with the clock's time first (08:27:05Z);
  - `keys: none`, the stop printed beside it;
  - stopped before the operator pass; no further live start;
  - each kept screen named in closed words only (the files read in place, never copied, quoted or committed);
  - no scheduled-wake or timer tool called.
- The hygiene refusal of the earlier phase run's `fold-review.py` held the land. On the overseer's word (inputs#I15)
  that one path was respelled, hygiene re-read `clean`, then the land and the start.
- Guard refusal: the Bash guard refused a `cat` heredoc into a scratch script. It was re-issued once through the
  Write tool and run by path (host-linux.md 2026-09-29).
- The scripted edits got no formatter hook. `cargo fmt --all` was run before the first firing, so the standing
  learning «a scripted write fires no hook» did not recur.
