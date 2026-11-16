# P5 review card 1 — 2026-11-16-wait-wake-condition, REVISION (entry 9)

**The plan to read:** `.andromeda/cache/docs/2026-11-16-wait-wake-condition/plan.md` (rev 6, rendered from the
store). No plan stands in the chunk's folder; writing in that file changes no record until a `rec.py put`;
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-wait-wake-condition` is a
diff of the chunk's records.

**Why a revision:** `/andromeda-implement` (run `.andromeda/runs/2026-11-16T01-25-19-implement/`) stopped surfaced on
entry 9 before the controls and the operator pass; no pre-CI commit exists. The word: the delegate overseer's, relayed
(`inputs#I7`, `inputs#I8`; the operator away). No row read and no codebase research re-ran.

## Entry 9, before → after
- **Before:** `after_tests == before_tests and after_max_test_s < 20 and after_wall_s < before_wall_s` over the whole
  class.
- **Why it missed:** the slowest test of the class is
  `runs::verify_window_without_screens_fails_every_interactive_row`, 43.164 s → 43.358 s. It waits ~43 s by design,
  under its own 60 s `ci` kill (`.config/nextest.toml`, the `verify_window_` override). The work is not red.
- **After:** every stamped-home test under ITS OWN `ci` kill, the stamping out of its measured time:
  1. the slowest test outside the `verify_window_` override under 20 s (measured 10.913 s after);
  2. each `verify_window_` test under its 60 s kill (43.358 s), at least one such test present;
  3. the same test count (321 = 321);
  4. the class wall time and `path1_run_start_one_home_in_order`'s own time printed before and after (56.069 →
     48.381 s; 6.593 → 2.385 s) — report-only, no threshold on a ratio.
- **The record it reads:** step 8 now also writes `{side}_window_tests`, `{side}_window_max_test_s`,
  `{side}_rest_max_test_s`, `{side}_rest_max_test` and `{side}_path1_s`. Implement extends `class-wall-read.py` and
  re-runs it over the standing `target/class-wall/` JUnits (no re-measure; `class-wall.sh` re-run whole only if those
  files are gone).
- The (ci) acceptance criterion moves with it: the old «`after_max_test_s` < 20, `after_wall_s` < `before_wall_s`»
  becomes the four lines above.

## The control's two readings (`.andromeda/runs/2026-11-16T01-41-00-phase/p5-controls.md`)
- Baseline on the tree: **red** as predicted, exit 1, `KeyError: 'before_path1_s'` (the split record not written yet).
- In a clone at `df3bad61`, a planted record:
  - must-pass, the stop's measured split → **green**, last line `True`;
  - must-fail, rest max 21.0 → **red**, last line `False`;
  - must-fail, window max 61.0 → **red**, last line `False`.

## The lists' moves (plan touchpoints and research's two lists; `gate.py scope --lists-only`: parsed 6 items)
- `tests/cli_verify/stamps.rs` **leaves**; `tests/cli_verify.rs` **joins** — the two stamp-isolation tests stand at
  its root, since entry 7's `test(=name)` filter matches names exactly.
- `tests/support/verify.rs` **leaves** — unchanged: the stamp reaches its tmp dir through the existing
  `verify::verify`.
- Step 5 gains a step for the other-stamp sweep, with its guard: the stamp setup removes another build's
  `viola-stamp-<key16>` dir only when its `.lock` is taken by a non-blocking `try_lock`; a stamp another process holds
  (a running test's) stays, and the current build's stamp is never swept.

## Where implement re-enters
Steps 1–7 stand written; entries 1–8 read green in the stopped run. Implement re-enters at step 8's split record, then
entry 9, entries 10–12, step 9's six controls (`control-1.toml` written, not fired) and the operator pass (13–15).

## Unchanged
Goal, steps 1–4 and 6–7's content, step 9's controls, entries 1–8 and 10–15 with their baselines, and cap v2-17's claim
and concretized acceptance — entry 9 is no leg of v2-17, so nothing is re-claimed or un-claimed. The scope gains the
premise-corrected bullet and the deviations bullet; nothing else in it moves.

## Mechanical checks
`gate.py run --dry-run` 15 entries, exit 0 · `gate.py baseline` seen green 5 · new 7 · leg 3 · `inputs.py verify` 8
entries, drifted 0 (I6, the earlier P5 word, uncited by design) · `planlint` hits 0; its check 12 (d) listing read:
each «unchanged» claim holds (`tests/support/verify.rs` reads no change in `git diff --stat`). Validation-1: aligned —
the scope's corrected bound is the plan's entry 9.

## Beyond the plan
1. (standing — `.andromeda/runs/2026-11-16T00-29-34-phase/p5-card-1.md`) **Candidate for the founder: carry
   `session_crons` as a count** (F1 option 2, not taken).
   - What a driver gains: it sees from the turn end that the CLI has a wake scheduled, so it does not treat that turn
     end as the session's last word.
   - What crosses: one count, no content.
   - Where it would land: `crates/viola-agent-claude/src/hook.rs`; `turn-ended` data keys 3 → 4; architecture,
     security-plan and obs-plan rows with Decisions Log entries.
   - Owner: the founder's own word. It stands in
     `.andromeda/runs/2026-11-16T00-29-34-phase/founder-forks.md`. No route entry is minted.
2. (standing — `.andromeda/runs/2026-11-16T00-29-34-phase/p5-card-1.md`) **The V150 fix in the overseer's waiter**
   (`pwait.sh`, `inputs#I3`): wake on every turn end, or read the message, in place of re-parking on
   `background_tasks` ≠ 0.
   - Owner: the overseer, in its own repository. It is outside viola's route and lands in no viola entry.

This revision adds no row of its own.

Apply? (yes / review / cancel)
