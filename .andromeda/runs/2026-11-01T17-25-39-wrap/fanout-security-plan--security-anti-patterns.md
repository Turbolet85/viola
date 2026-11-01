proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 8
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "In the Linux/FreeBSD parenthetical, replace the clause «the client-side euid check owed to the route entry Unix endpoint and home hardening» with «the client's euid decision `peer_admitted(client.server_euid())`, a peer of another euid or one it cannot read refused, reading `server-verify-failed`»; the macOS parenthetical stays as written, the euid and the verified 0700 socket directory its checks and the absent pid comparison its named gap."
    sidecar: "security-anti-patterns · Authentication frame ban: the client-side euid check reads as built, no longer owed to the route entry; macOS's named gap kept."
    rationale: "report.md:26 retires the dated gap of the client euid check; report.md:12 and report.md:15 state the decision and its `server-verify-failed` reading; report.md:33 keeps a row stating macOS compares no peer pid."
    basis: src/cmd/client.rs:177-179
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 9
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "Replace the closing «each verifying the server on Linux and Windows (the CLI verbs through `cmd::client::connect`); only every client's euid check and macOS's pid and start-time check stay owed to Unix endpoint and home hardening» with «each, on Unix, checking the recorded endpoint's directory (`check_endpoint_dir`) and the peer's euid (`peer_admitted`), and verifying the server's pid and start time on Linux and Windows (the CLI verbs through `cmd::client::connect`, the hook through its own `connect`, MCP in `frame`); macOS compares no peer pid or start time, the named carve-out, owned by no route entry»."
    sidecar: "security-anti-patterns · Authentication frame ban detail: each client seam's directory and euid checks read as built; macOS's pid and start-time comparison stays named, its route-entry owner gone."
    rationale: "report.md:26 retires the owed clauses; report.md:15, report.md:16 and report.md:17 state the checks at the three client seams in order; report.md:33 leaves macOS's pid gap named but owned by no route entry."
    basis: crates/viola-mcp/src/tools.rs:345-347
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 9
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "Add after this item: «- NEVER bind a `viola run` endpoint or write a channel frame through a socket directory that has not passed `viola_state::strict::check_socket_dir` on Unix (one `symlink_metadata`: a symlink, a non-directory or a directory failing the read rule is refused, an absent path passing), and NEVER chmod a directory it refused: a refused directory stays as found. `viola run` and `revive`'s start create the parent only when absent and check again, refusing `socket-dir-refused` before any snapshot, child or `Server::bind`; a CLI frame verb reads exit 21 `socket-dir-refused`, `viola hook` exits 0 with empty streams and one `hook-decision` at WARN, and MCP's `frame` returns that cause with no line, each writing no frame. The check sits at each seam, never inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge.»"
    sidecar: "security-anti-patterns · Authentication: new ban on binding or framing through an unchecked socket directory and on repairing a refused one."
    rationale: "report.md:11 states the check and that it never chmods; report.md:14, report.md:15, report.md:16 and report.md:17 state each seam's refusal; report.md:53 states why no check sits in `Client::connect`; report.md:62 expects §Security Anti-Patterns carried."
    basis: crates/viola-state/src/strict.rs:149-168
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 39
    section: "## Security Anti-Patterns (NEVER do these) › ### Universal"
    change: "Add after this item: «- NEVER add an env, flag or feature seam to force a refusing peer-euid decision: one account can make no foreign peer, so the decisions are proven over injected values (`peer_euid_admitted(observed, own, reported)`, the wrapper's private `serve_admitting(dispatch, admitted)` over an `Admission` function), and each client seam's `peer_admitted` wiring (`cmd::client::connect`, `cmd::hook::connect`, `viola-mcp`'s `frame`) stays the named weakest proof, read only through a run's forced refusal, no committed test repeating it.»"
    sidecar: "security-anti-patterns · Universal: the peer-euid decisions are proven by injected values with no seam; the client seams' wiring recorded as the named weakest proof."
    rationale: "report.md:13 states the injection and that it is no env, flag or feature seam; report.md:54 binds the wiring's survivor to this section's no-seam rule."
    basis: crates/viola-channel/src/client.rs:151-156
note: "D-security-input, D-security-auth and D-security-deps found no violation (no new dependency, library unchanged, the new surfaces closed decisions and lstat checks); the same retired owed clause also stands in the body's §Authentication & Authorization client verification row and its §Security Anti-Patterns › Authentication frame ban, outside this file so no proposal is written here."
