proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 8
    section: "## Error Handling"
    change: "In the As-landed sentence, replace «and the one-object parse and, on Linux and Windows, the server verification (macOS the recorded gap), and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable` · `server-verify-failed`.» with «and the one-object parse, the socket-directory check, the peer euid check and, on Linux and Windows, the server verification (macOS compares no peer pid), and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable` · `server-verify-failed` · `socket-dir-refused`.»"
    sidecar: "error-handling: the hook's fail-open words gain `socket-dir-refused`, and its checks now name the socket-directory and peer euid checks on every Unix, macOS's gap narrowed to no peer pid."
    rationale: "The hook's `connect` runs the same three checks as the CLI's and reads `detail` `socket-dir-refused` (report.md:16, report.md:72); `hook-decision`'s enum moves 7 to 8 (report.md:30); `server-verify-failed` now reaches macOS as an euid mismatch alone, and macOS's pid gap stays named (report.md:31, report.md:33)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 9
    section: "## Error Handling"
    change: "New item after this line: - A `viola hook` event or dialog whose recorded endpoint's socket directory fails `check_endpoint_dir`, run before `Client::connect` (`NotSent::SocketDir`, `Asked::SocketDir`), writes no frame and exits 0 with empty streams; its one `hook-decision` line, at WARN, carries `socket-dir-refused` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`), and the directory stays as found. A peer of another euid, where the euid is reported, is `NotSent::ServerUnverified` and reads `server-verify-failed`, on macOS that mismatch alone."
    sidecar: "error-handling: the hook's refusal on a refused socket directory and on a peer of another euid."
    rationale: "New fail-open path of this chunk, its order, words and record stated at report.md:16 and report.md:72; the check never chmods (report.md:11); the macOS reach at report.md:31."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 12
    section: "## Error Handling"
    change: "Replace «(the connected peer is not the recorded wrapper, or the check cannot be made)» with «(`peer_admitted(client.server_euid())` refuses the peer, on macOS the whole of the check, the connected peer is not the recorded wrapper, or the check cannot be made)»."
    sidecar: "error-handling: an MCP frame tool's server check now includes the peer euid decision, and reaches macOS by it alone."
    rationale: "`frame` now reads `!peer_admitted(client.server_euid()) || !server_verified(…)` into `UnreachableCause::ServerVerify` (report.md:17); `server-verify-failed` on macOS means an euid mismatch alone (report.md:31)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 13
    section: "## Error Handling"
    change: "New item after this line: - A `viola mcp` frame tool runs `check_endpoint_dir` on the live endpoint after `live_endpoint`; a refused check sends no frame and returns `result::unreachable(UnreachableCause::SocketDir)`: `isError` `instance-unreachable` with `detail` `socket-dir-refused`, text `refused: instance-unreachable`, and no log line."
    sidecar: "error-handling: the MCP frame tools' refusal on a refused socket directory."
    rationale: "New refusal at MCP's `frame`, no frame, no line, `isError` the record (report.md:17, report.md:73), the word on MCP less `v` (report.md:111)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 20
    section: "## Error Handling"
    change: "New item after this line: - A stream `accept_loop` refuses by `admitted(peer_euid_of(stream))` is dropped on the accept thread, before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no reply, no line and nothing on the terminal, there being no frame to key a `channel-response` on."
    sidecar: "error-handling: the wrapper's drop of a refused peer has no error body and no line."
    rationale: "The server-side refusal's error path, stated at report.md:13 and report.md:75; `channel-response`'s `detail` stays closed, so this entry records why no fault word appears."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 22
    section: "## Error Handling"
    change: "New item after this line: - The CLI's `connect`, behind `send` · `wait` · `last` · `answer` · `pause` · `release` · `link` · `unlink`, runs `check_endpoint_dir` on the recorded endpoint before `Client::connect`: a refusal (`CallError::SocketDirRefused`, `Unreachable::SocketDir`) exits 21 with `detail` `socket-dir-refused`, `during` `connect`, a `process-exit` at WARN, no frame and no `channel-request`, the directory left as found. After the connect, a refused `peer_admitted(client.server_euid())` is `CallError::ServerUnverified` and reads `server-verify-failed`, ahead of `verify_server`."
    sidecar: "error-handling: the CLI frame verbs' refusal on a refused socket directory and on a peer of another euid."
    rationale: "New refusal at the CLI's `connect` and its 8 callers (report.md:15), its record at report.md:71; a refused directory stays as found (report.md:11)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 23
    section: "## Error Handling"
    change: "Replace «a closed set of five causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`:» with «a closed set of six causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`:»."
    sidecar: "error-handling: exit 21's closed causes five to six, the sixth `socket-dir-refused`."
    rationale: "Exit 21's closed causes move five to six, and a row reading «five closed…» changes (report.md:28); the founder's own word on the new closed word (report.md:93)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 25
    section: "## Error Handling"
    change: "New item after this line: - A refused socket directory prints `unable: the socket directory for <name> cannot be trusted` and, last on stderr in plain ASCII with no SGR, `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` (`send_hint` cause `socket-dir`), `<name>` the parsed `ViolaName`, no path, pid or uid. The wording is a planning form, the founder's owed at Epoch 7's boundary batch (`inputs#I3`, `inputs#I4` item 5)."
    sidecar: "error-handling: the refused socket directory's pair, its wording PROVISIONAL, a planning form."
    rationale: "The new pair and its checks (report.md:76), the hint cause (report.md:19), the wording owed (report.md:49, report.md:94)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 27
    section: "## Error Handling"
    change: "New item after this line: - `viola run` and `viola revive`'s start run `socket_dir_ready` on the endpoint's directory (`check_endpoint_dir`, the parent created only when absent, then the check again, so a failed create and a refused check read one code) before any snapshot, child or `Server::bind`: a refusal exits 1 through `refuse_socket_dir`, its `process-exit` and `revive --json` carrying `socket-dir-refused`, and the directory stays as found. A `Server::bind` failure stays what it was."
    sidecar: "error-handling: the start's exit-1 refusal on a refused socket directory, ahead of any bind."
    rationale: "New refusal at the start (report.md:14), its acceptance and record (report.md:70, report.md:107), `revive --json`'s eleventh code (report.md:29), one code for a failed create and a refused check (report.md:95)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 30
    section: "## Error Handling"
    change: "New item after this line: - `Refused::NotADirectory` carries the fixed `Display` message `a trusted state path is not a directory`; `check_socket_dir` returns it for a path that is no directory, `Refused::Symlink` for a symlink and `Refused::Unreadable` for an unstattable path."
    sidecar: "error-handling: `Refused` gains the fixed-message variant `NotADirectory`; the `thiserror` derives stay nine."
    rationale: "New fixed-message variant and the verdict's order (report.md:11); a variant, not a derive, so line 31's nine stands (report.md:36)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
note: "D-security-input, D-security-auth and D-security-deps read no violation in this file (no dependency added, report.md:23; each new check a closed decision, report.md:70-75); the body's own §Error Handling, .andromeda/security-plan.md:413, still lists the five causes and is outside this file."
