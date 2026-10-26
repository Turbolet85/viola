# Classified — architecture · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the list payload and its item and budget shapes are unchanged, and list writes no frame and does no server check
standard-contracts#2.0 · yes · stale · the measured product callers of read_snapshot are three (src/cmd/hook.rs:204, src/cmd/hook/statusline.rs:142, src/cmd/run.rs:379), not five, and src/cmd/client.rs holds none
standard-contracts#3.0 · no · the ledger stamps envelope, its writer and the relied set are unchanged, and no ledger row was added
standard-contracts#4.1 · no · links, their restore and the log rule are unchanged, and the report leaves the link set untouched
standard-contracts#5.1 · yes · add-to · F1: CLI send and MCP send compare the connected stream's peer pid and its start time with the snapshot's pid and started_at before the send frame, and a mismatch or failed check writes no frame
standard-contracts#6.1 · yes · add-to · F1: CLI wait and MCP wait compare the connected stream's peer pid and its start time with the snapshot's pid and started_at before the wait frame, and a mismatch or failed check writes no frame
standard-contracts#7.1 · yes · add-to · F1: the hook's event path compares the connected stream's peer pid and its start time with the snapshot's pid and started_at before its hook.event frame, and a mismatch or failed check writes no frame and fails open with no body
standard-contracts#8.1 · no · list is not a channel method, writes no frame and does no server check, and the board and its reads are unchanged
standard-contracts#9.1 · no · event data per kind is unchanged with no kind or field added, and a mismatch appends no event line
standard-contracts#10.1 · no · the hook statusline arm is not changed (the report's untouched list), and it writes no frame so the server check does not reach it
standard-contracts#11.1 · no · the claude agents read, its callers and the agents-join are untouched, and the report adds no ledger row
