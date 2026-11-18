# Outcome — 2026-11-18-trust-record-row-read-live (implement run 2026-11-18T01-02-09Z)

**Outcome: surfaced.** The S0R live start read RED, the floor broken. The named candidate is K1 (K3 not excluded),
and its mend is the founder's, so it went to `founder-forks.md` F1. Implement stopped BEFORE the operator pass, as
the directive requires; no spare was spent. Every non-leg gate is green, and the four controls hold.

Next → the founder's answer to F1 (`.andromeda/runs/2026-11-18T01-02-09Z-implement/founder-forks.md`); then either
`/andromeda-phase` (a revision, if a spare is allowed) or the operator pass and `/andromeda-wrap-session`. The wrap
authors the chunk report (a surfaced run writes none).

## Files changed (14, all in research's lists; scope: clean — changed 14 · listed 14 · recorded 0)
- `crates/viola-agent-claude/src/screen.rs` — `FirstScreen` (trust · imports · box · none · exited), the classifier
  `Signatures::first_screen` (modal before box), `box_any`, `first_facts` → `RowFacts`; the tables
  `first_screen_reads_each_literal_then_the_exit`, `first_screen_facts_count_rows_and_any_box_literal`, and
  `first_screen_words_are_kebab_case`.
- `crates/viola-agent-claude/src/ledger.rs` — `TypedRun::first: [Option<FirstReading>; 6]`, `FirstReading`,
  `RUN_LETTERS`; `merge_stamp` writes `measured.first_screen` / `first_settle_ms` / `first_facts`.
- `crates/viola-agent-claude/src/ledger/test_cases.rs` — `clean_typed()` gains `first: [None; 6]`.
- `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` — `merge_stamp_writes_the_six_first_screens_their_settles_and_facts`;
  the two exact-`measured` oracles gain the three null maps.
- `src/cmd/verify/typed/run.rs` — `Run::keep_first` / `keep_first_now` / `first_settle`, `reading_of`, `with_exit`;
  `end_by_kill` / `end_by_ctrl_c` / `finish` return the reading with the exit status only for a child ended by
  the read; unit cases `with_exit_keeps_the_status_only_for_a_child_ended_by_the_read`,
  `reading_of_reads_the_feed_and_the_rows_at_the_settle`.
- `src/cmd/verify/typed/runs.rs` — A, B, F keep their reading in `typed.first`; C, D, E return theirs (E: its settle
  after SessionStart, or with none by the deadline the screen as it stands, no wait added).
- `src/cmd/verify/typed.rs` — the module comment; C, D, E's readings stored.
- `tests/cli_verify/runs.rs` — `verify_first_screens_read_trust_then_box_on_the_fake`; the no-screen case's added
  assertions (all `none`, ≥ 8 500, alive, rows 0, box no, bytes 0, last null — read on the first run) and doc.
