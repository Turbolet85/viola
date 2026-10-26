# Classified — test-plan · test-strategy.b1

## Answers
test-strategy#1.1 · yes · contradict · new unit modules (last.rs, pause.rs, refused_inside, the impostor rows) compile out on macOS whole, so "only the two harness tests" is no longer true
test-strategy#1.2 · no · the impostor case and held-child helper join existing crate-level suites on a real endpoint, so the suite claim still holds on the changed handlers.rs
test-strategy#1.3 · no · no path or scenario is added, so the seven paths and four scenarios keep their count and harness-booted runs; Paths 2–5 change what they assert, not how they run, and the new root negative is not a counted scenario
test-strategy#1.4 · no · the six fuzz surfaces and the corpus gate still hold on the changed client, tokio_client and hook files, which gain only a peer-pid accessor and a pre-frame check
test-strategy#1.5 · no · the five hyperfine rows and the gate are unchanged (the server check only enters the timed path), and classify still holds on the changed liveness.rs
test-strategy#1.6 · no · no chaos case is added or moved and the sysinfo pin is unchanged, so the listed fault sources still hold on the changed liveness.rs
test-strategy#1.7 · no · the mutation gate and the run --mutants harness are unchanged, and no per-chunk or CI mutation gate is added
test-strategy#1.8 · no · the contract_fake_agent_drift change only sequences agents-replay planting and adds no fixture, so its comparisons and fixture counts still hold on the changed file
test-strategy#2.1 · yes · stale · the new root binary tests/security_negatives_server_identity.rs sits outside the listed root prefixes, and the report does not show it, or the planting reorder in contract_fake_agent_drift.rs, keeps to the tests/support helpers this entity names
test-strategy#3.1 · no · the Module-IPC and Security control negatives pointers, the channel example and Scenario 1 still hold, and client.rs gains only a client-side peer-pid read
test-strategy#3.2 · no · the paste validator in viola-core, its proptest and Scenario 2 are unchanged, and send.rs still holds with only the pre-frame server check added
test-strategy#3.3 · no · the hook integration layer and the Security sweep pointer still hold, the impostor rows adding cases to the existing hook_fail_open.rs matrix, and no stdin parse changes
test-strategy#3.4 · no · the rmcp-client E2E layer and Scenarios 2, 5 and 7 stand, the impostor case is a crate-level handler case in handlers.rs, and tools.rs still holds with only the check added
test-strategy#3.5 · no · the root bin unit location and the unit and integration layers still hold on the changed root bin files, the verb changes being a pre-frame check and a hint
test-strategy#3.6 · no · the Module-disk and Scenario 1 pointers stand and E5 still exists, its re-pinned steps being E5's own text rather than this row's claim
test-strategy#3.7 · no · the process-boundary location and Scenario E2 stand, the held fake-agent child being a test-only spawn in handlers.rs with no product spawn added
test-strategy#3.8 · no · the CI gates are unchanged, the check set stays 15, and the macOS-target clippy is a host recipe, not a gate
test-strategy#3.9 · no · no property case is added or moved, so the property row's locations and the nightly fuzz job stand
test-strategy#3.10 · no · no chaos case is added or moved, so the Chaos suite location stands
test-strategy#3.11 · no · the perf rows and the gate are unchanged (the server check only enters the timed hook path), so the performance budget location stands
test-strategy#3.12 · no · the Scenarios 2 and E3 and the Cross-surface parity pointers stand, and the new impostor parity sits in Paths 2–5 and a root negative, neither named by this row
test-strategy#3.13 · no · the CI matrix builds are unchanged, and the macOS compile-outs are test selection, not a change to the matrix
test-strategy#3.14 · no · viola-agent-claude and the §7 fixtures are unchanged, so the unit and contract pointers stand
test-strategy#3.15 · no · the §6 Contract suite and §7 pointers stand, and the ETXTBSY fix only sequences planting inside contract_fake_agent_drift.rs, adding no contract case
test-strategy#3.16 · no · the §6 Error sanitization pointer and the integration and E2E layers still hold on the changed negative and hint files, the new digit-run assertions extending that section
