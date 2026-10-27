# New groups — test-plan · test-data-fixtures

## Verdicts
- fanout-test-plan--cascade-g2.md 21 · distinct · line 10 names `seed_conpty`'s hard links and owes the seeded home's DACL reading, but no standing group states that an inheritable `icacls` grant propagates through them into `target/conpty-seed/<key>/` or that a home-tree-widening test starts unseeded.
- fanout-test-plan--cascade-g2.md 22 · distinct · no standing group names the `widen` · `grant_read` · `unwiden` · `still_widened` helpers of `tests/support/home.rs` or the widened test homes as test-set data (line 13 names only `plant_statusline_source` there).
- cascade-compare-test-plan.md 2 · twin · fanout-test-plan--cascade-g2.md 21 · the same F14 fact, the seed hard-link propagation of an inheritable `icacls` grant and the unseeded start of `run_refuses_a_widened_home_tree_before_its_log`, proposed as an item after the table instead of a row.
- cascade-compare-test-plan.md 3 · twin · fanout-test-plan--cascade-g2.md 22 · the same F13 fact, the four home-widening helpers of `tests/support/home.rs` (Unix modes, Windows `icacls` with its exit asserted), proposed as an item after the table instead of a row.
- cascade-compare-test-plan.md 4 · distinct · no standing group states test-made homes through `viola_state::fs::create_private_dir`, the catch-site regular file in a viola-made home (`diagnostics/`, `ledger/`) or the per-OS oracle for a home under a regular file; line 10's three hand-write exceptions cover only its listed product files, none of which the proposal plants.
