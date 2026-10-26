# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · no · the list payload is unchanged per the task's Not needed, and list.rs and board.rs are not reached
standard-contracts#2.0 · yes · live_endpoint and the hook's VIOLA_DIR snapshot read are edited, so the unreadable-or-unsupported-is-absent rule binds, and the task names the replay fork over read_snapshot_or_replay
standard-contracts#3.0 · yes · update_stamps is edited for the stamps read, so its envelope and merge rules bind, and run's read_stamps_strict gate and the verify-only writer are re-read
standard-contracts#4.1 · yes · the wrapper's Snapshots::update writes links into snapshot.json, the wrapper-only writer the chunk re-reads, and run's start restore pass over events.ndjson sits behind the checks run.rs edits
standard-contracts#4.2 · no · the set changes through LinkSet::link and remove in src/run/links.rs, which the chunk does not reach and whose Links decisions take no change
standard-contracts#4.3 · no · the already_linked and since answers are wrapper channel behaviour in links.rs that the chunk does not change
standard-contracts#4.4 · no · the unlink {} answer and the absent-pair rule are wrapper channel behaviour that the chunk does not change
standard-contracts#4.5 · no · the -32602 and -32603 link answers are channel behaviour that the chunk does not change
standard-contracts#5.1 · yes · the check the chunk adds to send, wait, last, answer, pause, release, the hook events and the mcp frame tools precedes the one snapshot read that this server check uses, and a refusal writes no frame
standard-contracts#6.1 · yes · send's arrival budget gate reads budget.json through read_budget and check_path, which the chunk edits, so a closed gate's refusal must survive the Windows proof, while the send result shape is unchanged
standard-contracts#7.1 · no · the wait params, start offset, wake kinds and WaitFeed are wrapper behaviour the chunk does not change, and wait.rs gains only the pre-frame check that #5.1 covers
standard-contracts#8.1 · no · link and unlink read no from and answer after their line and snapshot write, a channel ordering the chunk does not touch
standard-contracts#9.1 · yes · hook.event goes out only after the server check, and a failed check sends nothing and fails open, the rule the chunk's strict check in the nine hook events must keep
standard-contracts#10.1 · no · list's board, budget and agents-join reads are unchanged per the task's Not needed, and list is not reached; the MCP list tool is another unit of tools.rs
standard-contracts#11.1 · no · prompt-submitted event data is unchanged per the task's Not needed (event data per kind), and the hook edit touches only its snapshot read
standard-contracts#12.1 · yes · the refusal document {"v":1,"error":"instance-unreachable","detail":null} with exit 21, empty stderr and no log line is the closed shape each frame verb's new refusal must print, and client.rs's check runs before any role file
standard-contracts#13.1 · yes · hook statusline's arm calls check_instance, whose Windows proof the chunk adds, and must stay check-first with a silent exit 0 on refusal, and its snapshot read through VIOLA_DIR is one that hook.rs edits
standard-contracts#14.1 · yes · live_endpoint is edited as the snapshot read behind the check, and this entry states its one read and the liveness-then-server order the chunk's check must precede
standard-contracts#15.1 · no · the claude agents --json spawn, parse and join are unchanged per the task's Not needed, agents.rs and resolve_program are not reached, and verify's edit is the stamps call only
