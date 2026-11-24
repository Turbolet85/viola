# P5 review card 2 — «Switching a session's model and effort» (`2026-11-24-switching-a-session-s-model-and-effort`)

The plan is `doc:2026-11-24-switching-a-session-s-model-and-effort/plan` rev 5. Its render is at
`.andromeda/cache/docs/2026-11-24-switching-a-session-s-model-and-effort/plan.md`, for an editor:
- no plan stands in a chunk's folder;
- writing in that file changes no record until a `put`;
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-24-switching-a-session-s-model-and-effort`
  is a diff of the chunk's records.

The review of card 1 (`.andromeda/runs/2026-11-24T12-23-47Z-phase/p5-card-1.md`) is folded. The word is snapped as
`inputs#I3`; its authority is the delegate overseer.

## What the review changed
1. **F2 and F4:** answered as proposed, on the delegate's technical word.
2. **F6 is now his fork** (`founder-forks.md` Block 2):
   - arm 1, a new `RESPAWN_REAP_BOUND` of 10 s, TAKEN PROVISIONAL, its a11y row a row of its own beside the two «Eight
     time bounds» rows;
   - arm 2 priced beside it: reuse the pump's existing 2 s `KILL_WAIT`, no new bound.
3. **Check 7: the code-graph query is recorded now.** The trace is
   `tree-query-2026-11-24-switching-a-session-s-model-and-effort.json`, 2 queries on the rust plane.
   - **Found:**
     - `pump_with_paste`'s three product callers: `pump` @ `pump.rs:293`, `pump_child` @ `src/cmd/run.rs:823` and
       verify's `Run::spawn` @ `src/cmd/verify/typed/run.rs:152`. The last is a touchpoint card 1 missed, now in
       Files to modify.
     - `pump_child`'s three callers: `run` @ `run.rs:305`, `revive` @ `revive.rs:149` and `adopt` @ `adopt.rs:107`.
     - `kill_and_reap`'s one: the panic arm @ `pump.rs:381`.
   - **New in the plan's Implementation notes:** «The pump's callers», which says what each caller relies on that a
     pump outliving its child changes:
     - **exit propagation:** only the last child's exit reaches `pump_child`, whose mapping and its three callers are
       unchanged; the old child's exit is logged by the switch;
     - **the reaper:** the panic arm kills the CURRENT child the pump owns, and the respawn has its own reap;
     - **the wheel and dialog state:** turn and dialog are clear by the refusal rungs; the holder and typed mark are
       kept; the wakes count is cleared as a `session-start` clears it; the cut mark is cleared by the new
       `session-start`;
     - **the old child's late hook events:** a hook frame names no child. An old `session-end` of the switched id
       arriving AFTER the new `session-start` is screened by a switch mark and not appended; one arriving before it
       is appended. A new unit case is added. Whether the real CLI fires SessionEnd on SIGHUP is UNMEASURED, carried
       with R6a.
4. **Correction found while reading the pump:** criterion 6 said a `NotEnded` read «leaves the session as it was».
   That is false: the SIGHUP has been sent. It now reads that nothing is spawned or appended and the switch is refused
   `child-not-ended`. The old child may still end later, the wrapper then ending with the OLD pair recorded. F1's
   pricing carries the same correction in Block 2.

## Goal (unchanged in substance)
`viola switch <name> [--model M] [--effort E] [--json]` switches a RUNNING session by an in-place restart.
- The session's own wrapper serves a channel method `switch`. It refuses unless the session is idle and the human is
  off the keys.
- It ends its OWN child (SIGHUP, reaped within 10 s, PROVISIONAL), appends one `launch` line, and respawns
  `claude --resume <id> --model M --effort E` in the same PTY.
- A later `revive` or `up` replays the switched pair. Nothing is written under `~/.claude*`, nothing is typed into the
  session, and there is no MCP tool.

**His forks, each PROVISIONAL:** F1 arm 1, F3 arm 1, F5 arm 1, F6 at 10 s, and F7 with no split. F8, the live
readings, is carried.

