# Classified — test-plan · test-scope-summary.b1

## Answers
test-scope-summary#1.1 · no · the channel's Tokio client feature and its framing, hash and refusal shapes are unchanged, since only the tokio patch pin moved
test-scope-summary#2.1 · no · agent-claude's hook, R8, shim, ledger, probe and parser surfaces are unchanged, since the session changes no source
test-scope-summary#3.1 · no · no CLI verb, exit code or --json shape moved, since the session changes no source, symbol or env var
test-scope-summary#4.1 · no · the cli drivers (assert_cmd, Command::cargo_bin) and their exit, JSON and human signals are unchanged, since assert_cmd did not move
test-scope-summary#5.1 · no · the tui outer-PTY driver and its event and exit signals are unchanged, since no product behaviour changed
test-scope-summary#5.2 · no · the wrapper channel's sync client and raw interprocess driver are unchanged, since interprocess did not move and no IPC method changed
test-scope-summary#5.3 · no · the MCP tools, result shapes and refusals are unchanged, since no MCP source or tool changed and every other crate version is unmoved
test-scope-summary#6.1 · no · the five-command strategy and its invoked suites are unchanged, since no test surface changed and no dev-tool version moved
test-scope-summary#7.1 · no · the fake-agent data strategy and fixture rules are unchanged, since no fixture, scrub, fake-agent or test surface changed
test-scope-summary#8.1 · no · the run start sequence, its events and snapshot fields are unchanged, since no product behaviour changed
test-scope-summary#8.2 · no · the confirmed-send path, its cursor and receipt signals are unchanged, since no product behaviour changed, the MCP send included
test-scope-summary#9.1 · no · the human-wheel path, its refusals and release rules are unchanged, since no product behaviour changed
test-scope-summary#9.2 · no · the budget-governor path, its files and refusals are unchanged, since no product behaviour changed
test-scope-summary#10.1 · no · the chaos triggers are unchanged, since no chaos, property or contract surface changed
test-scope-summary#11.1 · no · the contract-test trigger and its fake-agent suite are unchanged, since no fixture or fake-agent behaviour changed
