proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 14
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "Replace the line's last clause «the `peer_creds` decision and the directory verification stay with their route entries» with «the `peer_creds` decision and the directory verification landed with chunk 2026-11-01-unix-endpoint-and-home-hardening, the verification held in `viola-state` and called by the `viola` bin and `viola-mcp`, never inside `viola_channel::Client::connect`, `viola-channel` having no `viola-state` edge», the rest of the line unchanged."
    sidecar: "Bootstrap phases: the `peer_creds` decision and the socket-directory verification read as landed, no longer as owed to their route entries (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's **Retired** bullet names the client euid check, the socket-directory check and the code owed to the route entry Unix endpoint and home hardening as built; **Reverted / negative API facts** places no check inside `Client::connect`."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 14
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "New item after the line: «  - `viola-channel` wrapper, the peer euid decision on every accept: `Server::serve` calls private `serve_admitting(dispatch, peer_admitted)`, and `accept_loop(listener, dispatch, admitted)` filters `incoming()` by `admitted(peer_euid_of(stream))`; a refused stream is dropped on the accept thread, before its worker thread and before any frame, reading end-of-file, writing no line and nothing on the terminal. The decision is injected as a plain function: no env, flag or feature seam.»"
    sidecar: "Bootstrap phases: the wrapper's peer euid drop in `accept_loop` recorded (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's **Symbols / APIs** names `accept_loop`'s filter and the drop before any frame; **Coverage of new surfaces** holds the wrapper's drop as a closed decision writing nothing."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 15
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "New item after the line: «  - `viola-channel` client, the euid read and decision: `Client::server_euid` and `TokioClient::server_euid` (`peer_creds().ok()?.euid()` on Unix, none on Windows, where the DACL and the pid comparison stay the controls), writing and logging nothing; `peer_admitted` over `peer_euid_admitted(observed, own, reported)`, `!reported || observed == Some(own)`, so a peer it cannot read is refused where `PEER_EUID_REPORTED` (`cfg!(unix)`) holds. Each client seam (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `viola-mcp`'s `frame`) runs `check_endpoint_dir` on the recorded endpoint, connects, takes `peer_admitted`, then verifies the server, sending no frame on any refusal. A refused directory reads the sixth closed cause `socket-dir-refused`: exit 21 on the CLI frame verbs with one `process-exit` line, exit 0 with empty streams and one `hook-decision` at WARN on the hook, an `isError` record and no line on MCP. A refused euid reads `server-verify-failed` on every seam, on macOS an euid mismatch alone: macOS compares no peer pid, the named carve-out, owned by no route entry.»"
    sidecar: "Bootstrap phases: the client euid read and decision and the socket-directory check at the three client seams recorded, with their closed words and macOS's named carve-out (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's **Symbols / APIs** gives the read, the decision and each seam's order; **Counts / qualifiers moved** gives the six causes, `server-verify-failed`'s reach and macOS's pid gap staying named with no owner; the founder's own word on the two closed words is `inputs#I2` Q1 and Q2."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 16
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "New item after the line: «  - `viola-state`, the socket-directory check (`viola_state::strict`): `check_socket_dir(dir)` takes one `symlink_metadata`, an absent path passing and an unstattable one `Refused::Unreadable`, then `socket_dir_verdict(is_symlink, is_dir, mode, uid, euid)`: a symlink `Refused::Symlink`, not a directory the new fixed-message `Refused::NotADirectory`, else the Unix read verdict. Off Unix it returns `Ok(())`. `check_endpoint_dir(endpoint)` runs it on a recorded endpoint's parent. The check never chmods: a refused directory stays as found.»"
    sidecar: "Bootstrap phases: `check_socket_dir`, `check_endpoint_dir`, `socket_dir_verdict` and `Refused::NotADirectory` recorded in `viola_state::strict` (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's **Symbols / APIs** names the three functions, the verdict's order, the new variant and the never-chmod rule; line 16 lists `viola-state`'s strict-modes refusals and holds none of them."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 16
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "New item after the line: «  - `viola-state`, the board's one rule on the home: `viola list`'s `scan` holds the home through `check_read` on every OS, `instances/` staying `check_path`; order, the whole-board `ScanRefused::Home` and no-open unchanged; `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home. PROVISIONAL: the overseer's answer as the founder's delegate (`inputs#I2` Q3), the founder's own word owed at Epoch 7's boundary batch.»"
    sidecar: "Bootstrap phases: `viola list`'s home held to the read rule on every OS recorded, PROVISIONAL on the delegate's answer (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's **Symbols / APIs** gives `scan`'s `check_if_present(path, check)` and the callers keeping the write rule; **Retired** names `viola list`'s two rules on the home, and line 16 names the read rule's places without the board's; `inputs#I3` card row 2 holds the PROVISIONAL and its owner."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 18
    section: "## Bootstrap phases (derive for route / setup-project)"
    change: "New item after the line: «  - `viola run`, the socket directory before the bind: `socket_dir_ready(endpoint)` runs `check_endpoint_dir`, creates the parent only when absent, then checks again, so a race's loser reads the same; a failed create and a refused check read one code. `start`, `revive`'s start sharing it, refuses `socket-dir-refused` at exit 1 with one `process-exit` line, before any snapshot, child or `Server::bind`. A `Server::bind` failure stays what it was.»"
    sidecar: "Bootstrap phases: `viola run` and `revive`'s start refusing an untrusted socket directory before any snapshot, child or bind recorded (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The report's **Symbols / APIs** gives `socket_dir_ready`, the refusal's place on the start chain and `revive`'s sharing; **Decisions & corrections** gives the one code; **Coverage of new surfaces** gives the exit and the line."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
note: "D-security-input, D-security-auth and D-security-deps find no violation in this file (no new dependency, no auth library, each new surface's validation marked in the report's coverage bullets); the body's §Authentication & Authorization and §Security Anti-Patterns still read the client euid and directory checks as owed, outside this file."
