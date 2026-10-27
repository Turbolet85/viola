# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · the Tokio containment the Needs state (run, hook, send and the channel server stay std threads, only viola-mcp builds a runtime), and the tokio_client reader the chunk bounds stays behind the channel's tokio feature
established-decisions#2.1 · yes · the version gate never reads the recorded directory and resolves the program on the process's own PATH, which binds the bounded claude --version spawn in src/run/version_gate.rs (reached whole); the ConPTY sideload clauses are Not needed
established-decisions#3.1 · yes · the vt100 gate the chunk re-verifies: catch_unwind around feed and resize, a poisoned model reading input-not-ready, and the pump passthrough never waiting on the model (gate.rs, screen.rs, pump.rs)
established-decisions#4.1 · yes · the paste rule the chunk re-verifies: validate_paste_text then typed_text, with the empty-text refusal before any frame (src/cmd/send.rs) and before the first wheel read (src/run/send.rs); beyond typed_text the window and post-conditions are Not needed
established-decisions#5.1 · yes · the events.ndjson reader the chunk bounds keeps its three skip counts (unknown kinds not returned, unknown fields counted, torn lines counted and never rewritten), and budget reads hold the shared File::lock
established-decisions#6.1 · no · the viola_state::fs write helpers and their persist retry bound are not reached and the chunk adds no writer, so the snapshot, budget, stamps and pin write rules are a re-check every change owes
established-decisions#7.1 · no · Not needed: the no-clobber placement place_private_new of the pinned copy is excluded beyond the pinned key's shape
established-decisions#7.2 · no · Not needed: the placed-hook body of the pinned copy's placement is excluded beyond the pinned key's shape, and the chunk does not touch it
established-decisions#7.3 · no · Not needed: the write handle closed before the placement (ETXTBSY) belongs to the pinned copy's placement, which the task excludes beyond the pinned key's shape
established-decisions#8.1 · yes · the channel seam the chunk bounds: the wrapper frame bound on frame.rs, server.rs and both clients, the wait params after and timeout_ms, answer's dialog_id, and a pending hook.dialog answered null at once
established-decisions#9.1 · yes · the sync and Tokio clients the chunk bounds must keep the server verdict before their first frame and write and log nothing in server_pid, with the chunk's frame bound and paste refusal in those same paths
established-decisions#10.1 · no · the macOS no-peer-pid gap is owned by Unix endpoint and home hardening, which the task excludes, and the chunk adds no OS-specific branch to the server check
established-decisions#11.1 · yes · the snapshot.json read the chunk bounds must replay a higher v or an unparseable snapshot from events.ndjson, and a replayed snapshot gives no endpoint and no frame
established-decisions#12.1 · yes · hook statusline's serve runs nothing and prints nothing from a replayed snapshot (src/cmd/hook/statusline.rs, reached whole), beside the chunk's statusline stdout cap
established-decisions#13.1 · yes · the MCP surface the chunk re-validates: each handler re-validates the raw argument object with invalid-params and no frame, and a handler fault is the internal error (viola-mcp, src/cmd/mcp.rs)
established-decisions#14.1 · yes · each MCP frame tool runs check_instance before live_endpoint and writes no frame or line on refusal, the order the MCP argument re-validation and the MCP send paste check sit within (tools.rs)
established-decisions#15.1 · yes · the hook event names its tier map lists are the closed hook event argument list the chunk closes in src/cmd/hook.rs, and the exit-0 fail-open binds the stdin cap; the tier timings, spine deadline and hooks.json shape are Not needed
established-decisions#16.1 · yes · the event and dialog paths of src/cmd/hook.rs share one connect in a fixed order (snapshot through replay, connect_by, verify_server), on which the chunk's stdin cap and payload parse sit, and a failed verdict exits 0 with no body
