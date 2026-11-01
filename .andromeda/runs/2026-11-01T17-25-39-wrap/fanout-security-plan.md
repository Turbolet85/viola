proposals:
  - detector: D-retired
    severity: warning
    section: Authentication & Authorization
    change: "In the row IPC client-side server verification, replace the closing clause that the client euid check and macOS are owed to \"Unix endpoint and home hardening\" with the built flow: before any frame every client checks the recorded endpoint's directory (`check_endpoint_dir`, a refusal reading `socket-dir-refused`) and, on Linux and macOS, admits only a peer of its own euid, an unreadable peer refused (`peer_admitted`, a refusal reading `server-verify-failed`); macOS compares no peer pid or start time, a named carve-out owned by no route entry, so `server-verify-failed` there means an euid mismatch alone."
    sidecar: "§Authentication & Authorization, client-side server verification: the dated gap to \"Unix endpoint and home hardening\" discharged; socket-directory check and client euid check stated as built, macOS's missing pid comparison kept as a named carve-out with no owner."
    rationale: "Report Retired names the dated gaps (client euid check, socket-directory check) as built; Symbols / APIs gives the order at each client seam, and Counts / qualifiers moved says each owner clause is discharged except macOS's pid gap, which stays named but owned by no route entry."
    basis: src/cmd/client.rs:173-179
  - detector: D-retired
    severity: warning
    section: Security Anti-Patterns (NEVER do these) → Authentication
    change: "In the ban on writing a channel frame before verifying the server, replace the exception that every client's euid check and macOS's pid and start-time check are owed to \"Unix endpoint and home hardening\" with one exception only: macOS's pid and start-time check (no peer pid), a named carve-out owned by no route entry, its euid and verified socket-directory checks standing; no other frame or process may borrow it."
    sidecar: "§Security Anti-Patterns, Authentication: the frame-before-verification ban loses its dated gaps; the client euid check is no longer an exception, and macOS's pid comparison stays as a named carve-out with no owner."
    rationale: "Same retired claim as the §Authentication & Authorization row, restated as a ban exception; report Retired and Counts / qualifiers moved (pattern owner-title, pattern mac-gap) discharge the euid half and keep the macOS half named."
    basis: crates/viola-channel/src/client.rs:151-156
    dependent-of: D-retired
  - detector: D-security-auth
    severity: warning
    section: Error Handling
    change: "In the MCP `isError` row, extend the closed causes of `instance-unreachable` to six: `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`."
    sidecar: "§Error Handling: `instance-unreachable`'s closed causes five to six, adding `socket-dir-refused`, the client's refused socket directory."
    rationale: "The new client check's refusal reaches MCP as `UnreachableCause::SocketDir`; report Symbols / APIs and Counts / qualifiers moved (pattern causes) give the six-word list, and Decisions gives Q1 as the founder's own word."
    basis: crates/viola-core/src/lib.rs:190
  - detector: D-security-auth
    severity: warning
    section: Threat Model Summary
    change: "In Infrastructure, Networking, state the Unix endpoint's place as a verified per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, `/tmp/viola-<uid>/` only as a fallback) in place of Unix domain sockets at «`$TMPDIR` or `/tmp`»; no Linux abstract namespace stays."
    sidecar: "§Threat Model Summary, Networking: the Unix socket's place corrected to the verified per-user 0700 directory; the `$TMPDIR` or `/tmp` placement was disproved."
    rationale: "Report Spec claims disproved measures the threat model's `$TMPDIR` socket (else `/tmp`) false: the endpoint has stood in a per-user 0700 directory since chunk 2026-09-27-wrapper-channel. This body line restates that claim in other words, and the detail's copy is the cascade's."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:43
  - detector: D-security-auth
    severity: warning
    section: Threat Model Summary
    change: "In the tier justification, take `/tmp` out of the shared namespaces whose endpoint names are predictable, leaving Windows pipes; the Unix socket stands in a per-user 0700 directory."
    sidecar: "§Threat Model Summary, tier justification: `/tmp` dropped from the predictable shared namespaces, the same disproved socket placement."
    rationale: "A second restatement of the placement report Spec claims disproved retires. The section's verbatim label is the orchestrator's to weigh, since the report also retires a line in this section's detail."
    basis: .andromeda/security-plan-detail/threat-model-summary.md:43
    dependent-of: D-security-auth
note: D-security-input and D-security-deps hold (no new external-input surface, no dependency added or bumped). The report's disproved «a note in `diagnostics/`» and «sync + Tokio server» wording and the `TMPDIR`-unset row in threat-model-summary.md stand in the detail, not this body, so they are the cascade's. The body's tier justification and Auth model still read that no per-OS enforcement of the same-user boundary is decided; that is rationale older than this chunk and names no item in the report's Retired bullet, so it is not proposed, and the operator may want to weigh it.
