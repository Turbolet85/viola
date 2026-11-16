# P5 review card 1 — 2026-11-16-wait-wake-condition, REVISION 2 (step 9 / entry 4)

**The plan to read:** `.andromeda/cache/docs/2026-11-16-wait-wake-condition/plan.md` (rev 8, rendered from the
store). No plan stands in the chunk's folder; writing in that file changes no record until a `rec.py put`;
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-wait-wake-condition` is a
diff of the chunk's records.

**Why:** implement run `.andromeda/runs/2026-11-16T01-45-25-implement/` stopped before the operator pass —
`held-turn` and `no-false-wake` read `timeout` on unbounded `parked.recv()` waits. The word is the delegate
overseer's (`inputs#I10`, `inputs#I11`). Option (a) taken; (b) and (c) not.

## Step 2 (the park waits), before → after
- **Before:** the module's tests wait on `parked.recv()` with no limit.
- **After:** every wait on `parked` in `src/run/wait.rs`'s test module takes `recv_timeout(PARK_BOUND)`.
  - **The bound: `PARK_BOUND`, one constant of the test module, 5 s.** Its comment names what it guards: a wait that
    should park but returned. A healthy park signals in milliseconds.
  - It reaches the step-2 test and the module's two existing tests, `wait_a_line_appended_and_signalled_during_the_park_wakes_it`
    and `wait_a_signalled_log_only_line_parks_again`.
  - Test-module lines only; `wait`'s product code is unchanged.

## Step 9 (the controls), before → after
- **Before:** the four wake controls fire on entry 4 or 6 as written, at the tool's 1800 s default.
  `no-false-wake-e2e` is spelled with 17 characters.
- **After:**
  - `held-turn` and `no-false-wake` are re-fired after step 2's change, each under `gate.py control … --timeout 120`.
  - «Holds» means the guard's own cases ran and read FAILED, the line reads `holds`, and the removed log has no
    TIMEOUT line.
  - `held-turn-e2e`, `nofalsewake-e2e`, `copy-not-link` and `one-stamp` already read `holds` and are not re-fired.
  - The plan spells `nofalsewake-e2e` as fired.
  - The acceptance «all four wake controls read holds» stands.

## Entry 4, before → after
- **Run command:** unchanged.
- **Note:** now names the bounded park waits and the `--timeout 120` re-firings.
- **Baseline before:** «green, 43 passed, new rows not written».
- **Baseline after:** «green — exit 0, 48 tests run: 48 passed». This is this run's baseline run on the tree as the
  stopped run left it.

## Beyond the plan
1. (standing — `.andromeda/runs/2026-11-16T01-41-00-phase/p5-card-1.md`) **Candidate for the founder: carry
   `session_crons` as a count** (F1 option 2, not taken).
   - What a driver gains: it sees from the turn end that the CLI has a wake scheduled, so it does not treat that turn
     end as the session's last word.
   - What crosses: one count, no content.
   - Where it would land: `crates/viola-agent-claude/src/hook.rs`; `turn-ended` data keys 3 → 4; architecture,
     security-plan and obs-plan rows with Decisions Log entries.
   - Owner: the founder's own word. It stands in
     `.andromeda/runs/2026-11-16T00-29-34-phase/founder-forks.md`. No route entry is minted.
2. (standing — `.andromeda/runs/2026-11-16T01-41-00-phase/p5-card-1.md`) **The V150 fix in the overseer's waiter**
   (`pwait.sh`, `inputs#I3`): wake on every turn end, or read the message, in place of re-parking on
   `background_tasks` ≠ 0.
   - Owner: the overseer, in its own repository. It is outside viola's route and lands in no viola entry.

This revision adds no row of its own.

Apply? (yes / review / cancel)
