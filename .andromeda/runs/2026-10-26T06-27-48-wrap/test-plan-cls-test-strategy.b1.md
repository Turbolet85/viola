# Classified — test-plan · test-strategy.b1

## Answers
test-strategy#1.1 · no · the unit tier, its six crates plus the root bin, the nextest runner and rstest 0.27.0 are unmoved, and the 4-line lock delta is the tokio entry alone
test-strategy#1.2 · no · the integration tier and its crate-level suites are unchanged, and interprocess 2.4.4 did not move
test-strategy#1.3 · no · the E2E tier, its outer PTY and its rmcp client are unchanged, and no §6 scenario or product behaviour moved
test-strategy#1.4 · no · proptest 1.11.0 and cargo-fuzz 0.13.2 are unmoved, and no parser surface or fuzz target changed
test-strategy#1.5 · no · hyperfine 1.20.0 is unmoved, and no hook or perf surface changed
test-strategy#1.6 · no · sysinfo 0.39.6 and mockall 0.15.0 are unmoved in the lock delta, and no fault surface changed
test-strategy#1.7 · no · cargo-mutants 27.1.0 is unmoved, and no mutation run or verdict rule was involved
test-strategy#1.8 · no · insta 1.48.0 and jsonschema 0.57.0 are unmoved, and no fixture, scrub or fake-agent behaviour changed
test-strategy#2.1 · no · the directory pattern is unchanged, and the root package and sync crates still carry no tokio dev-dependency since only the pin moved
test-strategy#3.1 · no · the IPC vector, its layers and the security negatives are unchanged, and no channel or IPC surface moved
test-strategy#3.2 · no · the paste-injection layers and Scenario 2 are unchanged, and validate_paste_text did not move
test-strategy#3.3 · no · the hook-stdin boundary and the security sweep are unchanged, and no hook surface moved
test-strategy#3.4 · no · the MCP layers, Scenarios 2, 5 and 7 and the rmcp client are unchanged, and rmcp is unmoved
test-strategy#3.5 · no · the CLI and env layers are unchanged, and no CLI or env surface moved
test-strategy#3.6 · no · the filesystem layers and Scenarios 1 and E5 are unchanged, and no disk surface moved
test-strategy#3.7 · no · the child-spawn layers and Scenario E2 are unchanged, and no process surface moved
test-strategy#3.8 · no · the supply-chain stages it names (cargo deny with the fuzz lockfile audit, sole-root tokio ban, ban probes, zizmor) are unchanged, and the row states no job reading
test-strategy#3.9 · no · the property tier and the fuzz job are unchanged, and no proptest or fuzz target moved
test-strategy#3.10 · no · the chaos suite is unchanged, and no failure-injection surface moved
test-strategy#3.11 · no · the performance-budget rows and hyperfine gates are unchanged, and no hook or perf surface moved
test-strategy#3.12 · no · the cross-surface scenarios and parity suite are unchanged, and no §6 scenario or product behaviour moved
test-strategy#3.13 · no · the OS matrix and CI matrix builds are unchanged, and a lock-only bump does not touch them
test-strategy#3.14 · no · the viola-agent-claude unit and contract layers and the fixture set are unchanged, and no fixture moved
test-strategy#3.15 · no · the contract suite and the fixture set are unchanged, and no fake-agent behaviour moved
test-strategy#3.16 · no · the error-sanitization and secret-scan suites are unchanged, and no log or error surface moved
