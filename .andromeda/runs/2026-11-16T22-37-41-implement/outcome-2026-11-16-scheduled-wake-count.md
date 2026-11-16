# Outcome — 2026-11-16-scheduled-wake-count

**Outcome:** green (0 fix iterations; the block green 17/17 on its first firing) · smoke skipped — no boot-path
touchpoint and no smoke entry · Next → the OPERATOR PASS (entries 18 hygiene, 19 push, 20 ci.py conclusion), then
`/andromeda-wrap-session`.

## Files changed (8 modified · 0 new)
- `crates/viola-core/src/lib.rs` — TurnEnded `data_keys` and the `KIND_TABLE` row gain `scheduled_wakes`
- `crates/viola-agent-claude/src/hook.rs` — `Known.session_crons: Option<Vec<IgnoredAny>>`; the drift word `array`
  for `session_crons`; Stop writes `scheduled_wakes` = its length, StopFailure `null`; two new rstest tables; the
  exact-data cases gain the key
- `src/run/wait.rs` — `wait_a_turn_end_wakes_whatever_its_scheduled_wakes`
- `crates/viola-state/tests/state_events.rs` — `state_events_scheduled_wakes_is_a_known_key`
- `crates/viola-mcp/tests/handlers.rs` — `handlers_wait_carries_a_scheduled_wake_count_unchanged`
- `tests/hook_events.rs` — the booted case, the key sets, the exact data, the recorded Stop's count
- `tests/cli_wait_last.rs` — `boot_held`'s Stops gain `session_crons`; `path3_wait_carries_the_scheduled_wake_count_of_a_turn_end`
- `tests/cli_wheel.rs` — the API-error end's exact data gains `scheduled_wakes: null`

## Gates (run `.andromeda/runs/2026-11-16T22-37-41-implement`, trail `gate-2026-11-16-scheduled-wake-count.json`)
entries 20 · green 17 · red 0 · recorded 0 · timeout 0 · not-run 3 (18, 19, 20: leg operator)

## Controls (evidence/control-1.txt, control-2.txt)
- wake-count · entry 5 · holds · wake-drift · entry 5 · holds · wake-key · entry 9 · holds · wake-content · entry 10 · holds
- `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` · `control: 1 holds · … · clone removed`

## Matrix
- v2-35 · planned → implemented (refs: the seven named tests)

## Deviations
none — `gate.py scope`: clean — changed 8 · listed 8 · recorded 0. Guard entries: none in the plan.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo-nextest / test binaries / booted `viola` wrappers and fake agents of the gate block and the four control firings | this run (through gate.py) | terminated — census by `projects/viola/target/debug/viola` reads none |
| 7 `viola` processes the census lists (viola-lab prototype wrappers and waits, `~/.viola/bin/0.1.0-…/viola mcp`, `~/.local/viola-product/bin/viola wait viola-builder`) | NOT DERIVED — not this run's: other trees' builds and the bridge | left running: not this run's — their owners |

## Decisions & corrections
- The bash guard refused a `cd` into the skill's references dir (it moves the session cwd); re-issued as `cat` by absolute path — guard-refused:1.
- The operator's directive for this run (the session prompt): no live start; raise no dialog; a red that moves a plan entry stops before the operator pass; run the operator pass with the ci.py conclusion read; set no scheduled wake or timer.
