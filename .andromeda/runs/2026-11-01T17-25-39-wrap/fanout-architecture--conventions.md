proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 33
    section: '## Conventions'
    change: 'In the Unreachable cause item, the closed set reads six: `StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir`, each word through `as_str`, `SocketDir` the word `socket-dir-refused`; the rest of the item stands (no serde path, no `RefusalReason`).'
    sidecar: 'Conventions: `UnreachableCause` gains `SocketDir` (`socket-dir-refused`), exit 21 closed causes five to six.'
    rationale: 'The report adds the sixth cause, no serde path and no `RefusalReason`, and moves exit 21 closed causes five to six (report.md:10, report.md:28).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:10
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 118
    section: '## Conventions'
    change: 'In the MCP item, the `instance-unreachable` cause words read `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`.'
    sidecar: 'Conventions: MCP `instance-unreachable` cause list gains `socket-dir-refused`.'
    rationale: 'A list of the five causes extends with the sixth, and the word reaches MCP less `v` (report.md:28, report.md:111).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 94
    section: '## Conventions'
    change: 'In the exit 21 hints item, `src/human/hints.rs` also holds the socket-directory hint, and `hint_cause` keys `not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify` · `socket-dir`; `not-running` stays the key of the cause the log reads `instance-dead`.'
    sidecar: 'Conventions: exit 21 hint causes gain `socket-dir`.'
    rationale: 'The report lands `send_hint` cause `socket-dir` and the client hint cause of the same name (report.md:15, report.md:19).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:19
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 42
    section: '## Conventions'
    change: 'In the `--json` code list of `client::refused_document`, add `socket-dir-refused` after `squatted-name`, the list then holding eleven codes; this line stands at 1997 chars, so break it before «under `--json` each of these revive refusals» into a space-indented continuation line, as the item already does, keeping every line within 2 000 chars.'
    sidecar: 'Conventions: `revive --json` exit-1 codes ten to eleven, `socket-dir-refused` added.'
    rationale: 'The report moves the exit-1 codes of `revive --json` ten to eleven, and each list holding all ten extends (report.md:29); the start refusal is shared with `revive` (report.md:14).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: '## Conventions'
    change: 'Replace the item whole: - Exit `1` also covers `viola run`''s start and `viola revive`''s start arm refusing the socket directory (`src/cmd/run.rs` `socket_dir_ready`: `check_endpoint_dir`, the parent created only when absent, then checked again; a refused directory stays as found, nothing chmods it), before any snapshot, child or `Server::bind`: `unable: the socket directory for <name> cannot be trusted` / `hint: viola''s socket directory must be a directory you own, readable only by you; viola does not change its permissions` on stderr, no path, pid or uid (a planning form, the founder''s wording owed at Epoch 7''s boundary batch, `inputs#I3`), one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, and under `viola revive --json` the `socket-dir-refused` document. A failed create reads the same code; a `Server::bind` failure stays what it was.'
    sidecar: 'Conventions: the socket-directory absence and its OPEN retire, replaced by the built `socket-dir-refused` start refusal on exit 1.'
    rationale: 'The report retires the socket-directory check and code owed to the route entry (report.md:26, report.md:33) and lands the refusal, its pair and its line (report.md:14, report.md:70, report.md:76, report.md:95).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 100
    section: '## Conventions'
    change: 'Replace the item whole: - An exception: `server-verify-failed` is claimed on Linux, Windows and macOS, where on macOS it means an euid mismatch alone; macOS reports no peer pid, so the pid and start-time comparison stays absent there, `v1-18`''s named carve-out, owned by no route entry.'
    sidecar: 'Conventions: `server-verify-failed` reach widens to macOS on euid alone; the macOS pid gap stays named, owned by no route entry.'
    rationale: 'The word moves from off macOS to all three OSes (report.md:31), and the owner clause is discharged except the macOS pid gap, named and unowned (report.md:33, report.md:68, report.md:110).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 88
    section: '## Conventions'
    change: 'In the server-identity item, the parenthesis reads: (on Unix the connected peer''s euid is not this user''s or cannot be read, `peer_admitted(client.server_euid())` before `verify_server`; on Linux and Windows also its pid or OS start time is not the snapshot''s, or the check cannot be made; on macOS an euid mismatch alone); the rest of the item stands.'
    sidecar: 'Conventions: exit 21 `server-verify-failed` on the frame verbs now includes the Unix peer euid check.'
    rationale: 'The CLI `connect` reads the euid before `verify_server` and a refused euid reads `server-verify-failed` (report.md:12, report.md:15, report.md:31, report.md:93).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 125
    section: '## Conventions'
    change: 'In the frame-tool server item, the parenthesis reads: (its euid is not this user''s or cannot be read on Unix, `peer_admitted(client.server_euid())`; on Linux and Windows also its pid or OS start time is not the snapshot''s, or the check cannot be made, the start time read on `tokio::task::spawn_blocking`; on macOS an euid mismatch alone); the rest of the item stands.'
    sidecar: 'Conventions: MCP frame tools take the Unix peer euid check into their server check.'
    rationale: 'MCP `frame` maps a refused euid or a failed server check to `UnreachableCause::ServerVerify` (report.md:17, report.md:31).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 90
    section: '## Conventions'
    change: 'A new item under `conv-exit-codes`, in its own detail block: - Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, a refused socket directory (`src/cmd/client.rs` `connect`: `check_endpoint_dir` on the recorded endpoint before `Client::connect`; a symlink, a non-directory, a foreign owner or a group or other bit refused, an absent path passing, the directory left as found): no frame and no `channel-request`, `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, and `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` under `--json` or the verb''s `instance-unreachable` line with the socket-directory hint last on stderr; no path, pid or uid. The founder''s own word, `inputs#I2` Q1.'
    sidecar: 'Conventions: exit 21 on the eight CLI frame verbs covers a refused socket directory, `socket-dir-refused`.'
    rationale: 'The report lands the check at the eight frame verbs with this record and these acceptance results (report.md:11, report.md:15, report.md:71, report.md:93, report.md:107).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 112
    section: '## Conventions'
    change: 'At the end of the exit 21 CLI mapping item, add: `CallError::SocketDirRefused` reads `Unreachable::SocketDir` (`during` `connect`, hint cause `socket-dir`), and a refused euid reads `CallError::ServerUnverified`.'
    sidecar: 'Conventions: exit 21 CLI mapping names `SocketDirRefused` and a refused euid.'
    rationale: 'The report states both mappings at the CLI `connect` (report.md:15).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 133
    section: '## Conventions'
    change: 'A new item under `conv-mcp-results`, in its own detail block: - A frame tool (`send`, `wait`, `last`, `answer`), after `live_endpoint`, runs `check_endpoint_dir` on the live endpoint: a refusal returns the `instance-unreachable` `isError` result with `detail: "socket-dir-refused"`, text `refused: instance-unreachable`, and writes no frame and no line, the `isError` result the record.'
    sidecar: 'Conventions: MCP frame tools refuse a loosened socket directory with `socket-dir-refused`.'
    rationale: 'MCP `frame` gains the check after `live_endpoint`, with no frame, no line and `isError` the record (report.md:17, report.md:73).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
note: 'Outside this file, the body still states the five causes at .andromeda/architecture.md:226, the ten codes at .andromeda/architecture.md:229 and the server check without the euid at .andromeda/architecture.md:158.'
