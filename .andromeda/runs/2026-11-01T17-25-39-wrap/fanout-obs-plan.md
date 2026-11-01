proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 Observability Harness Contract
    change: 'In the `run` bullet the start-refusal list gains a socket directory the check refuses (exit 1, before any snapshot, child or `Server::bind`), and the `human::refuse` census reads `run` (8 sites), re-derived at chunk 2026-11-01-unix-endpoint-and-home-hardening; `verify` (6), `list` (2), `revive` (1), `plugin install` (1) and `client` (1) stay.'
    sidecar: '§3: the start refusals gain the socket directory, the `human::refuse` census `run` 7 → 8 (19 under `src/`).'
    rationale: 'Report **Counts / qualifiers moved** names `obs-plan.md:311` as the carrier of the census (18 → 19, `run` 7 → 8), and **Symbols / APIs** adds `refuse_socket_dir` at `src/cmd/run.rs:822-829`; the list of start refusals omits the new one.'
    basis: 'src/cmd/run.rs:822-829'
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 Observability Harness Contract
    change: 'In the same `run` bullet, `viola revive` adds three refusals to `run`''s six, not five: its start shares the socket-directory refusal.'
    sidecar: '§3: `run`''s start refusals, as `viola revive` counts them, five → six.'
    rationale: 'Report **Symbols / APIs**: `revive`''s start shares `refused_with(launch.json, "socket-dir-refused", …)`, so the count of `run`''s refusals that `revive` extends moves with the list.'
    basis: 'src/cmd/run.rs:822-829'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: '§4 Span / Trace Coverage → Scenario: `wait` / `last` event-driven readback'
    change: 'After the instance check, a socket directory the check refuses ends a CLI `wait` / `last` before any frame with one `process-exit{subject:"self", exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN and no `channel-request` line, and MCP''s returns `instance-unreachable` with `detail` `socket-dir-refused` and no line; `server-verify-failed` is then read on Linux and Windows for a serving process that is not the recorded wrapper and on Linux and macOS for a server of another euid, on macOS an euid mismatch alone (macOS compares no peer pid, the recorded gap).'
    sidecar: '§4 `wait` / `last`: the `socket-dir-refused` path added; `server-verify-failed` reaches macOS as an euid mismatch alone.'
    rationale: 'Report **Counts / qualifiers moved** (`verify-word`: Linux, Windows and macOS, on macOS an euid mismatch alone, each row changes; a row stating macOS compares no peer pid stays) and **Coverage of new surfaces** (the eight CLI frame verbs and MCP''s `frame`); the scenario still limits the word to Linux and Windows and holds no line for the new word.'
    basis: 'src/cmd/client.rs:173-175'
  - detector: D-obs-instrumentation
    severity: warning
    section: §7 Error Capture & Reporting
    change: 'In the MCP rejects bullet, `instance-unreachable` logged by no line reads: a frame tool''s refused instance check (made before the liveness pre-check), the liveness pre-check''s, a frame tool''s refused socket directory (`socket-dir-refused`), or a frame tool''s server-verification mismatch on Linux, Windows and macOS (on macOS an euid mismatch alone).'
    sidecar: '§7 MCP rejects: the refused socket directory joins the no-line causes; the server-verification mismatch reaches macOS as an euid mismatch alone.'
    rationale: 'Report **Symbols / APIs**, `crates/viola-mcp/src/tools.rs` `frame`: `check_endpoint_dir` → `UnreachableCause::SocketDir`, `!peer_admitted(…) || !server_verified(…)` → `UnreachableCause::ServerVerify`, no frame, no line, `isError` the record; the bullet closes its list on three causes and on Linux and Windows.'
    basis: 'crates/viola-mcp/src/tools.rs:345-347'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 → OTel SDK init
    change: 'In the `cli` per-role anchor, after a passed instance check the log opens with `process-start`, then follow the liveness pre-check, the socket-directory check on the recorded endpoint (refused: `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, no frame), the peer euid read and the server check.'
    sidecar: '§3 OTel SDK init: the `cli` chain after the log open gains the socket-directory check and the peer euid read.'
    rationale: 'Report **Symbols / APIs**, `src/cmd/client.rs` `connect`: `check_endpoint_dir`, `Client::connect`, `peer_admitted(client.server_euid())`, then `verify_server`, for all 8 callers; the anchor states the chain as the liveness pre-check and the server check alone.'
    basis: 'src/cmd/client.rs:177-179'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage
    change: 'The **IPC invocation** boundary bullet adds: a peer the wrapper''s `accept_loop` refuses by euid is dropped on the accept thread before its worker and before any frame, and writes no line by rule (no frame to key a line on, `channel-response`''s `detail` closed); the client reads end-of-file.'
    sidecar: '§6 IPC invocation: the wrapper''s drop of a refused peer recorded as writing no line, by rule.'
    rationale: 'Report **Coverage of new surfaces** (the wrapper''s drop: instrumentation n/a, no frame to key a line on) and **Decisions & corrections** (the P5 lean: the drop writes no line); every accepted connection passes this operation, and the must-log list leaves its silence unstated, so it reads as an uninstrumented boundary.'
    basis: 'crates/viola-channel/src/server.rs:173-176'
note: D-obs-stack and D-obs-pii are clean (no dependency added or bumped; the new lines carry no path, pid or uid per the report's Coverage bullets). D-retired has no site here, the owner title and the two-rules claim standing only in the detail files, which are the cascade's. The `dependent-of` rows follow the primary before them, the sweep's three primaries sharing one detector id.
