# P5 review card 1 — 2026-11-16-wait-wake-condition, REVISION 3 (held-turn on its own entry)

**The plan to read:** `.andromeda/cache/docs/2026-11-16-wait-wake-condition/plan.md` (rev 10, rendered from the
store). No plan stands in the chunk's folder. Writing in that file changes no record until a `rec.py put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-wait-wake-condition` is a
diff of the chunk's records.

**Why:** implement run `.andromeda/runs/2026-11-16T02-56-58-implement/` stopped before the operator pass.
`no-false-wake` read `holds`, but `held-turn` read `timeout` at 120.01 s (`evidence/control-2.txt`). Revision 2's
diagnosis did not hold: after the two older tests panic, `std::thread::scope` waits for a waiter that never ends. The
word is the delegate overseer's (`inputs#I13`, `inputs#I14`). Option (b) is taken; (a) and (c) are not.

## The new entry, entry 5 (inserted beside entry 4)
- **Run:** `cargo nextest run --features fake-agent --bin viola --no-fail-fast -E "test(/run::wait::tests::wait_a_turn_end_wakes_whatever_its_background_tasks/)"`
  — entry 4's run, filtered to the held-turn guard's four cases.
- **Expect:** `exit 0` and `contains 4 tests run: 4 passed`, so a missing or misnamed case reads red.
- **Baseline (this run, on the tree as the stopped run left it):** green — `4 tests run: 4 passed, 1004 skipped`
  (`case_1_zero`, `case_2_one`, `case_3_three`, `case_4_null`).
- **Entry 4 is unchanged** and stays green. Only its note drops `held-turn`.
- **Numbering:** every entry after entry 4 now counts one higher (the operator entries are 14–16). The plan's current
  references are renumbered. The «Done before …» sections keep the numbers of their own runs.

## Step 9's one changed line
- **Before:** `held-turn` — entry 4.
- **After:** `held-turn` — entry 5, fired with `gate.py control … --timeout 120`. Its edit and case are unchanged.
  - «Holds» means those four cases ran and read FAILED, with no TIMEOUT line in the removed log.
- No other control is re-fired. `no-false-wake` (holds, 7.29 s) and the four that held before stand.
- `PARK_BOUND` and the seven bounded waits stay.
- The acceptance «all four wake controls read holds» stands.
- **Implement re-enters at entry 5**, then `held-turn` on entry 5, then the operator pass (14–16).

## The carry
- `wait_a_line_appended_and_signalled_during_the_park_wakes_it` and `wait_a_signalled_log_only_line_parks_again`
  (`src/run/wait.rs`, test module) hang once they fail. The scoped waiter thread never ends, so the test runs until
  the runner's kill.
- It is a Carry-forward, not this chunk's work. The plan's Implementation notes record it.

## Unchanged
- Cap v2-17's claim stands as written. Its acceptance names no entry number, and the revised plan still proves it. No
  re-claim and no un-claim.
- The write set is unchanged (6 files). The scope gained one `[premise-corrected]` bullet, and research gained a
  «Revision 3» section.

## Checks (step 1)
- The fence parses: 16 entries.
- `planlint`: 0 hits.
- `gate.py scope --lists-only`: parsed.
- `inputs verify`: no vanished or broken entry. `inputs#I3` (`pwait.sh`) reads **drifted**: the overseer's script has
  changed since it was snapped. It is cited only for the waiter's re-park on `background_tasks` ≠ 0, and this revision
  rests on nothing in it.

## Beyond the plan
1. (standing — `.andromeda/runs/2026-11-16T02-52-20-phase/p5-card-1.md`) **Candidate for the founder: carry
   `session_crons` as a count** (F1 option 2, not taken).
   - What a driver gains: it sees from the turn end that the CLI has a wake scheduled, so it does not treat that turn
     end as the session's last word.
   - What crosses: one count, no content.
   - Where it would land: `crates/viola-agent-claude/src/hook.rs`; `turn-ended` data keys 3 → 4; architecture,
     security-plan and obs-plan rows with Decisions Log entries.
   - Owner: the founder's own word. It stands in
     `.andromeda/runs/2026-11-16T00-29-34-phase/founder-forks.md`. No route entry is minted.
2. (standing — `.andromeda/runs/2026-11-16T02-52-20-phase/p5-card-1.md`) **The V150 fix in the overseer's waiter**
   (`pwait.sh`, `inputs#I3`): wake on every turn end, or read the message, in place of re-parking on
   `background_tasks` ≠ 0.
   - Owner: the overseer, in its own repository. It is outside viola's route and lands in no viola entry.
3. **Carry-forward: the two older wait tests hang past a failure** (`src/run/wait.rs`, test module, the two tests
   named above). The wrap pins it as a CARRY.
   - **The owner is asked.** The relay says: the first later entry that edits `src/run/wait.rs`; if none is certain,
     ask. No working-route entry names `src/run/wait.rs` today, so none is certain.
   - Name the working-route entry to carry it: for example «Send confirmation» (the next entry in Epoch 3) or
     «Version done-check» (Epoch 7). Or say a new entry, with its title and place.

Apply? (yes / review / cancel) — and the carry's owner for row 3.
