proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 15
    section: "## Stack and Technologies"
    change: "Wrapper IPC row, Role cell, append one sentence: Before its first frame every client (the six CLI frame verbs, `hook`'s event and dialog paths, `viola mcp`'s frame tools) reads the connected stream's peer pid through interprocess 2.4.4 `StreamCommon::peer_creds().pid()` (`viola_channel::Client::server_pid` / `TokioClient::server_pid`; Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none, `SERVER_PID_REPORTED` false: the recorded gap) and compares it with the snapshot's `pid` + `started_at` (`viola_state::liveness::verify_server`); a mismatch writes no frame (as landed, chunk 2026-10-26-server-verification-before-any-frame)."
    sidecar: "stack-and-technologies Wrapper IPC row: the client's pre-frame server check through interprocess 2.4.4 `peer_creds().pid()` recorded (no new dependency)."
    rationale: "The report's Symbols / APIs bullet names `Client::server_pid` / `server_pid_of` reading `peer_creds().pid()` (interprocess 2.4.4 `StreamCommon`; Linux SO_PEERCRED, Windows GetNamedPipeServerProcessId, macOS none) and `verify_server` used by every frame-writing client; Dependencies: none added (interprocess pin unchanged), so this is new detail on an existing stack row, not a new library."
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 26
    section: "## Stack and Technologies"
    change: "Process liveness row, Role cell, append: sysinfo's process start time also backs the server check before every client frame — the endpoint peer pid's start time, read through `process_start_time` (a zombie none; in `viola mcp` on `tokio::task::spawn_blocking`), against the snapshot's `started_at` (closed `ServerVerdict` `Matches` · `Mismatch` · `Unverifiable`; as landed, chunk 2026-10-26-server-verification-before-any-frame)."
    sidecar: "stack-and-technologies Process liveness row: sysinfo's start-time read recorded as the server-verification input, not liveness alone."
    rationale: "Report Symbols / APIs: `verify_server` reads the observed start time through `process_start_time` and the pure `server_verdict` returns the closed `ServerVerdict`; `crates/viola-mcp/src/tools.rs` `server_verified` does that read on `tokio::task::spawn_blocking` (348-357). The row today names sysinfo's role only as enriching the heartbeat signal; Dependencies: sysinfo pin unchanged."
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:12
note: no D-arch-decisions or D-arch-resources drift in this file (no dependency, runtime, crate or resource added; tokio stays in viola-mcp); the host's `rustup target add aarch64-apple-darwin` (report Dev-tool versions) is a host lint recipe, not a toolchain pin, so it is no stack-row edit here.
