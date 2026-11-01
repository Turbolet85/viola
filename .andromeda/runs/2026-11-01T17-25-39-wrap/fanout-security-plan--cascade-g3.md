proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 8
    section: "## Error Handling"
    change: "Three clause edits only. The first parenthesis reads «(server verification mismatch, refused directory, oversized stdin, strict-modes failure)». «the one-object parse and, on Linux and Windows, the server verification (macOS the recorded gap)» reads «the one-object parse and, on Unix, the socket-dir and peer euid checks and, on Linux and Windows, the server verification (macOS the recorded gap)». The `hook-decision` detail list ends «· `server-verify-failed` · `socket-dir-refused`»."
    sidecar: "error-handling: the hook's fail-open row gains the socket-directory and peer euid checks and the detail `socket-dir-refused`."
    rationale: "`src/cmd/hook.rs` `connect` runs the directory check, the connect and the peer euid decision before the server check and fails open with `socket-dir-refused` (report.md:16, report.md:72); macOS's missing pid comparison stays a true claim with its gap wording kept (report.md:33)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "- A hook meeting a refused socket directory is `NotSent::SocketDir` and `Asked::SocketDir`: exit 0, empty streams, no frame, one `hook-decision` of `detail` `socket-dir-refused` at WARN, on the dialog path `hook-invoked` with a null `corr` first and `decision_emitted` false."
    sidecar: "error-handling: new detail, the hook's record of a refused socket directory."
    rationale: "The report states these variants and this record (report.md:16, report.md:72); the row on a hook's failed server check is its sibling."
    fact: F18
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 12
    section: "## Error Handling"
    change: "The parenthesis «(the connected peer is not the recorded wrapper, or the check cannot be made)» reads «(the connected peer's euid is refused, the whole of the check on macOS; it is not the recorded wrapper; or the check cannot be made)»."
    sidecar: "error-handling: MCP's failed server check now includes a refused peer euid, on macOS that alone."
    rationale: "`frame` refuses on `!peer_admitted(client.server_euid()) || !server_verified(…)` as `UnreachableCause::ServerVerify` (report.md:17); `server-verify-failed` now reaches macOS, an euid mismatch alone (report.md:31)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 13
    section: "## Error Handling"
    change: "Append to the row «With a live endpoint it then runs `check_endpoint_dir` on that endpoint's parent: a refusal sends no frame and returns the same unreachable result, `detail` `socket-dir-refused`, and no log line.»"
    sidecar: "error-handling: MCP's `frame` gains the socket-directory check after `live_endpoint`."
    rationale: "`check_endpoint_dir` runs after `live_endpoint` at `crates/viola-mcp/src/tools.rs:345-347`, to `result::unreachable(UnreachableCause::SocketDir)`, no frame, no line, `isError` the record (report.md:17, report.md:73)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 23
    section: "## Error Handling"
    change: "«a closed set of five causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`» reads «a closed set of six causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`»."
    sidecar: "error-handling: exit 21's closed causes move from five to six, the sixth `socket-dir-refused`."
    rationale: "Exit 21's closed causes moved five to six, a row reading five changes (report.md:28)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 23
    section: "## Error Handling"
    change: "- A refused euid has no word of its own: it is `CallError::ServerUnverified`, `NotSent::ServerUnverified` and `UnreachableCause::ServerVerify` on the three client seams, reading `server-verify-failed`."
    sidecar: "error-handling: new detail, a refused euid reads `server-verify-failed` on the three client seams."
    rationale: "The report maps a refused euid to these variants at each seam (report.md:15, report.md:16, report.md:17); it belongs beside the closed cause list."
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 24
    section: "## Error Handling"
    change: "- `viola_core::UnreachableCause::SocketDir`, word `socket-dir-refused`, has no serde path and no `RefusalReason`; on a CLI frame verb it is `CallError::SocketDirRefused` to `Unreachable::SocketDir`: exit 21, `during` `connect`, hint cause `socket-dir`, one `process-exit` at WARN and no `channel-request`."
    sidecar: "error-handling: new detail, the sixth cause and its record on a CLI frame verb."
    rationale: "The report states the variant, its word and the frame verbs' record (report.md:10, report.md:15, report.md:71); the row on `UnreachableCause` having no serde path is its place."
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 25
    section: "## Error Handling"
    change: "- The socket-directory refusal names only the parsed `ViolaName`, and its lines hold no path, pid or uid; its wording is a planning form, the founder's wording owed at Epoch 7's boundary batch."
    sidecar: "error-handling: new detail, what the socket-directory refusal names and its wording's standing."
    rationale: "The new pair names only the parsed `ViolaName` (report.md:76), its lines hold no path, pid or uid (report.md:116), and the wording is owed to the founder (report.md:94)."
    fact: F19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 27
    section: "## Error Handling"
    change: "- `viola run` and `viola revive`'s start refuse a socket directory with exit 1 and one `process-exit` of `detail` `socket-dir-refused`, a step with no span, through `refused_with` and `refuse_socket_dir`."
    sidecar: "error-handling: new detail, the start's socket-directory refusal."
    rationale: "`start` refuses through `refused_with` and `refuse_socket_dir`, and `revive`'s start shares it (report.md:14, report.md:70); the row on the start's strict-modes refusal is its sibling."
    fact: F16
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 413
    section: "## Error Handling"
    change: "The `instance-unreachable` closed cause list ends «`server-verify-failed` · `socket-dir-refused`»."
    sidecar: "security-plan §Error Handling: MCP's closed `instance-unreachable` causes gain `socket-dir-refused`."
    rationale: "A row listing the causes extends with the sixth word (report.md:28), and `frame` returns it as an `isError` result (report.md:17)."
    dependent-of: D-cascade
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 8
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "The clause «the client-side euid check owed to the route entry \"Unix endpoint and home hardening\"» reads «the client-side euid check built, `peer_admitted(client.server_euid())` at the three client seams, before the server verification»; the macOS parenthesis stays."
    sidecar: "security-anti-patterns: the frame ban's client euid check is built, no longer owed."
    rationale: "The client euid check, owed to that route entry, is retired as built (report.md:26), its decision at `src/cmd/client.rs:177-179`, `src/cmd/hook.rs:236-238` and `crates/viola-mcp/src/tools.rs:352`; the macOS row stays a true claim (report.md:33)."
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 9
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "The row's end reads «each verifying the server on Linux and Windows and, by the peer euid alone, on macOS (the CLI verbs through `cmd::client::connect`); only macOS's pid and start-time check stays absent, the named carve-out, owned by no route entry.»"
    sidecar: "security-anti-patterns: the frame ban's owed clause ends; macOS's pid and start-time check stays the one named carve-out, owned by no route entry."
    rationale: "The dated gaps are retired as built, macOS's pid gap staying named but owned by no route entry (report.md:26, report.md:33); `server-verify-failed` now reaches macOS as an euid mismatch alone (report.md:31)."
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan.md
    near: 442
    section: "## Security Anti-Patterns (NEVER do these) › ### Authentication"
    change: "The exception reads «except the named carve-out: macOS's pid and start-time check (no peer pid), owned by no route entry; no other frame or process may borrow it.»"
    sidecar: "security-plan §Security Anti-Patterns: the frame ban's exception narrows to macOS's pid and start-time check, owned by no route entry."
    rationale: "Every client's euid check is built and leaves the exception, leaving macOS's pid and start-time check alone (report.md:26, report.md:33)."
    dependent-of: D-retired
note: "D-security-input, D-security-auth and D-security-deps fire nothing here, as the report adds no dependency and no off-spec auth library; the disproved note in `diagnostics/` is §Authentication's, not this group's; the pick trail is not in this detector's inputs, so the orchestrator reports it."
