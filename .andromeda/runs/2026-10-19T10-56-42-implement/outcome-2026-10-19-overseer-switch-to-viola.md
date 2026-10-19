# Implement outcome — 2026-10-19-overseer-switch-to-viola (INTERIM: paused at the switch point, step 9)

**Outcome word:** not yet. Parts A and B are done under the prototype. The run is paused at step 9's switch point, on
the operator's direction: stop there and wait, because the overseer's O1–O7 need overseer1's review of its own tools
first. Part C (steps 10–13) and the gated Part D (step 14) re-enter under the product.

## Files changed (Part A, research's Files to modify, all six)
- `crates/viola-agent-claude/src/dialog.rs`: `option_details` in `question_data`, the two new unit cases, the data
  pins (the fixture case and the header-less case).
- `crates/viola-agent-claude/src/hook.rs`: `background_tasks` (a count) on the Stop arm through `Known`'s
  `Option<Vec<IgnoredAny>>`. A non-array value drifts with `expected:"array"`. Two new unit cases; three existing
  Stop pins updated.
- `crates/viola-core/src/lib.rs`: `TurnEnded`'s data keys and the `KIND_TABLE` pin.
- `crates/viola-state/tests/state_events.rs`: the `every_kind` rows for both keys, and
  `state_events_background_tasks_and_option_details_are_known_keys`.
- `tests/hook_events.rs`: `hook_a_recorded_stop_carries_its_background_task_count` and
  `hook_a_recorded_question_carries_its_option_details`; the existing all-hooks case's turn-ended pins.
- `src/run/dialog.rs`: the `question` frame pin.

New evidence files: `evidence/switch-round.md` (steps 7–9) and `evidence/profraw-red.md` (the open red).
`switch-readings.ndjson` comes at step 10.

## Gates (run dir `.andromeda/runs/2026-10-19T10-56-42-implement`, trail `gate-2026-10-19-overseer-switch-to-viola.json`)
- Entries 1–18 green on the first firing.
- Entry 19 (`pre-push`):
  - First firing: red (`19.log`). 2958/2958 tests passed, then `llvm-cov` refused one truncated `.profraw` of a
    `viola hook`. It is OPEN, its killer not known (`evidence/profraw-red.md`).
  - Second firing: green (`19.2.log`).
- Entries 20–22 are the operator pass's. 23–24 wait for the evidence of steps 8 and 10–12. 25 is gated.
- WATCH `send_budget_paused…` case_4: PASS in entry 5, entry 19's first firing and `19.2`. Three nextest runs, no
  plain `cargo test`.

## Part B
- Step 7: `target/release/viola` sha256 `231c0225…db52e3755`.
- Step 8: `stamped 2.1.296  24 pass  0 fail` in `~/.viola`; live starts 6 of 8 spent.
- Step 9: the hand-off is written.

## Deviations
- **Step 8's mode reading:** `~/.viola` reads `755`, not `700`. The verify opens no home log, so the narrowing the
  plan attributes to it happens at the first `run` start. It is read at step 10. Recorded in `switch-round.md`.
- **Step 5's fixtures:** they carry no option `preview` and an empty `background_tasks` array, so the fixture cases
  read null previews and a count of 0. The non-null values are held by the unit cases.
- **The per-start ledger** is one clock reading before the verify command, not one before each of its six internal
  spawns.

## Process hygiene
| process | started by | final state |
|---|---|---|
| smoke session `p-osv-smoke` | a leg (gate entries 13–18) | terminated (entry 18: `processes_gone`, `endpoint_gone`) |
| `viola verify` and its six `claude` children | this run (step 8) | terminated (exit 0) |

`gate.py census` (this repo's `viola` debug and release builds, `target/harness`, the fake agent, the harness):
none running.

## Decisions & corrections
- The operator's direction (the `/andromeda-implement` arguments, this session): a cap of eight live claude 2.1.296
  starts; Parts A and B under the prototype; stop at step 9's switch point and wait; no mutation run.
- The Bash guard refused one `cd` into `target/llvm-cov-target`; it was re-issued as a subshell.
