# Outcome — 2026-11-28-the-fleet-on-the-founder-s-host · implement run 2026-11-28T20-33-21Z-implement

**Outcome: green**, with no smoke: no start path was touched and the plan lists no smoke entry. Step 8 is HELD and
NOT built (fork U-B2 is unanswered; inputs#I7, the implement directive). R4b was fired in its rehearsal form only. Its
live arm never ran against the operator's home or compositor.

## Files changed
- `src/human/console.rs`: `CONSOLE_KEYS` is now the 79-column line
  `q quit  jk  l last  p pause  r rel  f focus  v rev  s send  m sw  a ans  o open`. Its doc comment changed, the frame
  test's pinned bytes were updated, and the new unit test `console_keys_fit_the_80_column_floor` was added.
- `src/console/mod.rs`: `console_run_on_q_draws_the_frame_then_quits_restored` was renamed to
  `console_run_on_q_draws_the_whole_key_line_at_80_columns`. Its 80x24 literal now ends in the whole key line, with no
  `...`.
- `scripts/r4-up.sh`: the `after` phase was rewritten.
  - **Live arm** (`after <home> --granted`). It refuses `r4b-not-granted` as its first act. Then it strips `VIOLA_*`
    and checks, in order, `root-present`, `no-hyprctl`, `locked`, `no-jq` and `no-home`. Then it runs `read_after`
    over the board (with no claude on PATH) and the events-log `window` lines.
  - **Rehearsal arm** (`after --rehearse`). It plants the R4a fleet at workspaces 7 and 8, ends it, runs one
    `viola up --json`, then runs `read_after`, then cleans up.
  - **Shared.** `read_after` and `after_verdict` are shared by both arms. `plant` still refuses as its first act.
  - **Header.** It names the new usage, the verdict line and what the live arm needs present and absent.

## Symbols / APIs
- `CONSOLE_KEYS` (`src/human/console.rs`, `pub(crate)`) changed value only. Its callers are unchanged: the re-export at
  `src/human.rs` and the draw at `src/console/mod.rs`.
- The `scripts/r4-up.sh` usage gains `after <home> --granted` and `after --rehearse`.
- The new verdict line is `r4: <green|red> phase=after live=<n> placed=<p>/<w> order=<…> gone=<g>`.
- The new refusal words are `no-jq` and `no-home`.

## Schema / config
none

## Gates (the block, trail `.andromeda/runs/2026-11-28T20-33-21Z-implement/gate-2026-11-28-the-fleet-on-the-founder-s-host.json`)
`entries 21 · green 14 · red 0 · recorded 1 · timeout 0 · not-run 6`. The 6 not run are the `leg = 'operator'`
entries. Their first four belong to the operator pass. The two GATED ones are never fired here.
Entry 12 (hostwatch) was recorded as `QUIET - 0 s stalled on IO`.
The fix loop took no iterations: everything was green on the first firing.

## Controls (evidence/must-pass.md, control-width.txt, control-lock.txt)
- **width:** `holds`.
- **lock:** the guard was removed and the entry read red (`contains 'r4: red precondition=locked' ✗`). The tool's word
  is `INDETERMINATE: case r4b-locked no result line`, because a script entry has no test case. The plan's table said
  «no case: the entry itself reads red».
- **grant:** fired BY HAND in a scratch copy with a stand-in `hyprctl` (deviation 1). It read
  `r4: red precondition=locked`, so entry 7's atom does not hold: the guard holds.
- **The rehearsal's fail side:** the plan's own form read GREEN (deviation 2). A parent that is not restored reads red,
  `live=2 placed=1/1 order=unread`.

## Deviations
1. **The `grant` control was not fired through `gate.py control`.**
   - **Why:** its entry (7) runs on the host's PATH. With the grant check removed, its next precondition calls the
     real `hyprctl locked`, which is a read of the operator's compositor. inputs#I7 says «the live arm is never fired;
     … the test-scoped PATH».
   - **What was fired instead:** the same removal, in a scratch copy, with a stand-in `hyprctl` first on PATH.
2. **The rehearsal's fail side named in the plan is false.**
   - The plan says «a run that ends one wrapper fewer reads `live=2` and so is not green». Measured:
     `live=3 placed=2/2 order=parents-first gone=0`, green. A wrapper that was never ended stays live.
   - A second variant, the root ended but never restored, ALSO read green: `live=2 … gone=0`.
   - Only a parent that was not restored read red. That is recorded as the measured fail side.
3. **Spec claim disproved.**
   - Plan step 6 says «gone is the count of board items not live». The board omits gone instances (architecture ·
     contract/list-payload/liveness, «gone instances are omitted»), so `gone` counts stale items only.
   - Step 6's green rule (live ≥ 1, p = w, parents-first) does not see an instance that `up` left unrestored, as
     long as one live parent edge stands.
   - The plan was not edited. It is reported for the wrap, and it bears on the operator's U-B2 Q2 (one-session
     green): his answer may want `gone` read from `<home>/instances/` instead.
- **Scope record:** none. `gate.py scope` reads `scope: clean — changed 3 · listed 3 · recorded 0`.

## Surfaced
- Deviation 3 (above), for the wrap's reconcile and the operator's U-B2 answer. The acceptance is met as written.
- No matrix ref was written: v2-12 and v2-28 are both gated, and a rehearsal never verifies them.

## Process hygiene
| process | started by | final state |
|---|---|---|
| foot ×3 (pids 529738, 529781, 3062947) | the operator: his two overseer terminals and one shell; NOT DERIVED from this run | left running: his — not this run's |
| stand-in foot / hyprctl / viola-fake-agent / target/debug/viola of the rehearsals (entries 9, 10, the 3 hand controls) | this run | terminated: the census reads none of them (`gate.py census` · `census: 3 running`, all three the foot rows above) |

## Decisions & corrections
- A plan control that conflicts with the run's directive was fired by hand in the directive's bounds. Evidence:
  deviation 1, the evolve friction record `2026-11-28T20:41:17Z-c`.
- **Sweep hazard:** a fail-side control written as «one fewer ended» measures nothing on a heartbeat-liveness board.
  A wrapper left running stays live. Evidence: evidence/must-pass.md, item 1.
- The Bash guard refused one `cd` into the skill's references dir. It was re-issued with absolute paths.
