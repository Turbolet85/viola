
## 2026-10-17-end-to-end-path-binary-and-harness-legs — tests: Path 1 as one flow and Path 6's harness-session leg landed
**Section:** §6 Path 1 (As landed); §6 Path 6 (step 1, As landed); §3 → 5-command implementation (`run` step 2, the `viola_e2e::fixtures` copy)
**Change:**
- Path 1: steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs` (`path1_run_start_one_home_in_order`), in the E2E layer and on the verify-driven 20 s kill line, its helpers in `tests/support/start.rs`; wall 4.69 s (5.14 s instrumented) on the Linux dev host against the 10 s `mutants` kill. The `viola_e2e::fixtures` copy still waits for its first consumer: Path 1 is not one.
- Path 6: `boot --statusline-echo` and the harness-session form landed (was owed) as `crates/viola-e2e/tests/path6_budget_harness.rs` `path6_two_instance_release_over_a_harness_session`; step 7's leg asserts the lifted `builder` and the still-paused `overseer`, one override record for `builder` alone, the WHEEL cell, and the Unix / Windows statusline branches. Only the MCP form stays owed to "MCP server for drivers".
**Why:** Fork 1 (Path 1 on the root chain, not a hand-mirrored fixture copy) is the overseer's answer as the founder's delegate, PROVISIONAL until his Epoch 5 batch.
**Ref:** .andromeda/runs/2026-10-17T01-57-16-wrap/
