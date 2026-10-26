# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the Untouched line holds (`src/cmd/list.rs` and the board read model unchanged), and the reached `src/run/budget.rs` edit is test-only
standard-contracts#2.0 · no · the Untouched line holds (`viola-state` unchanged), and the five product `read_snapshot(` callers are product code the report leaves unchanged, the reached `src/cmd/run.rs` and `src/cmd/hook/statusline.rs` edits being test-only
standard-contracts#3.0 · no · the Untouched line holds (`viola-state` unchanged, no ledger row, stamp or relied-set count moved), and the reached `src/cmd/run.rs` edit is test-only
standard-contracts#4.1 · no · the link restore and the first snapshot's `links` are product behaviour the report leaves unchanged, and the reached `src/cmd/run.rs` edit is test-only
standard-contracts#5.1 · no · `send`'s product behaviour is unchanged, and the reached `src/run/send/test_support.rs` and `src/run/budget.rs` edits are test-only
standard-contracts#6.1 · no · the `wait` wake set and its line-start rule are unchanged, `src/run/wait.rs` is not reached, and the reached `src/run/dialog.rs` edit is test-only
standard-contracts#7.1 · no · `hook.event`'s validation and append are product behaviour the report leaves unchanged, and no reached edit is product code
standard-contracts#8.1 · no · the Untouched line holds (`src/cmd/list.rs` and the board read model unchanged), so CLI and MCP `list` keep their order and provider
standard-contracts#9.1 · no · `viola-agent-claude` was not edited (Untouched), and the reached `src/run/send/test_support.rs` edit is test-only
standard-contracts#10.1 · no · the statusline arm's order, shell-out and Windows `None` are product behaviour the report leaves unchanged, and the reached `src/cmd/hook/statusline.rs` and `src/cmd/run.rs` edits are test-only
standard-contracts#11.1 · no · the Untouched line for session liveness holds (`viola-state` unchanged), `src/run/agents.rs` is not reached, and no reached edit is product code
