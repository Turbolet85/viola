# Cascade compare — architecture · wrap · 2026-11-01-unix-endpoint-and-home-hardening
E items 52 · held 33 · lost at classify 4 · lost at strong 0 · beside a placed fact 0 · cascade only 8 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### conventions#18.1 · lost at classify · .andromeda/architecture-detail/conventions.md:133
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 133
    section: '## Conventions'
    change: 'A new item under `conv-mcp-results`, in its own detail block: - A frame tool (`send`, `wait`, `last`, `answer`), after `live_endpoint`, runs `check_endpoint_dir` on the live endpoint: a refusal returns the `instance-unreachable` `isError` result with `detail: "socket-dir-refused"`, text `refused: instance-unreachable`, and writes no frame and no line, the `isError` result the record.'
    sidecar: 'Conventions: MCP frame tools refuse a loosened socket directory with `socket-dir-refused`.'
    rationale: 'MCP `frame` gains the check after `live_endpoint`, with no frame, no line and `isError` the record (report.md:17, report.md:73).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17

### stack-and-technologies#1.6 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 15
    section: "## Stack and Technologies"
    change: "In the Wrapper IPC row, extend the clause on the per-user socket directory to read: on Unix, libc gives the uid for the per-user socket directory and the socket is chmod 0600 after the bind; the directory itself is read by `viola_state::strict::check_endpoint_dir` (the recorded endpoint's parent, through `check_socket_dir`, one std `symlink_metadata`) before a bind (`viola run`, `revive`'s start) and before each client's connect (the CLI frame verbs, `hook`, MCP's `frame`), a symlink, a non-directory, another owner or a group or other bit refused and never chmodded, an absent path passing."
    sidecar: "Stack and Technologies, Wrapper IPC: the per-user socket directory is now verified before a bind and before each client's connect, by std `symlink_metadata`, with no new dependency (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The row names how the Unix socket directory is provided but not the check this chunk lands on it; report Changes, Symbols / APIs: `check_socket_dir`, `check_endpoint_dir` and `socket_dir_verdict`, with the bind and the three client seams, and Dependencies: none added."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11

### standard-contracts#18.1 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:139
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 139
    section: "## Standard Contracts"
    change: "Replace «so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the server check» with «so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the socket-directory check on the recorded endpoint, then on its connected stream the peer euid decision, then the server check»; the macOS sentence and the rest of the line stand."
    sidecar: "The frame-writing client's order of checks gains the socket-directory check and the peer euid decision."
    rationale: "Report Symbols / APIs: `src/cmd/client.rs` `connect` and `crates/viola-mcp/src/tools.rs` `frame` run `check_endpoint_dir` after the endpoint read and `peer_admitted(client.server_euid())` ahead of the server check; pattern `mac-gap` keeps the statement that macOS compares no peer pid."

### standard-contracts#18.2 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:140
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 140
    section: "## Standard Contracts"
    change: "Add the item: - The peer euid decision is `viola_channel`'s. `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer it cannot read is refused where reported; `peer_admitted(observed)` is that decision over this process's `own_euid()` and `PEER_EUID_REPORTED` (`cfg!(unix)`). `Client::server_euid` and `TokioClient::server_euid` read the connected peer through `peer_euid_of` (`peer_creds().ok()?.euid()` on Unix, none on Windows), writing and logging nothing."
    sidecar: "New: `peer_euid_admitted`, `peer_admitted`, `PEER_EUID_REPORTED` and the two `server_euid` readers."
    rationale: "Report Symbols / APIs, the `viola_channel` bullet; Deviations item 1 for the name `peer_euid_of`."
    basis: crates/viola-channel/src/lib.rs:20

## Held

- conventions#11.1
- conventions#12.1
- conventions#13.1
- conventions#15.1
- conventions#16.1
- conventions#17.1
- conventions#4.1
- conventions#5.1
- conventions#7.1
- cross-cutting-patterns#4.1
- established-decisions#11.1
- established-decisions#16.2
- established-decisions#20.1
- established-decisions#22.3
- established-decisions#25.2
- established-decisions#29.1
- established-decisions#29.3
- established-decisions#35.1
- occupied-resources#1.1
- occupied-resources#18.1
- occupied-resources#2.1
- occupied-resources#23.1
- occupied-resources#24.1
- occupied-resources#28.1
- occupied-resources#33.1
- occupied-resources#35.1
- occupied-resources#4.1
- occupied-resources#5.1
- stack-and-technologies#2.1
- standard-contracts#12.1
- standard-contracts#15.2
- standard-contracts#16.5
- standard-contracts#7.1

## Cascade only

- established-decisions#10.1
- established-decisions#29.7
- established-decisions#9.1
- occupied-resources#26.1
- occupied-resources#6.1
- standard-contracts#15.1
- standard-contracts#16.1
- standard-contracts#6.1
