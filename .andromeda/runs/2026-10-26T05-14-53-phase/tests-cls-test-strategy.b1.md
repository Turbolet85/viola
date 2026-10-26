# Classified — tests · test-strategy.b1

## Answers
test-strategy#1.1 · yes · the unit tier of all six crates and the root bin re-runs per OS under nextest, the workspace-tests gate the chunk owes on the bumped lock
test-strategy#1.2 · yes · the integration tier re-runs, and its MCP crate-level suite on the Tokio client (viola-mcp handlers.rs) is a named reached path
test-strategy#1.3 · yes · the E2E tier re-runs through the rmcp client over a viola mcp child, the E2E MCP stdio path the Needs name and the bump reaches
test-strategy#1.4 · no · property suites are Not needed (no case added or changed) and proptest is unmoved, the fuzz and corpus parts sit in a workspace holding no tokio
test-strategy#1.5 · no · performance budgets and the perf job's hook rows are Not needed, the hook path is sync and no hook code changes
test-strategy#1.6 · no · chaos suites are Not needed (no case added or changed), their faults are sync file, process and mockall seams the bump does not reach
test-strategy#1.7 · no · the mutation gate is judged at the epoch boundary, never by a chunk (Not needed)
test-strategy#1.8 · no · contract suites, the fake agent, the recorded fixtures and the Mcp. exchanges are Not needed (no fixture recorded, added or scrubbed), and the fake agent is sync
test-strategy#2.1 · yes · the Tokio-client suites' home in crates/viola-e2e and the no-tokio dev-dependency split of the root and sync crates are the §2 directory convention the scope names
test-strategy#3.1 · no · the IPC rows name the sync listener and module suite, §6 Scenario 1 and the Windows listener negatives (Not needed), none a Tokio path, and the Tokio IPC path is bound under V5
test-strategy#3.2 · yes · its E2E leg, §6 Scenario 2, is an rmcp-client MCP scenario the bump reaches (the MCP vector's E2E layer, 3.4), while its viola-core unit and proptest legs are untouched
test-strategy#3.3 · no · hook stdin is tested by sync assert_cmd hook integration on a path that starts no Tokio runtime, and the Security sweep's MCP parts bind under V5
test-strategy#3.4 · yes · the MCP vector's rmcp-client E2E scenarios (§6 Scenarios 2, 5, 7) run over viola-mcp's Tokio stack and viola-e2e's Tokio clients, the surface the bump moves
test-strategy#3.5 · no · CLI verbs, env handling and home resolution are sync and outside every Tokio path, the bump reaches none of them
test-strategy#3.6 · no · filesystem integration (§5 Module ↔ disk) and Scenario 1 are sync, Scenario 1 not being among the MCP scenarios, and the chaos leg is Not needed
test-strategy#3.7 · no · child-spawn integration and Scenario E2 spawn PTY and fake-agent children directly, and Scenario E2 is not among the MCP scenarios (2, 5, 7)
test-strategy#3.8 · yes · the supply-chain gates (cargo deny over the root lock, the sole-root tokio ban, the channel tree form, the deny probes, zizmor) are the gates the chunk re-runs and reads green
test-strategy#3.9 · no · property suites and the §9 fuzz job are Not needed (no case added or changed, the fuzz workspace holds no tokio)
test-strategy#3.10 · no · chaos suites are Not needed (no case added or changed) and their faults are sync
test-strategy#3.11 · no · performance budgets are Not needed, the hook path is sync and no hook code changes
test-strategy#3.12 · yes · cross-surface parity (the cross suites in viola-e2e, Tokio-client E2E) and §6 Scenario 2, an MCP scenario, drive the rmcp surface the bump reaches
test-strategy#3.13 · yes · the Tokio paths re-run on windows-2025, macos-latest and ubuntu-latest, a scope obligation, and a Linux-only local run proves neither of the other two
test-strategy#3.14 · no · viola-agent-claude, its per-CLI-version fixture replay and the Claude shapes are untouched and no Tokio path runs through them (fixtures and stamped versions are Not needed)
test-strategy#3.15 · no · contract suites are Not needed (no case added or changed), and the fake agent is sync with no fixture moving
test-strategy#3.16 · no · its plan location (§6 error sanitization and secret scan, §3 logs) names no Tokio-bound path, and the chunk changes no error body or log line
