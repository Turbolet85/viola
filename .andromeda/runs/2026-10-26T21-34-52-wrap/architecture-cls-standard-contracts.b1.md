# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the item fields, the budget read and the beat-based live or stale reading stand, since the report keeps the budget read and the snapshot field set unchanged and board.rs and src/cmd/list.rs are not reached
standard-contracts#2.0 · yes · stale · the replay's product readers go from two to six and read_snapshot's three product callers go to none, so the counts this entry states have moved
standard-contracts#3.0 · yes · add-to · F10 update_stamps now runs check_stamps before creating ledger/ and again under its lock and returns StampsError, so a refusal reads and writes nothing
standard-contracts#4.1 · no · the link-set restore, its one pass over events.ndjson and its place before server.serve stand, since the report keeps the snapshot field set and its single writer and the src/cmd/run.rs start change is the home-tree check ahead of the log and the pinned exe
standard-contracts#4.2 · no · the set still changes only on an implicit link or a served call through LinkSet's record and remove, and the change reaches only the CLI client in src/cmd/link.rs
standard-contracts#4.3 · no · the already_linked answer and the new pair's since come from the served link method, whose params and ok payloads the report keeps unchanged
standard-contracts#4.4 · no · the unlink answer {} and the no-op on an absent pair stand, since the report keeps the channel ok payloads and only the CLI link client changed
standard-contracts#4.5 · no · the -32602 and -32603 answers and the no-line rule stand, since the report keeps the JSON-RPC error codes and a refused strict-modes check answers no frame at all
standard-contracts#5.1 · yes · add-to · the strict-modes instance check now runs before the peer-pid and start-time comparison in every frame-writing client, so a refused check writes no frame (F6, F7, F8)
standard-contracts#6.1 · no · the send params, ok payload, cursor, budget gate and refusal rungs stand, since src/cmd/send.rs changes only the CLI client's pre-frame check and the wrapper's send path is not reached
standard-contracts#7.1 · no · the wait params, start offset, wake set and WaitFeed rule stand, since src/cmd/wait.rs changes only the CLI client's pre-frame check and the wrapper's wait path is not reached
standard-contracts#8.1 · no · link and unlink still read no from and answer only after their line and snapshot write, since src/cmd/link.rs changes only the CLI client's pre-frame check
standard-contracts#9.1 · yes · add-to · hook.event is now sent only after the strict-modes instance check in src/cmd/hook.rs, which runs ahead of the server check and sends no frame on a refused or absent instance (F8)
standard-contracts#10.1 · no · the list items, the CLI and MCP board reads, the agents join and the budget read stand, since board.rs, src/cmd/list.rs and run::agents are not reached and the viola-mcp tools.rs change is the frame tools' check
standard-contracts#11.1 · no · the prompt-submitted text, origin and the event data per kind stand, since the report keeps the event data unchanged and a refused hook writes no line
standard-contracts#12.1 · yes · add-to · the six frame verbs (send, wait, last, answer, pause, release) now also print the instance-unreachable document with exit 21 on a refused strict-modes check, as link and unlink do (F6)
standard-contracts#13.1 · yes · stale · serve now reads the replayed snapshot, which carries no statusline_command, so nothing runs or prints where this entry says the snapshot is read and the command runs
standard-contracts#14.1 · yes · stale · live_endpoint now reads through the replay as well, so a replayed snapshot gives no endpoint and the entry's one-read_snapshot-call reading has moved
standard-contracts#15.1 · no · the agents read's spawn sites (revive's preflight, verify's Run B, the mcp list tool) and its parse and join stand, since revive now hands one snapshot read to its collision check and directory reading and verify's change is a home check before its log
