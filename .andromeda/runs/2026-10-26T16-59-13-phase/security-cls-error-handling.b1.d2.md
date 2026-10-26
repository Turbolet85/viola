# Classified — security · error-handling.b1

## Answers
error-handling#1.1 · yes · the chunk's strict-modes read in `hook` (`src/cmd/hook.rs`, through `VIOLA_DIR`) must fail open at exit 0 with no body and empty stderr
error-handling#1.2 · yes · a strict-modes failure before `hook.event`'s and `hook.dialog`'s server check must take the no-frame, exit-0 path, with a SessionEnd appended through `append_session_end`
error-handling#1.3 · yes · the strict-modes check in the frame tools' `live_endpoint` read (`crates/viola-mcp/src/tools.rs`) must return `instance-unreachable` with `detail: null` and write no line
error-handling#1.4 · no · the chunk's scope names no link or unlink verb; its `live_endpoint` and `client::start` edits move the liveness read and role-file open, not a wrapper reply's class or exit
