
## 2026-10-19-newer-claude-cli-rows — the readiness stall report in wait_ready
**Section:** §5 Integration Test Strategy (Readiness stall report).
**Change:** New. When `tests/support/home.rs` `wait_ready` reaches its deadline, it appends the stalled wrapper's state to the watch report before the panic, so the report reaches the JUnit. The state is the wrapper's last `run-<name>.ndjson` event; on macOS `/usr/bin/sample <pid> 1`'s call-graph stack lines (bounded at 4 s, at most 120); on Linux `/proc/<pid>/wchan` and the `stat` state (`note_stall`, `wrapper_pid`, `process_state` per OS). It signals nothing and adds nothing to a passing boot. It is the witness for the macOS readiness stall inside `run.start`, whose phase is not yet named.
**Why:** The stall recurred on 1 of 4 macOS attempts read and has not been located. The founder's own word, relayed verbatim by the operator, closes the chunk with the criterion unmet and a route WATCH as its owner, the report as its witness. That is his departure from §10's in-chunk quarantine for this flake; §10 is not amended.
**Ref:** .andromeda/runs/2026-10-20T05-48-20-wrap/