## Touchpoints
- **New:** `src/cmd/switch.rs`, `src/run/switch.rs`, `tests/cli_switch.rs`.
- **Modified:** `crates/viola-pty/src/pump.rs`, `crates/viola-pty/src/lib.rs`, `src/cmd/run.rs`,
  `src/cmd/verify/typed/run.rs` (new on this card), `src/run/mod.rs`, `src/cmd/mod.rs`, `src/human/hints.rs`,
  `tests/cli_output_plain.rs`, `tests/contract_error_bodies.rs`, `tests/cross_exit_causes.rs`,
  `schemas/diag-line.v1.json`.

## Test commands (18 entries, unchanged from card 1; the baselines stand, no `run` moved)
- **Unit:** viola-pty 40/40 green · the root binaries 1370/1370 green · `::switch_` red-vacuous (0 of 1370).
- **Integration:** `^switch_` red-vacuous (0 of 4359) · the readers 168/168 green.
- **Host read:** recorded CONTENDED.
- **Layers:** the harness's three.
- **Operator pass:** the smoke trio, pre-push, hygiene, push, CI read.
- **Controls:** `sw-turn`, `sw-dialog`, `sw-launch-first`, `pty-reap-bound`.

## Capabilities — no claim (as card 1)
- v2-30 and v2-31 stay unclaimed, each with a dated notes line after the word.

## Mechanical checks — the whole set re-run on rev 5
- 1 sections present.
- 2 the modify paths exist, now 11; none is a master; no ledger note on an unverified cap.
- 3 the new paths are free.
- 4 (0) the fence parses: 18 entries, exit 0.
- 4 (2) the smoke trio stands.
- 4 (9) the baseline listing reads `seen green 5 · not seen 0 · new (key) 6 · leg 7`, and the six new entries'
  `baseline` keys stand (no `run` changed).
- 5 no placeholder.
- 7 **now consulted:** the trace is present.
- 10 inputs: 3 entries, unchanged 1, n/a 2, uncited 0, unparsed 0.
- 12 (c) 0 of 5.
- 12 (d) 4 «unchanged» claims listed (plan lines 59, 417, 419 and 435). Each was read against the steps and holds:
  each claims a behaviour unchanged where the file changes only to pass the respawn handle, or where an existing rule
  still does the clearing.
- planlint: hits 0. Research's lists: `scope lists: parsed 14 items (new 3 · modify 11 · derived 0)`.
- Validation-1: aligned with `doc:…/scope` rev 3.

## The directive's item-11 block — card 1's, plus this fold
- The union lines and `rows-union: 7 master(s) · picked 1973 of 4975 · rule any` are as on card 1. No row read was
  re-run: the review added a touchpoint, not a scope file or surface.
- **Cited rows:** the plan now also cites code sites from the graph, not rows. The cited row set is card 1's 15, 13 in
  the union and 2 found by `rec.py find`.
- **Re-spawns:** none. **[RETYPED]:** the five on card 1.
- **New non-zero exits:** none. The graph queries exited 0 (stdout whole, the trace the record).
- **Where a letter was wrong or silent:** card 1 let check 7 stand as a WARN. The letter allows it ("NOT a FAIL"),
  but the reviewer's reading — the mechanism stood, so the query was owed — is the stricter one, and it found a
  missed touchpoint and a false criterion.

## Beyond the plan
| # | Proposal the word approves | Owner |
|---|---|---|
| 1 | The founder's answers to F1, F3, F5, F6 and F7 (his forks), meanwhile built PROVISIONAL on arm 1 / arm 1 / arm 1 / 10 s / no split | the founder, through the overseer; `founder-forks.md` Blocks 1 and 2 |
| 2 | The live readings R6a (an idle switch of a real session of his, read on the next turn, his settings byte-equal, and whether SessionEnd fires on SIGHUP) and R6b (a switch refused mid-turn), not fired by this chunk | carried to «The fleet on the founder's host» |
| 3 | v2-31's console leg (the switch reached by keyboard alone) | «Console actions» |
| 4 | v2-30 and v2-31 left unclaimed, each with a dated notes line now | the ledger, P5 step 4 |

Apply? (yes / review / cancel)
