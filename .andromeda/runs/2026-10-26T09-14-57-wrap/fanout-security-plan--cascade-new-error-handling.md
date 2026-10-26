proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 8
    section: "## Error Handling"
    change: "- A `viola mcp` frame tool (`send` · `wait` · `last` · `answer`) whose server check fails (the connected peer is not the recorded wrapper, or the check cannot be made) sends no frame and returns `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`: the same result as its unreachable path, no new error word and no log line written. The start-time read runs on `spawn_blocking` (chunk 2026-10-26-server-verification-before-any-frame)."
    sidecar: "error-handling detail: new item, the MCP frame tools' failed server check returns isError instance-unreachable (detail null), no frame, no line (F5)."
    rationale: "Report Changes, Symbols: `crates/viola-mcp/src/tools.rs` `frame` verifies after `TokioClient::connect` through `server_verified` (348-357), the start-time read on `tokio::task::spawn_blocking` (351-353), a mismatch returning `result::unreachable()` with no frame and no new log line; Schema / config: MCP writes no line for the mismatch, no MCP error word added; acceptance MET: MCP send / wait / last / answer against the impostor give isError `{\"error\":\"instance-unreachable\",\"detail\":null}`, text `refused: instance-unreachable`, no frame."
    basis: crates/viola-mcp/src/tools.rs:348
    fact: F5
note: line 8's own `hook` row (server verification still "join at Epoch 6") is stale per the task's Touches; that standing-entity edit is the strong readers', not proposed here.
