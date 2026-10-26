proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 34
    section: "## Conventions"
    change: "the exit `21` row gains one clause: also a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) whose endpoint is served by a process that is not the snapshot's recorded wrapper (the connected stream's peer pid and its OS start time against the snapshot's `pid` / `started_at`, `viola_state::liveness::verify_server` through `cmd::client::connect`), or whose check cannot be made, on Linux and Windows (macOS the recorded gap: no peer pid, no comparison): no frame written, the client's own exit-13 refusals still first, the same `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}` document under `--json` (stderr empty), in human mode the `unable  <name>  instance-unreachable` line with the hint `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` last on stderr, and one `process-exit{subject:\"self\", exit_code:21, detail:\"server-verify-failed\", during:\"connect\"}` at WARN in `cli-<name>.ndjson` holding no pid, start time, path or endpoint (chunk 2026-10-26-server-verification-before-any-frame)"
    sidecar: "conv-exit-codes: exit 21 also covers a CLI frame verb's server-identity mismatch or unmakeable check (same instance-unreachable document, the server-verify hint, process-exit server-verify-failed during connect) — chunk 2026-10-26-server-verification-before-any-frame"
    rationale: "report Changes › Symbols / APIs (src/cmd/client.rs `Unreachable::ServerUnverified`, `connect`; src/human/hints.rs `server-verify`; the six verbs: «a mismatch or an unmakeable check is exit 21 with the unchanged … document, or the `unable  <name>  instance-unreachable` line with the new hint last on stderr») and › Schema / config (the first build that writes `process-exit{… exit_code:21, detail:\"server-verify-failed\", during:\"connect\"}`); the row today defines 21 only as «no live endpoint for the `ViolaName`», so a live endpoint served by an impostor is an exit-21 cause the file does not state"
    basis: src/cmd/client.rs:75
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 40
    section: "## Conventions"
    change: "after the `invalid-params` sentence, add: as landed (chunk 2026-10-26-server-verification-before-any-frame), the frame tools `send` · `wait` · `last` · `answer` verify the serving process after `TokioClient::connect` (`server_verified`: the peer pid and the recorded `pid` / `started_at`, the start-time read on `tokio::task::spawn_blocking`), and a server that is not the recorded wrapper, or a check that cannot be made, is the same `isError: true` result `{\"error\":\"instance-unreachable\",\"detail\":null}`, text `refused: instance-unreachable`, with no frame written and no log line, on Linux and Windows (macOS the recorded gap: no peer pid)"
    sidecar: "conv-mcp-results: the MCP frame tools' server-identity mismatch reads as the instance-unreachable result, no frame, no line — chunk 2026-10-26-server-verification-before-any-frame"
    rationale: "report Changes › Symbols / APIs (crates/viola-mcp/src/tools.rs: «`frame` verifies after `TokioClient::connect` through `async fn server_verified` … a mismatch returns `result::unreachable()` with no frame, no new log line») and Outcome (MCP impostor: `isError`, `{\"error\":\"instance-unreachable\",\"detail\":null}`, text `refused: instance-unreachable`, no frame); the MCP row names an unreachable instance's result but not this new cause of it"
    basis: crates/viola-mcp/src/tools.rs:348
note: D-arch-resources and D-arch-decisions hold for this category (report: no IPC method, endpoint, port, socket, env var, crate or dependency added); no passage of conventions.md states a dated server-identity gap as standing, so no D-retired.
