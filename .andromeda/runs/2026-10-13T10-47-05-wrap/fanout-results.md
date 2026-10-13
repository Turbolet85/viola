# Fan-out results — 2026-10-13-epoch-4-cleanup

Path check: before `tree a3c919fb8a6a916168c8c9381beb881392b3b66b index 435f4a4dd8c1c5d83d3066c9b88968b042f4f0ca head 3b75dc729a105468329f9dfde4ed725b2fabc8ec`; after `tree 82eca54144d565917444738d2432eeadb3953cc0`, index and head unchanged; `git diff-tree` printed exactly the seven `A` lines below. No file fixed, no twin.

architecture: 7 proposals · fanout-architecture.md
- note: no new IPC / endpoint / event / socket / port / env var / crate; the `viola-e2e` → `viola-state` edge adds no crate; the Windows-target clippy is a chunk gate only, so Build system and §Occupied Resources take no row for it.
1. apply — check 1 "Accurate this-chunk addition" (routine; plan expected amendment) — Project directory structure, the `human.rs` row → module root + `src/human/` children.
2. apply — check 1 same — Project directory structure, the `run/` `send.rs` row → module root + `run/send/` children.
3. apply — check 1 same (dependent of 1) — Build system `build-system.md:4`: the only human writer is the `human` module.
4. apply — check 1 same (dependent of 1) — [CLI Version Compatibility] `architecture.md:91`: `send_hint` cited at `src/human/hints.rs` (text re-derived: the path alone, no history in the body).
5. apply — check 1 same (dependent of 2) — [Delivery Confirmation] `architecture.md:49` @c8015: the claim / relabel / settle cited at `src/run/send/slot.rs` (text re-derived).
6. apply — check 1 same (dependent of 2) — [Delivery Confirmation] `architecture.md:49` @c9591: the local-command decision cited at `src/run/send/classify.rs` (text re-derived).
7. apply — check 1 same (plan expected amendment) — §Occupied Resources `architecture.md:452`: the scratch re-created owner-only through `create_private_dir`, the witness, the baseline's green reading not yet measured (text re-derived, shorter).

security-plan: 0 proposals · fanout-security-plan.md
- note: no detector fires; the plan's Code Patterns entry (`:607`) is an addition the orchestrator raises.
1. (raised by main, check 5) apply — routine, the plan's P5-approved expected amendment names the change — §Security Anti-Patterns › Code Patterns, the `run --mutants` row: the wipe re-creates the scratch owner-only through `viola_state::fs::create_private_dir`. A narrowing, not a widening.

design-system: 0 proposals · fanout-design-system.md
- note: no new UI element; no site names the split files or the scratch.

layout-templates: 0 proposals · fanout-layout-templates.md
- note: no new surface or region; no site.

test-plan: 4 proposals · fanout-test-plan.md
- note: §9 Lint row not proposed (no test-plan detector governs a lint gate); `:441`, `:566`, `:590`, `:1123`, `:1226`, `5-command-implementation.md:46`, `:49`, `:57`, `:142`, `:163` stand true.
1. apply — check 1 "Accurate this-chunk addition" (plan expected amendment §4) — `test-plan.md:582`: `send_budget_paused_*` at `src/run/send/budget.rs`.
2. apply — check 1 same (dependent of 1) — `test-plan.md:583`: the six send-module `trailing_newline` tests (classify 3 · relabel 2 · root 1), the two `inner_cr_is_one_lf` tests in `relabel.rs`, the eight `send_local_command_` functions in `classify.rs`.
3. apply — check 1 same (dependent of 1) — §5 `test-plan.md:655`: the board units in `src/human/board.rs`.
4. apply — check 1 same (plan expected amendment §3 `run` step 4) — `5-command-implementation.md:45`: the owner-only re-create, the Windows witness, the baseline's green reading owed to the next dispatch (text re-derived).
5. (raised by main, check 5) apply — routine, the plan's P5-approved expected amendment names the change — §9 the Lint row (`test-plan.md:1157`): the Windows-target check-only clippy as a chunk gate entry on the dev host, not a CI step or a `pre-push` stage.

obs-plan: 0 proposals · fanout-obs-plan.md
- note: no drift; `:556`, `:767`, `:973`, `:1157` stand true.

a11y-plan: 0 proposals · fanout-a11y-plan.md
- note: neither detector fires; the two stale citations are the plan's expected amendment, the orchestrator's to raise.
1. (raised by main, check 5) apply — routine, the plan's P5-approved expected amendment names the change — §4 `a11y-plan.md:543`: `board_row_caption_and_default_row_are_the_literals` at `src/human/board.rs`.
2. (raised by main, check 5, same entry) apply — §8 `a11y-plan.md:853`: the units `bay_line_*`, `board_row_*`, `instance_and_home_state_pairs_are_the_fixed_texts` at `src/human/board.rs`.

## Validate
- Opening rule: 0 rejected. The detectors' `basis` coordinates in the split files are New-text rows of the report (checked); every applied text is re-derived from the report's facts.
- Check 2 (cross-contradiction): none.
- Check 3 (intent): the frozen entry names the survivors disposed and the baseline green; the survivors left on the founder's split (`inputs#I2`, card row 2) to "Epoch 4 survivors" (`inputs#I3`), and the baseline's green reading is owed to the next `windows-mutants.yml` dispatch (`inputs#I1`): justified, so the master desc is rewritten to the actuals at the P7 flip. Scope record: `src/run/send/relabel.rs` · in-intent · serves step 5 — holds (step 5's own over-800 clause).
- Check 4 (absence): no absence claim proposed.
- Check 5 (expected amendments): 7 entries — arch tree (arch 1, 2) · arch Occupied Resources (arch 7) · test-plan §3 (tests 4) · test-plan §4 (tests 1, 2; 3 the §5 twin) · test-plan §9 (tests 5, raised) · security Code Patterns (security 1, raised) · a11y §4 (a11y 1, 2, raised). Citation dispositions: no `claim false` row.
- Check 6 (disproved claims): none in the report.
- Escalations: 0.
