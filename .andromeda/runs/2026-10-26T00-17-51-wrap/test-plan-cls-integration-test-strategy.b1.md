# Classified — test-plan · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · no · the direct crate-API calls and the cargo-nextest and rstest 0.27.0 tools hold: the new cases call viola-channel directly and Cargo.lock is unchanged
integration-test-strategy#1.2 · no · the concurrent-append and tempfile 3.27.0 claims hold: no product behaviour moved and Cargo.lock is unchanged
integration-test-strategy#1.3 · no · the fake agent and its .cmd shim fixtures are unchanged: no fixture, script or seam was added
integration-test-strategy#1.4 · yes · add-to · the Windows pipe DACL's effect is now asserted, a remote-client bit read and an anonymous-open denial (F1, F3) on test pipes named per case (F10), with windows-sys' two root dev features (F8)
integration-test-strategy#1.5 · no · the ConPTY, pump and sideload claims hold: no product behaviour moved, the reached run.rs changed only its unit captures, and Cargo.lock keeps portable-pty =0.8.1
integration-test-strategy#1.6 · no · the hook-exec, channel and budget.json assertions hold: no product behaviour moved and Cargo.lock keeps assert_cmd 2.2.2
integration-test-strategy#2.0 · no · the home, seed and stamped_home claims hold: no product behaviour moved, the reached statusline.rs, run.rs and gate.rs changed only their unit captures, and the readiness stall report is untouched
integration-test-strategy#3.1 · no · the channel verb, paste and budget-gate claims hold: no product behaviour moved and none of its named test files is reached
integration-test-strategy#3.2 · no · the strict-modes refusals and the statusline arm's no-read, no-open, no-line claim hold on the changed statusline.rs: no product behaviour moved and its change is in the unit captures only
integration-test-strategy#4.1 · yes · stale · src/obs.rs is reached whole, its scoped captures moved to the helper, and this entity cites its unit config_keys_are_the_config_schema_properties by file, which no report line re-asserts
