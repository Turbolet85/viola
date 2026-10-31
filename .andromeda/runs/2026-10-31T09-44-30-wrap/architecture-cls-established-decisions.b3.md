# Classified — architecture · established-decisions.b3

## Answers
established-decisions#27.1 · no · no ledger row, probe, stamp or verify step moved, and the reached version_gate, statusline, dialog and env changes are test cases only
established-decisions#28.1 · no · viola-state is untouched and StampsError::StrictModes keeps its code, so the refusal still reads and writes nothing and the home pair is unmoved
established-decisions#29.1 · no · dec-session-liveness is untouched (no heartbeat, pid or start-time rule moved) and the reached run files change only resolve_program's error type, so the start order stands
established-decisions#29.2 · no · the strict-modes check, the liveness pre-check and the peer pid and start-time check before the first frame did not move, and the reply screens act only after the first frame
established-decisions#29.3 · no · dec-session-liveness is untouched, and the collision check's one snapshot read and heartbeat-only decision did not move
established-decisions#29.4 · no · dec-session-liveness is untouched, and the revive preflight's one snapshot read and its one state-recovered line did not move
established-decisions#29.5 · no · viola-state is untouched, so check_home_tree and the home or instance refusal pair stand, and the reached run files change only resolve_program's error type
established-decisions#30.1 · yes · contradict · the crate no longer holds Refusal, folded into AgentError (F8), and the thiserror census reads 9 enums (F9), so the open-divergence list no longer holds
established-decisions#31.1 · no · no manifest gains an edge, since the reached root Cargo.toml adds only test tables and viola-mcp and the root bin read the helpers over existing edges
established-decisions#32.1 · no · the plugin embedding, the pinned copy and the hooks.json and .mcp.json registrations are unmoved, and a registered viola mcp's exit 1 still does not block, as the session's exit-1 measurement agrees
established-decisions#33.1 · no · viola-state's pin_key, pin_bytes and pin_exe are untouched, the reached run file changes only resolve_program's error type, and no change is named to verify's pin_exe call
established-decisions#33.2 · no · viola-state is untouched and PinError::StrictModes keeps its code, and the Windows companion path in src/conpty.rs is not reached
established-decisions#33.3 · no · the reproducibility measurement concerns the release bin, unmoved by the test-only Cargo.toml tables, and the owed action stays on Version done-check
