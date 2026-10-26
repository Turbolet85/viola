# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the list items and budget come from viola-state's sync board read and join and the root bin's list, which the bump does not reach
standard-contracts#2.0 · no · the snapshot envelope is read by viola-state's sync snapshot code, which the bump does not reach
standard-contracts#3.0 · no · the stamps envelope is written and read by sync stamps code and the ledger module, which the bump does not reach
standard-contracts#4.1 · no · the link set is the wrapper's sync LinkSet in src/run/links.rs, which the bump does not reach
standard-contracts#5.1 · no · the send rule is the wrapper's budget gate and link recording, which the mcp Tokio client only transports
standard-contracts#6.1 · no · the wait wake is the wrapper's plain Mutex and Condvar in src/run/wait.rs, not a Tokio primitive
standard-contracts#7.1 · no · hook.event is sent by the sync hook verb, which carries no Tokio edge
standard-contracts#8.1 · no · the list payload is built by the sync list::board_document provider, which the bump leaves unchanged and the mcp tool only transports
standard-contracts#9.1 · no · prompt-submitted event data is produced by the sync hook parser in viola-agent-claude, which the bump does not reach
standard-contracts#10.1 · no · the statusline hook arm is a sync path that shells out through /bin/sh with no Tokio edge
standard-contracts#11.1 · no · the agents read is a direct spawn in the root bin, and the mcp list tool's blocking-thread placement is viola-mcp's own call, which the bump leaves as it is
