# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · viola-state is a sync crate, so the fix in pin_bytes / pin_exe and run's pin path stays on std threads and blocking I/O with Tokio excluded
established-decisions#2.1 · yes · the ConPTY companions `run` pins under bin/<version>-<hash>/conpty/ (pin_companions, edited), with their fail-open to the inbox ConPTY, are the Windows counterpart the race may reach through replace_private_shared
established-decisions#3.1 · no · the vt100 readiness gate and its feed thread are untouched, the chunk edits run's start pin and companions, not the screen model
established-decisions#4.1 · no · delivery confirmation and the send path are untouched, the chunk sends no frame and changes no send path
established-decisions#5.1 · yes · any guard the fix adds must be a std File::lock on a separate `<name>.lock` sibling, never an exclusive lock on the guarded file
established-decisions#6.1 · yes · replace_private and replace_private_shared (both edited) are the one snapshot-writer helper, and the fix must keep its persist, REPLACE_ATTEMPTS retry and already-holds-those-bytes rule for the pinned copy, plugin files and companions
established-decisions#7.1 · no · the wrapper's endpoint, its interprocess local_socket transport and the Windows SQOS client open are untouched, the chunk changes no channel method
established-decisions#8.1 · no · the frame clients' server_pid check guards frames the chunk never writes
established-decisions#9.1 · no · the macOS no-peer-pid gap in server_verdict concerns frame clients the chunk does not touch
established-decisions#10.1 · no · the replay readers (read_snapshot_or_replay) are untouched and the chunk reads no snapshot
established-decisions#11.1 · no · the statusline serve and the run.collision_check span order are units the chunk does not edit
established-decisions#12.1 · no · the MCP tool set, the stdio server and its result shapes are untouched, since the chunk changes no tool
established-decisions#13.1 · no · the frame tools' check_instance gate ahead of live_endpoint is untouched and the chunk writes no frame
established-decisions#14.1 · yes · the exec-form hook command (the pinned path with args hook and the event) and the tier map, which the per-start plugin rewrite (edited) writes into hooks.json
established-decisions#15.1 · no · the event and dialog connect (read_snapshot_or_replay, connect_by, verify_server) runs inside a hook process that already started, and the chunk does not change it
established-decisions#16.1 · yes · the fail-open hook contract (exit 0, never 2, no stderr) that the race's observed symptom rests on, and the rendered plugin's byte-identity across starts that the per-start rewrite must keep
established-decisions#17.1 · no · the dialog-expired append at the hold's end is untouched, the chunk touches no dialog path
established-decisions#17.2 · no · the measured 671 s dialog-hold return records the dialog path, which the chunk does not change
