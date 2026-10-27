# Classified — arch · occupied-resources.b2

## Answers
occupied-resources#26.1 · no · the home strict-modes reconciliation of board::scan against check_instance is owned by "Unix endpoint and home hardening" (Not needed), and board.rs is not reached
occupied-resources#27.1 · yes · the budget.json read through read_budget binds, with its take(MAX_FRAME + 1) cap, tolerant parse, v 1 and every failure no reading (Needs, budget.rs reached)
occupied-resources#28.1 · yes · the stamps readers' MAX_FRAME cap (over the cap an error) and run's version gate recording cli_verified:false on a refusal bind (Needs, stamps.rs and version_gate.rs reached)
occupied-resources#29.1 · yes · read_statusline's take(MAX_FRAME + 1) reads, its 16-byte shell cap and the capture arms' stdin bounds bind (Needs, verify.rs reached)
occupied-resources#30.1 · no · verify's probe-dir lifecycle, guard removal and stop handler are probe-run design, Not needed beyond the capture-arm bounds that 29.1 carries
occupied-resources#31.1 · no · the root-test Watch bound and its report are a convention each new root wait owes, and the unit-level proof adds no root wait to the count
occupied-resources#32.1 · no · standing S1 · each new test runs in its own viola home under target/e2e-home/viola-test-*/home, a convention the task lists
occupied-resources#33.1 · no · standing S5 · a new fixture is scrubbed and hygiene-walked, with an unclean recording refused whole naming the file and check code, a convention the task lists
occupied-resources#34.1 · no · the harness session dir and the fake agent's agents-mode file are e2e seam plumbing, and the unit-level proof boots no harness
occupied-resources#35.1 · no · standing S1 · each new test's own viola home under target/e2e-home/viola-test-*/home, with its owner record and sweep, a convention the task lists
occupied-resources#36.1 · no · the Windows icacls inheritance into a seeded home's conpty seed is a test-seeding rule, and the chunk adds no seeded home or grant
occupied-resources#37.1 · no · the profraw census is a dev-host witness for the corrupt-profile mechanism that no CI job or chunk line runs
occupied-resources#38.1 · no · the host mutation scratch serves run --mutants, which the chunk does not run
occupied-resources#39.1 · no · the Windows protected-DACL oracle for the home strict-modes check is owned by "Unix endpoint and home hardening" (Not needed)
occupied-resources#39.2 · no · the Windows inherited-ACL oracle for the plain-home strict-modes case is owned by "Unix endpoint and home hardening" (Not needed)
