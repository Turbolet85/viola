proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 15
    section: "## Stack and Technologies"
    change: "In the Wrapper IPC row, extend the clause on the per-user socket directory to read: on Unix, libc gives the uid for the per-user socket directory and the socket is chmod 0600 after the bind; the directory itself is read by `viola_state::strict::check_endpoint_dir` (the recorded endpoint's parent, through `check_socket_dir`, one std `symlink_metadata`) before a bind (`viola run`, `revive`'s start) and before each client's connect (the CLI frame verbs, `hook`, MCP's `frame`), a symlink, a non-directory, another owner or a group or other bit refused and never chmodded, an absent path passing."
    sidecar: "Stack and Technologies, Wrapper IPC: the per-user socket directory is now verified before a bind and before each client's connect, by std `symlink_metadata`, with no new dependency (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The row names how the Unix socket directory is provided but not the check this chunk lands on it; report Changes, Symbols / APIs: `check_socket_dir`, `check_endpoint_dir` and `socket_dir_verdict`, with the bind and the three client seams, and Dependencies: none added."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 39
    section: "## Stack and Technologies"
    change: "Add one item after this one: - Wrapper IPC: on Unix the peer's euid comes from the same interprocess call, `StreamCommon::peer_creds().euid()` (`viola_channel::client::peer_euid_of`; none on Windows, where the DACL and the pid comparison stay the controls), read on both sides: `Client::server_euid` and `TokioClient::server_euid` at each frame-writing client's connect, and `accept_loop` on each incoming stream, which `viola_channel::peer_admitted` drops before its worker thread and before any frame; an unreadable peer is refused where euid is reported, a client's refusal reads `server-verify-failed`, and on macOS, which reports no peer pid, the euid is the whole server check (§Standard Contracts, Session liveness)."
    sidecar: "Stack and Technologies, Wrapper IPC: recorded the peer euid read through `peer_creds().euid()`, its use on both client and wrapper sides, and macOS's euid-only server check (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "Line 39 records `peer_creds().pid()` as the channel's identity read, and this chunk uses the same dependency's euid on both ends; report Changes, Symbols / APIs on `peer_euid_of`, `server_euid` and `accept_loop`, Counts on `server-verify-failed` meaning an euid mismatch alone on macOS, Dependencies: none added."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
note: "D-arch-decisions and D-retired read clean here: no library, runtime or locked decision moved (report.md:23), lines 39 and 45 stay true of the pid comparison, and this file holds no owed clause; the chunk's registry rows (D-arch-resources) fall to other categories."