- `tests/cli_verify.rs` — `verify_record_writes_its_trust_record_to_the_kept_config_dir_only`.
- `crates/viola-e2e/src/rig/isolate.rs` — `Scratch::make_git_root` (git's layout as files, no process) and
  `git_root_word` (read from the disk).
- `crates/viola-e2e/src/rig/mod.rs` — the V arm's `git-root:`, `rows:`, `failing:`, `screen-/settle-/facts-<x>`
  lines (`stamp_reading` → `stamp_lines`); labels `S0R`, `S0R-2`, `S0R-3`; unit cases
  `stamp_reading_reads_unread_for_an_absent_version`,
  `stamp_reading_puts_each_runs_screen_settle_and_facts_and_the_failing_ids`.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — the V rehearsal's added assertions.
- `crates/viola-e2e/src/rig/ledger.rs` — a `-3` label is a spare; `admit_counts_a_dash_three_label_as_a_spare`.
- `.config/nextest.toml` — the `verify_window_` kill 60 s → 80 s in `ci` and `mutants`, with the measured walls.

## Gates (trail `.andromeda/runs/2026-11-18T01-02-09Z-implement/gate-2026-11-18-trust-record-row-read-live.json`)
- First firing: entry 1 `red · exit 0 ✗ (exit 1)` (`cargo fmt --all --check`). Cause: the edits I wrote with python
  scripts got no formatter hook. Fix: `cargo fmt --all`; `--only 1,2` green.
- Full re-fire: `entries 19 · green 7 · red 0 · recorded 0 · timeout 0 · not-run 12` (entries 1–7 green; 8–19 legs).
- Controls (`evidence/control-1.txt`): `control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
- The 80 s kill (`evidence/kill-80s.txt`): the planted hang read `TIMEOUT [  80.005s]` under `ci` and
  `PASS [  90.004s]` with no kill. The no-screen case read 51.585 / 51.722 / 52.231 / 51.951 s under the moved kill.
- Live round (`evidence/round-012252Z.txt`): `round: STOPPED at 11 (red · lacks 'input-box-signature' ✗) · legs
  fired 1/1`. The reading is in `evidence/live-starts.md`.
- No `gate.py guard` entry in the block. No deferral. No matrix ref: the chunk owns no coverage row (0 rows),
  and no capability is claimed.

## What was surfaced
S0R: `verify-exit 1 · 7/23 · trust-record fail`, with 14 floor rows below `pass`. Runs B/C/D/F read `none`: bytes
1167–1319, alive, rows 9–10, `box_any` yes, last byte 322–651 ms. Run E: null (no resume target). Run A: `trust`
at 471 ms. This matches K1 on every column of the plan's table (the box predicate does not accept what 2.1.325
draws at 80×24 in a fresh config). Run A's own `box=yes` on the trust form shows that `box_any` cannot tell K1
from K3. Mend: the founder's (the predicate `run`'s gate shares, verify's 80×24, or a new literal / naming the
screen). F1 offers four options; the recommended one is a spare on finer closed facts.

## Deviations from the plan, with justifications
- Unit cases beyond the plan's named ones, in listed files: `first_screen_words_are_kebab_case` (screen.rs);
  `with_exit_…` and `reading_of_…` (run.rs); `stamp_reading_puts_each_runs_screen_settle_and_facts_and_the_failing_ids`
  (rig/mod.rs). Why: each new pure function needs an effect a test can observe (testing.md 2026-09-24), and
  `exited` and `exit` are proven at the unit tier only.
- The no-screen case asserts `bytes 0` and `last_byte_ms null`. These are the values its first run read (the fake
  draws nothing without `--screens`), not values typed from the plan.
- `fs-boxany`'s case is named as the one rstest row it reddens (`…::case_2_rule_alone`), so "every case read
  failed" is decidable.
- The planted-hang pair used a viola-core test in a scratch clone (a small build). The `mutants` profile's
  identical override was not fired with the plant.
- The operator pass (entries 13–19) was not run: the live entry read RED (the directive).

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig live S0R` and its `viola verify` + 7 `claude` 2.1.325 children | a leg (entry 11) | terminated: rig `census: 0` in its summary and again by hand at 01:24:24Z; the round left no survivors |
| gate entries 1–7, 8–10 (cargo, nextest, harness) | this run | terminated (each entry's tree censused by the tool) |
| control clone `~/.cache/andromeda-control/bae5bf29` and its firings | this run (`gate.py control`) | terminated; `clone removed` |
| planted-hang clone's cargo/nextest | this run | exited (script ended 01:20:00Z); the clone directory `$TMPDIR/{scratchpad}/plant-clone` LEFT — a recursive `rm` was refused by the host's deny rule; it is in the session scratchpad |
| 9 `claude` sessions on the `latest` install (`gate.py census`, 01:24:24Z) | NOT DERIVED — the host's own sessions (overseers, builders, this session) | left running: not this run's |

## Decisions & corrections
- Guard refusal: the recursive `rm` of the plant clone in the session scratchpad was refused (the host's deny rule).
  It is left and named above, not routed around.
- Recurrence of the standing learning «a scripted write fires no hook» (SKILL P1): the python-scripted edits left
  `cargo fmt` red on the first block firing.
- A spare was not spent on K1: R6 and the directive allow a spare only after a named cause whose mend is this chunk.
  K1's mend is the founder's, so the fork went to founder-forks.md and implement stopped.
