# New groups — test-plan · unit-test-strategy

## Verdicts
- fanout-test-plan--cascade-g1.md 9 · distinct · line 19 names the same four snapshot readers but states no replayed-snapshot unit case for any of them, and no other group holds these four cases.
- fanout-test-plan--cascade-g1.md 11 · distinct · it replaces its own near line 23; no other group states the read rule, `unix_read_verdict`, `windows_read_verdict` or `READ_RIGHTS` (line 24's synthetic-SDDL descriptor cases are the write-side check, not these tables).
- fanout-test-plan--cascade-g1.md 14 · distinct · no group states `check_home_tree_*`, `check_diagnostics_*` or `strict_instance_holds_the_content_folders_to_the_read_rule`; line 25's `strict_instance_*` list names five widened modes, a written tree, a missing instance directory and an unstattable path only.
- fanout-test-plan--cascade-g1.md 15 · distinct · it replaces the "Not covered" sentence on its own near line 25; line 24's "read grant on `diagnostics/`" is a synthetic descriptor case, not the `cfg(windows)` real-tree cases this proposal records.
- fanout-test-plan--cascade-g1.md 16 · distinct · no group states the stamps or pin strict-modes cases; line 32's version gate against stamps is another fact.
- fanout-test-plan--cascade-g1.md 17 · distinct · no root-bin group states the `src/obs.rs` diagnostics-open refusal or its three cases, and proposal 14 states the viola-state check, not its obs.rs callers.
