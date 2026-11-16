# Outcome — 2026-11-16-wait-wake-condition · run 2026-11-16T01-45-25-implement (re-entry on plan rev 6)

**Outcome: surfaced.** Two of step 9's six remove-the-guard controls read `timeout`, not `holds`. Reaching `holds`
would move a plan step or entry (step 9, entry 4 or the module's existing tests). Per the operator's direction ("A
red that moves a plan entry: stop BEFORE the operator pass"), this run stopped before the operator pass: no hygiene
entry, no pre-CI commit, no push, no CI read. No matrix ref was set, and no chunk report was written (a surfaced run
writes none).

## The gap
- **Plan:** step 9 and the acceptance «Controls `held-turn`, `held-turn-e2e`, `no-false-wake` and `no-false-wake-e2e`
  each read `holds`»; the four wake controls are «fired on entry 4 or 6 as written».
- **Measured** (`.andromeda/cache/chunks/2026-11-16-wait-wake-condition/evidence/control-1.txt`):
  `control: 4 holds · 2 timeout · 0 does not hold · 0 unreproduced · clone removed`.
  - `held-turn-e2e`, `nofalsewake-e2e` (the plan's `no-false-wake-e2e`), `copy-not-link`, `one-stamp`: holds.
  - `held-turn` (entry 4): **timeout** at the tool's 1800 s bound. Every case of the guard read FAIL in under 0.1 s
    (the four `wait_a_turn_end_wakes_whatever_its_background_tasks` rows). The bound fired on two PRE-EXISTING tests
    of the module, `wait_a_line_appended_and_signalled_during_the_park_wakes_it` and
    `wait_a_signalled_log_only_line_parks_again`, each blocked on an unbounded `parked.recv()`. Entry 4 (default
    nextest profile, no slow-timeout kill, whole-module filter) runs them.
  - `no-false-wake` (entry 4): **timeout**. The guard's own case,
    `wait_armed_at_a_task_free_end_parks_through_a_notification_and_a_harness_turn`, HUNG rather than failed. With
    `activity` in the wake set the wait returns, so `parked.recv().expect("parked after the notification")`
    (`src/run/wait.rs:584`) blocks forever. `wait_a_signalled_log_only_line_parks_again` hung the same way.
- **What would close it** (phase's or the operator's call; implement authored none):
  - (a) the step-2 test (and, for `holds` on entry 4, the module's two existing tests) waits with
    `parked.recv_timeout(..)`, so a false wake fails instead of hanging. This edits pre-existing tests outside steps
    1–2.
  - (b) step 9 fires the unit controls on an entry filtered to the guard's own cases, with a bound.
  - (c) the acceptance reads `timeout` with every guard case FAIL as proof for `held-turn` (its cases did read
    FAIL). This is not available for `no-false-wake`, whose case hung.

## Gates (this run)
- Step 8's split record: `class-wall-read.py` extended, re-read over the standing `target/class-wall/` (no
  re-measure). Window 3 tests, max 43.164 → 43.358 s · rest max 10.996 → 10.913 s
  (`send_window_no_prompt_submitted_refuses_and_logs`) · path1 6.593 → 2.385 s · wall 56.069 → 48.381 s · tests
  321 = 321.
- `entries 15 · green 4 · red 0 · recorded 0 · timeout 0 · not-run 11` — 9 green (`last line True`) · 10 green ·
  11 `run --unit` green 17.09 s · 12 `pre-push` green 89.17 s.
- Entries 1–8: green in the stopped run `2026-11-16T01-25-19-implement` (its outcome file). They were not re-fired:
  no source changed since, only the evidence read script.
- Not run: operator entries 13 (hygiene), 14 (push), 15 (`ci.py conclusion`).

## Files changed (whole chunk, unchanged since the stopped run)
`src/run/wait.rs` (tests) · `fixtures/fake-scripts/wait-held-turn.json` (new) · `tests/cli_wait_last.rs` ·
`tests/support/home.rs` · `tests/cli_verify.rs` · `.config/nextest.toml` (comment). This run edited only
`{work_dir}/evidence/` (`class-wall-read.py`, `class-wall.json`, `control-1.txt`, `journal.md`) and the run dir.

## Deviations
- The control id `no-false-wake-e2e` → `nofalsewake-e2e`: the tool's id form caps at 16 chars (first firing
  refused, exit 2, nothing made).
- The stopped run's deviations stand (absorbed by plan rev 6, `.andromeda/runs/2026-11-16T01-41-00-phase/`).
- Scope: `scope: clean — changed 6 · listed 6 · recorded 0 … excluded 238`. Record line:
  ``- `tests/cli_verify.rs` · in-intent · serves step 6 · self``.

## Process hygiene
| process | started by | final state |
|---|---|---|
| nextest / cargo / viola / fake agent of entries 9–12 and the controls (clone `~/.cache/andromeda-control/275cfae1`) | this run | terminated. Census `gate.py census --comm viola --comm cargo-nextest --comm viola-fake-agent --args projects/viola/target` at about 2026-11-16T02:51Z: 7 running, none of this repository's `target/` (the product install `~/.local/viola-product`, the lab prototype's builds, `~/.viola/bin/0.1.0-…` mcp). The clone was removed by the tool |

## Decisions & corrections
- The `gate.py control` id form (≤ 16 chars) refused the plan's `no-false-wake-e2e`. Nothing was made, and the run
  re-fired under `nofalsewake-e2e`. Planlint did not catch the 17-char id: a plan hazard.
- A control fired on an entry with no per-test kill waits out the tool's 1800 s default per hanging firing (two
  firings ≈ 1 h of wall). A wake control on a park test needs a bounded wait or an entry `timeout`.
- The Bash guard refused a `cd` into the evidence dir at the start; re-issued with absolute paths.
- The operator's direction (no live start, no mutation run, stop before the operator pass on a plan-moving red) was
  followed.
