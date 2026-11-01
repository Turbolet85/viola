proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: '## Input Validation'
    change: 'In **Frame verbs:**, the sentence on a passed check now reads: A passed check is followed by the liveness pre-check `live_endpoint` (the endpoint, `pid` and `started_at`), `check_endpoint_dir` on the parent of that endpoint (a refusal exit 21, `socket-dir-refused`, no frame), the connect, on Unix the peer euid decision (a refusal `server-verify-failed`) and then, on Linux and Windows, the server check before the frame (IPC client-side server verification row).'
    sidecar: 'input-validation CLI row: the frame verbs now run the socket-directory check and the peer euid decision between the liveness pre-check and the server check.'
    rationale: 'The row orders the frame verbs as liveness then server check; report.md:15 puts `check_endpoint_dir`, `Client::connect` and `peer_admitted` in that path, report.md:71 gives the exit and word, and report.md:12 confines the euid read to Unix.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 37
    section: '## Input Validation'
    change: 'After the sentence on the `viola mcp` tools running `check_instance` before `live_endpoint`, add: Then `frame` runs `check_endpoint_dir` (`socket-dir-refused`) and the peer euid decision (`server-verify-failed`), each `instance-unreachable`, no frame and no line.'
    sidecar: 'input-validation CLI row: the MCP frame tools now run the socket-directory check after `live_endpoint` and the peer euid decision beside the server check.'
    rationale: 'The row stops the MCP path at `check_instance` and `live_endpoint`; report.md:17 adds `check_endpoint_dir` after `live_endpoint` as `UnreachableCause::SocketDir` and `peer_admitted` as `UnreachableCause::ServerVerify`, with no frame and no line.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 39
    section: '## Input Validation'
    change: 'The closed exit-1 list of `viola revive --json` gains its eleventh code, so it reads: `strict-modes-failed` · `socket-dir-refused` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`.'
    sidecar: 'input-validation `revive --json` codes: ten to eleven, adding `socket-dir-refused`.'
    rationale: 'report.md:29 moves the codes from ten to eleven and says each list holding all ten extends; report.md:14 has the start of `revive` share the refusal.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 42
    section: '## Input Validation'
    change: 'In **Home resolution:**, the clause on `board::scan` now reads: `viola list`''s `board::scan` then runs `check_read` on the home (on every OS, PROVISIONAL: the overseer as delegate, `inputs#I2` Q3) and `check_path` on `instances/` (the rest of the sentence unchanged).'
    sidecar: 'input-validation Home resolution: the two rules of `viola list` on the home retire; the home takes the read rule on every OS, PROVISIONAL.'
    rationale: 'report.md:26 retires the two rules on the home, report.md:18 states `check_read` on the home and `check_path` on `instances/` with order and no-open unchanged, report.md:32 names this line, and report.md:94 records the word as the delegate and PROVISIONAL.'
    basis: .andromeda/security-plan-detail/input-validation.md:42
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 14
    section: '## Bootstrap phases (derive for route / setup-project)'
    change: 'The last clause, «the `peer_creds` decision and the directory verification stay with their route entries», now reads: the `peer_creds` decision (`peer_admitted`, the wrapper dropping a refused peer in `accept_loop` before any frame) and the directory verification (`check_socket_dir` and `check_endpoint_dir`, in `viola-state`, called at the seams) landed with chunk 2026-11-01-unix-endpoint-and-home-hardening.'
    sidecar: 'bootstrap auth-scaffolding-baseline: the peer euid decision and the directory verification are built, no longer owed to a route entry.'
    rationale: 'report.md:26 retires the client euid check and the socket-directory check as built; report.md:13 builds the drop, report.md:11 the directory check, and report.md:53 places no check inside `Client::connect`, since `viola-channel` has no `viola-state` edge.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 15
    section: '## Bootstrap phases (derive for route / setup-project)'
    change: 'After the peer pid parenthetical add: and the peer euid (`Client::server_euid`, `TokioClient::server_euid`: `peer_creds().ok()?.euid()` on Unix, none on Windows; the decision `peer_admitted`, on Linux and macOS, a refusal reading `server-verify-failed`); and the closing clause now reads: then makes the liveness pre-check, the socket-directory check on the parent of the recorded endpoint (`check_endpoint_dir`), the connect, the peer euid decision and the server check.'
    sidecar: 'bootstrap auth-scaffolding-baseline: the client server verification now holds the peer euid, and the client order names the directory check and the euid decision.'
    rationale: 'The row lists the client read as pid alone and the order as liveness then server check; report.md:12 adds the euid read and decision, report.md:15, report.md:16 and report.md:17 give the order on the three seams, and report.md:93 gives the word on Linux and macOS.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 16
    section: '## Bootstrap phases (derive for route / setup-project)'
    change: 'The closing of the `viola-state` list now reads: `check_present` (the `conpty/` companions); `check_socket_dir` and `check_endpoint_dir` (the socket directory, one `symlink_metadata` read, never a chmod, off Unix `Ok(())`); and the `Readable`, `Symlink` and `NotADirectory` refusals; the Windows clause that follows unchanged.'
    sidecar: 'bootstrap auth-scaffolding-baseline: the strict-modes list in `viola-state` gains the socket-directory checks and the `NotADirectory` refusal.'
    rationale: 'The row enumerates the strict-modes checks and refusals of `viola-state`; report.md:11 adds `check_socket_dir`, `check_endpoint_dir`, `socket_dir_verdict` and the fixed-message variant `Refused::NotADirectory`, with no chmod and `Ok(())` off Unix.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
note: 'D-security-input, D-security-auth and D-security-deps read no drift; no F{n} falls in this group''s categories; the report does not say whether `check_read` goes through `symlink_metadata`, so the parenthetical kept at input-validation.md:42 rests only on its order and no-open staying unchanged (report.md:18).'
