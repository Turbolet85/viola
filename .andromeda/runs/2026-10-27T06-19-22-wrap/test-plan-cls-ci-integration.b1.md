# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the lint commands, the sync-crates check list and the rmcp-features last line are unchanged, and CI ci#40458923095 is green on the changed Rust files
ci-integration#1.2 · yes · add-to · F6 joins its probe list, the new serde_json unbounded_depth ban in deny.toml is one this stage lists, and the deny-probes verdict moves from 13/13 to 14/14
ci-integration#1.3 · no · the stage still runs agent-run run --unit under nextest and names no test count, so the new unit cases join that run without moving its text
ci-integration#1.4 · no · the stage still runs agent-run run --integration and names no count, so the moved integration count and the chaos test change move no line of it
ci-integration#1.5 · no · the stage still runs agent-run run --e2e on all three OSes, and the new contract binary and mcp_tools case run under that same command, so no line of it moves
ci-integration#1.6 · no · the coverage run, its ConPTY vendor steps and its gate line are unchanged, and the coverage floors of 85, 95 and 80 per OS still hold on the changed tree
ci-integration#1.7 · no · the windows-mutants matrix keeps its ten items and its root-package file split, the session adds no matrix item, and the mutants leg stays report-only
ci-integration#1.8 · no · the perf job's steps and budgets are unchanged, and its schema-check still passes on the widened diag-line enum in green CI ci#40458923095
ci-integration#1.9 · no · the 1.96 floor and its cargo check and nextest-unit steps are unchanged, and CI ci#40458923095 is green on the changed tree
ci-integration#1.10 · no · the fuzz-replay and nightly fuzz steps are unchanged, no fuzz target is among the reached files, and CI ci#40458923095 is green
ci-integration#1.11 · no · release-check and its 5/5 probe are unchanged, and CI ci#40458923095 is green on the changed tree
ci-integration#1.12 · no · the gate still reads the same reports and exits on §10 breaches, the §10 thresholds are unchanged, and the entity names no count the session moved
