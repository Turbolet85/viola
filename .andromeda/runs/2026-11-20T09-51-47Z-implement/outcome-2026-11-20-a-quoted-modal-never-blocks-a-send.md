# Outcome — 2026-11-20-a-quoted-modal-never-blocks-a-send

**Run:** 2026-11-20T09-51-47Z-implement · **Outcome:** green (1 iteration) · smoke skipped (no `role = 'smoke'` entry; the
stamping-verify boot path ran inside the block's harness entries)

## Files changed
- `crates/viola-agent-claude/src/screen.rs` — `Signatures::option_shaped` (pub), `Signatures::box_below` (private),
  `Signatures::standing_modal` (pub); `input_box_up` and `Screen::verdict` read `standing_modal`; doc lines; unit cases
  `option_shape_reads_each_clause_boundary`, `option_shape_a_shaped_label_with_the_box_below_still_refuses`,
  `box_below_a_quoted_label_without_the_box_below_still_refuses`, `quoted_label_with_the_box_below_reads_ready`
- `src/run/gate.rs` — test helper `verified_gate_over_the_box`; `wait_ready_verified_past_a_quoted_label_above_the_box_is_ready`,
  `wait_ready_verified_over_an_option_shaped_label_above_the_box_is_input_not_ready`
- `tests/support/fake.rs` — `quoting_screen()`, synthetic
- `tests/cli_send.rs` — `send_on_a_verified_cli_goes_past_a_quoted_modal_label`

## Gates (`gate.py run`, trail `.andromeda/runs/2026-11-20T09-51-47Z-implement/gate-2026-11-20-a-quoted-modal-never-blocks-a-send.json`)
Summary: `entries 18 · green 14 · red 0 · recorded 0 · timeout 0 · not-run 4` (15–18 `leg operator`, the operator pass's).
- entry 3: 721 tests run, 721 passed · entry 4: 34 run, 34 passed (687 skipped) · entry 5: 39 run, 39 passed (682 skipped)
- entry 7: passed 91, failed 0 (baseline 90: the new E2E) · entry 8: passed 1 · entry 9: passed 1
- entry 12, the guard's pass line (from its log, whole):
  ```
  gate v1.38 · c4d66f32
  root . · base 9e2b7e7d · allow none · record 0 · plan steps 19 · listed 4
  control: plan · serves · allow — each fired on its synthetic known positive
  scope-record.md: absent — no companion is excused
  guard: allowed 0 · companions excluded 0 · changed 0 · excluded 0
  guard: clean
  ```
  No allowed and no excluded file.

## Controls (`gate.py control`, `.andromeda/runs/2026-11-20T09-51-47Z-implement/control-1.toml`; evidence `evidence/control-1.txt`)
`control: 4 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` — shape-off (entry 4), shape-off-e2e
(entry 8), below-off (entry 5), raw-presence (entry 9). One file, one call with `--timeout 2400`; each entry's own
`timeout` key (1200 for 4 and 5, 2400 for 8 and 9) is the bound that applied, as the tool reads the key over the flag.

## Watches and carries
- WATCH cry-ekw6oxvp `stop_presses_ctrl_c_again_when_the_first_is_lost`: PASS in entry 6 (1 green run this implement).
- CARRY cry-zbogma7z `rig_rehearsal_reads_the_box_empty_after_x_and_del`: PASS in entry 11; not red, so not acted on.
- CARRY cry-enjonrk7: closed by steps 1–16 as built.

## Deviations
- Plan step 19 numbers the operator pass's entries «14–17»; the block holds them at 15–18 (entry 14 is the whole e2e
  layer). The pass fires them by the block's own numbering.
- scope record: none — `gate.py scope` clean, changed 4 · listed 4 · recorded 0.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / nextest / viola-harness / viola / viola-fake-agent under `projects/viola/target` (the block's and the controls' entries) | this run (gate entries) | terminated — `gate.py census --args projects/viola/target` lists none |
| the host's overseer `claude` sessions, the prototype's and the product's `viola` | not this run | left running — the host's own; not this run's to stop |

## Decisions & corrections
- Bash guard refused a `cd` into the skill's references dir (Setup), re-issued as Read by absolute path.
- Bash guard refused a bare `grep` with a bounded repetition (P2 log read), re-issued as `/usr/bin/grep` (host-linux rule, CLAUDE.md global Shell rule).
- The directive's no-literal-in-chat rule was kept by reading every label-bearing row through a redacting scratch script.
