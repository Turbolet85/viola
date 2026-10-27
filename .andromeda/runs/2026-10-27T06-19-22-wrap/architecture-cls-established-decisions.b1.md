# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · the new AsyncRead reader sits inside viola-mcp, the one Tokio crate, so the re-read holds: no Tokio ban or dependency changed
established-decisions#2.1 · yes · add-to · F6, an OPEN Windows finding that the child's wide character reaches the outer terminal after a contained feed panic under a 1-column ConPTY, owned by Windows-only live measurements, joins this record's open Windows ConPTY findings
established-decisions#3.1 · yes · contradict · the measurement shows a caught vt100 panic logs panicked and only a dropped copy logs oversize (src/run/gate.rs:49 and :72)
established-decisions#4.1 · no · the send, confirmation and window code lies outside every reached file, and the MCP stdin cap bounds input, not send's wait
established-decisions#5.1 · no · the new mcp-stdin line goes to the diagnostics log, not events.ndjson; append, lock and the events reader are unchanged, and replace_private atomicity is re-read unchanged
established-decisions#6.1 · no · the fallback compare now reads bytes+1 through holds_exactly with the outcome of a whole-file compare, so "counts as done when the target already holds exactly those bytes" still holds on the changed fs.rs and its callers are unchanged
established-decisions#7.1 · no · fs.rs changes only the replace_private_shared compare, the new private holds_exactly and its test; no merge, conflict or dependency moved place_private_new, so its placement claims hold on the changed file
established-decisions#7.2 · no · the placed-hook body place_private_new_with sits outside the fs.rs change, and no reached test is one of the placed-copy tests
established-decisions#7.3 · no · the write handle closed before persist_noclobber is outside the fs.rs change, and no merge or dependency moved it
established-decisions#8.1 · no · the channel endpoints, transport and method set are untouched; the new cap is on viola mcp's own stdio, not the local-socket channel
established-decisions#9.1 · no · server_pid and the peer-pid check in viola-channel are untouched
established-decisions#10.1 · no · the macOS no-peer-pid gap is unchanged and no socket code changed
established-decisions#11.1 · no · the snapshot replay readers, read_snapshot_or_replay among them, are unchanged
established-decisions#12.1 · no · the statusline serve reads the snapshot through the replay reader, which is unchanged
established-decisions#13.1 · yes · add-to · the body must carry F1 (viola mcp's stdin is capped at MAX_FRAME per line and a longer line ends the session), F2 (one parse-rejected mcp-stdin WARN line, then exit 0 once initialized or exit 11 when cut before initialize) and F3 (rmcp's stdio read is unbounded, so the bound is viola's reader)
established-decisions#14.1 · no · the frame tools' check_instance-then-live_endpoint order and instance-unreachable shape are untouched; lib.rs changes only serve's stdin wiring and the cut line
established-decisions#15.1 · no · the hook transport and the hook contract have no changed code path
established-decisions#16.1 · no · the event and dialog connect path (replay, connect, verify_server) is untouched
