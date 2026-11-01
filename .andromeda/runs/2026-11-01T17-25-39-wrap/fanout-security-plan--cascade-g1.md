proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 43
    section: '## Threat Model Summary'
    change: 'Row now reads: - One endpoint per `viola run`: named pipe `\\.\pipe\viola-<h12>` on Windows, socket file `<dir>/viola-<h12>.sock` on Unix, `<dir>` the per-user 0700 directory (§ Authentication & Authorization, IPC access control (Unix)).'
    sidecar: 'threat-model-summary#2.1: the Unix socket file stands in the per-user 0700 directory, not at the `$TMPDIR` or `/tmp` root.'
    rationale: 'Report, Spec claims disproved: the socket file `$TMPDIR/viola-<h12>.sock` (else `/tmp`) at threat-model-summary.md:43 is false; the endpoint has stood in a per-user 0700 directory since chunk 2026-09-27-wrapper-channel.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:58
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 53
    section: '## Threat Model Summary'
    change: 'Row now reads: - On Unix the socket sits in the per-user 0700 directory, not in world-writable `/tmp`; a directory another user made or loosened is refused by the socket-directory check, with no bind on `run` and no frame on a client.'
    sidecar: 'threat-model-summary#2.1: the consequence of a socket in world-writable `/tmp` retired with the claim it rests on.'
    rationale: 'The row places the socket at the root the report disproves, so it reads the same claim; the report records the refusal of a loosened directory and a foreign owner at `socket_dir_verdict`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 56
    section: '## Threat Model Summary'
    change: 'Keep the row; its last clause «macOS, where the peer reports no pid, stays a recorded gap (owed to "Unix endpoint and home hardening")» now reads: ahead of that comparison, on Linux and macOS, each runs the socket-directory check on the recorded endpoint before connecting (a refusal `socket-dir-refused`, no frame) and the peer euid decision past the connect (a refusal `server-verify-failed`, no frame); macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry.'
    sidecar: 'threat-model-summary#2.1: every client now runs the directory and euid checks; the macOS pid gap stays named, owned by no route entry.'
    rationale: 'Report, Retired: the client euid check and the socket-directory check owed to the route entry are built; Counts: a row stating macOS compares no peer pid stays true, its owner gone.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 60
    section: '## Threat Model Summary'
    change: 'In the row, «one of five closed words» now reads «one of six closed words»; `server-verify-failed` (off macOS) now reads `server-verify-failed` (Linux, Windows and macOS, on macOS an euid mismatch alone); the list ends with `socket-dir-refused` (the socket directory of the recorded endpoint refused).'
    sidecar: 'threat-model-summary#2.1: the closed causes five to six, with `socket-dir-refused`; `server-verify-failed` reaches macOS as an euid mismatch alone.'
    rationale: 'Report, Counts: closed causes five to six; the reach of `server-verify-failed` moves from off macOS to Linux, Windows and macOS.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 100
    section: '## Threat Model Summary'
    change: 'Extend the row: past the liveness pre-check `frame` runs the socket-directory check on the recorded endpoint, a refusal `instance-unreachable` with `detail` `socket-dir-refused`, and past the connect the peer euid decision beside the server check, a refusal with `detail` `server-verify-failed`, each with no frame and no line.'
    sidecar: 'threat-model-summary#6.1: the MCP pre-frame checks gain the directory check and the peer euid decision.'
    rationale: 'Report, Symbols: `crates/viola-mcp/src/tools.rs` `frame` runs `check_endpoint_dir` after `live_endpoint` and reads `peer_admitted` beside `server_verified`, with no frame and no line.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 18
    section: '## Authentication & Authorization'
    change: 'In the row, «CLI/MCP clients report `instance-unreachable` (exit 21), and `hook` fails open (exit 0, no body, a note in `diagnostics/`) without writing any frame» now reads: CLI/MCP clients report `instance-unreachable` with `detail` `socket-dir-refused` (the CLI exit 21), and `hook` fails open (exit 0, empty streams, one `hook-decision` of `detail` `socket-dir-refused` at WARN) without writing any frame.'
    sidecar: 'authentication-authorization#1.4: the record of a hook meeting a refused directory is one `hook-decision` line at WARN, not a note in `diagnostics/`.'
    rationale: 'Report, Spec claims disproved: the chunk writes one `hook-decision` at WARN, the form the server check took; the claim of a note in `diagnostics/` is false.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:59
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 18
    section: '## Authentication & Authorization'
    change: 'In the row, the **Socket-directory failure at bind:** clause now reads: **Socket-directory failure at bind:** a failed create and a refused check read one code: `run` and the start of `revive` exit 1 with `socket-dir-refused` and one `process-exit` of that `detail`; a `Server::bind` failure stays what it was.'
    sidecar: 'authentication-authorization#1.4: the code owed to the route entry for a socket-directory failure at bind is built.'
    rationale: 'Report, Retired: the code owed to "Unix endpoint and home hardening" is built; Decisions: a failed create and a refused check read one code.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:95
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'Row now reads: - **Stack reference:** `viola-channel` server, `#[cfg(unix)]`; interprocess 2.4.4; `viola-state` `strict::{check_socket_dir, check_endpoint_dir, socket_dir_verdict}`, called by `src/cmd/run.rs`, `src/cmd/client.rs`, `src/cmd/hook.rs` and `crates/viola-mcp/src/tools.rs`.'
    sidecar: 'authentication-authorization#1.4: the stack reference names where the directory check lives and its callers.'
    rationale: 'Report, Symbols: the check lives in `viola_state::strict`, with the bind and the three client seams as its callers; `viola-channel` has no `viola-state` edge.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC access control (Unix), the directory verdict ⏎ - **Implementation:** `viola_state::strict::check_socket_dir` makes one `symlink_metadata` read of the path: absent passes, unstattable is `Refused::Unreadable`, a symlink `Refused::Symlink`, a non-directory `Refused::NotADirectory`, else the Unix read verdict on owner and mode. ⏎ - **Stack reference:** `crates/viola-state/src/strict.rs` (`check_socket_dir`, `socket_dir_verdict`)'
    sidecar: 'authentication-authorization: added the verdict of the socket-directory check.'
    rationale: 'Report, Symbols, `viola_state::strict::check_socket_dir`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC access control (Unix), a refused directory ⏎ - **Implementation:** The socket-directory check never chmods: a refused directory stays as found. ⏎ - **Stack reference:** `crates/viola-state/src/strict.rs` (`check_socket_dir`); `tests/security_negatives_socket_dir.rs`'
    sidecar: 'authentication-authorization: added that the socket-directory check leaves a refused directory as found.'
    rationale: 'Report, Symbols: the check never chmods, a refused directory stays as found.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC access control (Unix), the recorded endpoint ⏎ - **Implementation:** `check_endpoint_dir` is the check on a recorded endpoint parent, the one helper the bind and the three client seams share. ⏎ - **Stack reference:** `crates/viola-state/src/strict.rs` (`check_endpoint_dir`); `src/cmd/run.rs`, `src/cmd/client.rs`, `src/cmd/hook.rs`, `crates/viola-mcp/src/tools.rs`'
    sidecar: 'authentication-authorization: added `check_endpoint_dir`, the helper the bind and the client seams share.'
    rationale: 'Report, Symbols and Deviations 2: `check_endpoint_dir` added beside `check_socket_dir`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:80
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC access control (Unix), at a start ⏎ - **Implementation:** `socket_dir_ready` checks, creates the parent only when absent, then checks again, so a race loser reads the same; it runs before any snapshot, child or `Server::bind`, the start of `revive` shares it, a failed create and a refused check read one code, and a `Server::bind` failure stays what it was. ⏎ - **Stack reference:** `src/cmd/run.rs` (`socket_dir_ready`, `refuse_socket_dir`, `enum Bound`)'
    sidecar: 'authentication-authorization: added the order of the socket-directory check at a start.'
    rationale: 'Report, Symbols, `src/cmd/run.rs`; Decisions, the leans the P5 word took.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC access control (Unix), a leftover socket file ⏎ - **Implementation:** A killed wrapper leaves its socket file in the per-user directory until a next bind of that name takes it over: nothing removes it, no owner is proposed, recorded as a harmless file in a 0700 directory. ⏎ - **Stack reference:** `crates/viola-e2e/src/harness/cleanup.rs` (`unconnectable`, which removes no leftover file)'
    sidecar: 'authentication-authorization: added the recorded choice on a killed wrapper leftover socket file.'
    rationale: 'Report, `inputs#I3`, Beyond the plan row 3; Symbols, `unconnectable` removes no leftover file.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:51
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC access control and server verification, provenance ⏎ - **Implementation:** The new word `socket-dir-refused` and `server-verify-failed` on Linux and macOS are the word of the founder himself (`inputs#I2` Q1, Q2, 2026-11-01); the read rule of the board on every OS and the P5 word are the word of the overseer as delegate, PROVISIONAL, the word of the founder owed at the Epoch 7 boundary batch. ⏎ - **Stack reference:** `inputs#I2`, `inputs#I3`, `inputs#I4`'
    sidecar: 'authentication-authorization: added the provenance of the chunk boundary answers, the delegate ones PROVISIONAL.'
    rationale: 'Report, Decisions & corrections: the words of the founder (`inputs#I2` Q1, Q2) and the overseer as delegate, PROVISIONAL (Q3, the P5 word).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:94
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 22
    section: '## Authentication & Authorization'
    change: 'Row now reads: - **Stack reference:** `viola-channel` sync server (`crates/viola-channel/src/server.rs`, `accept_loop`); `TokioClient` is a client, and no Tokio server exists.'
    sidecar: 'authentication-authorization#1.5: the server-side peer check sits in one sync server, the sync + Tokio server claim disproved.'
    rationale: 'Report, Spec claims disproved: the «sync + Tokio server» is false, one sync server is what the code holds.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:60
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 22
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC server-side peer check (Unix), the drop ⏎ - **Implementation:** The drop filters `incoming()` in `accept_loop` on the accept thread, before the worker thread and any frame: the peer reads end-of-file, and no line and nothing on the terminal is written. ⏎ - **Stack reference:** `crates/viola-channel/src/server.rs` (`accept_loop`, `serve_admitting`, `peer_admitted`)'
    sidecar: 'authentication-authorization: added where and how the wrapper drops a refused peer.'
    rationale: 'Report, Symbols, `crates/viola-channel/src/server.rs`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
    fact: F7
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 24
    section: '## Authentication & Authorization'
    change: 'In the row, «(the client-side euid check is not built, owed to the route entry "Unix endpoint and home hardening")» now reads: (built at `cmd::client::connect`, the `connect` of `cmd::hook` and the `frame` of `viola-mcp`, through `peer_admitted`; a peer whose euid cannot be read is refused, and a refused euid reads `server-verify-failed`). The **macOS:** clause stands, its euid and directory checks now built.'
    sidecar: 'authentication-authorization#1.6: the client euid check owed to the route entry is built at the three client seams.'
    rationale: 'Report, Retired: the client euid check is built; Symbols: each seam reads `peer_admitted(client.server_euid())`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 28
    section: '## Authentication & Authorization'
    change: 'Extend the row: `viola-channel` client also holds the euid read and decision (`Client::server_euid` / `TokioClient::server_euid` over `peer_creds().euid()`, `peer_admitted`); `viola-state` `strict::check_endpoint_dir` is called by `cmd::client::connect`, the `connect` of `cmd::hook` and the `frame` of `viola-mcp`.'
    sidecar: 'authentication-authorization#1.6: the stack reference names the euid read and decision and the directory check at each seam.'
    rationale: 'Report, Symbols: `viola_channel` re-exports and the three client seams.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 28
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC client-side server verification, the order at a seam ⏎ - **Implementation:** On every client seam the order is the directory check, `Client::connect`, the peer euid decision, then the server verification against the snapshot. ⏎ - **Stack reference:** `src/cmd/client.rs` (`connect`), `src/cmd/hook.rs` (`connect`), `crates/viola-mcp/src/tools.rs` (`frame`)'
    sidecar: 'authentication-authorization: added the order of the checks on every client seam.'
    rationale: 'Report, Symbols: `src/cmd/client.rs` `connect` and `src/cmd/hook.rs` `connect`, the same three checks in the same order, and `frame`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    fact: F5
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'In the row, «`viola_state::board::scan` runs `check_path` on the home and on `instances/`, each that exists» now reads: `viola_state::board::scan` runs `check_read` on the home and `check_path` on `instances/`, each that exists.'
    sidecar: 'authentication-authorization#1.8: `viola list` holds the home to the read rule.'
    rationale: 'Report, Retired: the two rules of `viola list` on the home (`inputs#I2` Q3); Symbols, `crates/viola-state/src/board.rs` `scan`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'In the row, the clause from «proven by `cfg(unix)` cases only» through «a CARRY owed to the route entry "Unix endpoint and home hardening"» now reads: proven on all three OSes; the board holds the home to the read rule on every OS, so a 0750 home is refused as `ScanRefused::Home` with or without an instance (the delegate answer `inputs#I2` Q3, PROVISIONAL).'
    sidecar: 'authentication-authorization#1.8: the CARRY on the two rules of the board is discharged; the read rule on every OS stands PROVISIONAL.'
    rationale: 'Report, Counts: `viola list` home, two rules to the read rule at authentication-authorization.md:36; Outcome: `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`, met on all three OSes.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 43
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** `~/.viola/` access control, the board ⏎ - **Implementation:** `scan` holds the home to `check_read` on every OS and `instances/` to `check_path`, with the order, the whole-board `ScanRefused::Home` and no-open unchanged; `read_budget_stamped`, the start of `mcp` and `plugin install` keep the write rule on the home. ⏎ - **Stack reference:** `crates/viola-state/src/board.rs` (`scan`, `check_if_present`)'
    sidecar: 'authentication-authorization: added the rule the board holds on the home and the readers that keep the write rule.'
    rationale: 'Report, Symbols, `crates/viola-state/src/board.rs` `scan`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 65
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC client-side server verification, the observed peer euid ⏎ - **Implementation:** `peer_euid_admitted(observed, own, reported)` is admit when not reported or observed equals own, with `PEER_EUID_REPORTED` being `cfg!(unix)`, so a peer whose euid cannot be read is refused on Unix; on Windows no euid is read and the DACL and the pid comparison stay the controls. Reading the euid (`Client::server_euid` / `TokioClient::server_euid`, `peer_creds().ok()?.euid()`) writes and logs nothing. ⏎ - **Stack reference:** `viola-channel` `client.rs` and `tokio_client.rs` (`peer_euid_of`, `peer_euid_admitted`, `peer_admitted`); interprocess 2.4.4'
    sidecar: 'authentication-authorization: added the peer euid read and its decision, Unix only.'
    rationale: 'Report, Symbols: `viola_channel::{peer_euid_admitted, peer_admitted, PEER_EUID_REPORTED}`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 73
    section: '## Authentication & Authorization'
    change: 'In the row, «then, on Linux and Windows, the server check through `cmd::client::connect` before their one `link` / `unlink` frame (macOS the recorded gap)» now reads: then `cmd::client::connect` before their one `link` / `unlink` frame: the socket-directory check on the recorded endpoint, the connect, the peer euid decision on Unix and, on Linux and Windows, the server check (macOS compares no pid or start time, the named carve-out).'
    sidecar: 'authentication-authorization#3.4: the checks of `link` and `unlink` through `connect` gain the directory check and the euid decision.'
    rationale: 'Report, Symbols: `connect` runs the checks for its 8 callers, `link` and `unlink` among them, none edited.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 76
    section: '## Authentication & Authorization'
    change: 'Extend the row: a refused peer euid is this same mismatch, with the same document and hint. A socket-directory refusal, before the connect, writes no frame and exits 21: under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` on stdout with stderr empty; in human mode the same `unable` line with the hint of cause `socket-dir` last on stderr, its wording a planning form.'
    sidecar: 'authentication-authorization#3.5: the CLI refusal gains `socket-dir-refused`; a refused euid reads `server-verify-failed`.'
    rationale: 'Report, Symbols: `CallError::SocketDirRefused` to `Unreachable::SocketDir`, and a refused euid is `CallError::ServerUnverified`, reading `server-verify-failed`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 77
    section: '## Authentication & Authorization'
    change: 'Extend the row: `src/cmd/client.rs` also `CallError::SocketDirRefused`, `Unreachable::SocketDir` and `CallError::ServerUnverified`; `src/human/hints.rs` also the `socket-dir` cause.'
    sidecar: 'authentication-authorization#3.5: the stack reference names the socket-directory refusal.'
    rationale: 'Report, Symbols: `src/cmd/client.rs` `connect`, `src/human/hints.rs` `send_hint` cause `socket-dir`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:19
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 79
    section: '## Authentication & Authorization'
    change: 'Row now reads: - **Implementation:** The `frame` of `viola mcp` maps a refused `check_instance` to `strict-modes-failed`, no `live_endpoint` to the reading of `without_endpoint`, a refused `check_endpoint_dir` after `live_endpoint` to `socket-dir-refused`, and a refused `peer_admitted` or a failed `server_verified` to `server-verify-failed`, each the `instance-unreachable` result with its cause in `detail`, with no frame and no line on any of them, `isError` the only record.'
    sidecar: 'authentication-authorization#3.6: the MCP refusal gains `socket-dir-refused` and the euid decision.'
    rationale: 'Report, Symbols: `crates/viola-mcp/src/tools.rs` `frame`, the directory check and the refused euid, no frame, no line, `isError` the record.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 85
    section: '## Authentication & Authorization'
    change: 'Row now reads: - **Implementation:** On `macos-latest` a per-user socket directory at 0755 or a symlink to a 0700 one gives exit 21 on a frame verb, with its own refusal code, and the client euid read holds there. The server-identity negatives compile out on macOS, the pid and start-time comparison absent there, the named carve-out.'
    sidecar: 'authentication-authorization#3.8: the macOS CARRY is discharged; the socket-directory and euid cases hold on macOS.'
    rationale: 'Report, Retired and Outcome: the macOS directory case is met on `macos-latest`; the real euid read holds on Linux and macOS.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 86
    section: '## Authentication & Authorization'
    change: 'Extend the row: `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`).'
    sidecar: 'authentication-authorization#3.8: the stack reference names the socket-directory negatives.'
    rationale: 'Report, Outcome basis: no case of `security_negatives_socket_dir` on Windows, `#![cfg(unix)]`; its cases run on `macos-latest`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:128
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 86
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC client-side server verification, the macOS carve-out ⏎ - **Implementation:** The absent pid and start-time comparison of macOS is a named carve-out owned by no route entry: `server_verdict_rows` keeps its row and the server-identity negatives stay compiled out there. ⏎ - **Stack reference:** `server_verdict_rows`; `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`)'
    sidecar: 'authentication-authorization: added the macOS pid and start-time carve-out, owned by no route entry.'
    rationale: 'Report, Outcome: macOS comparison absent and named; Counts: the macOS pid gap stays named, owned by no route entry.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:110
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 86
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC access control (Unix), the tests holding the directory check ⏎ - **Implementation:** `tests/security_negatives_socket_dir.rs`, `#![cfg(unix)]`, holds the start, frame-verb and hook negatives over a 0755 directory and a symlink to a 0700 one, on `ubuntu-latest` and `macos-latest`, none on Windows. ⏎ - **Stack reference:** `tests/security_negatives_socket_dir.rs`'
    sidecar: 'authentication-authorization: added the socket-directory negatives and the OSes they run on.'
    rationale: 'Report, Outcome: acceptance on `ubuntu-latest` and `macos-latest`; Outcome basis: none on Windows.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 86
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC peer and directory checks, what no test drives ⏎ - **Implementation:** A foreign euid and a foreign owner are not driven by any test: one account can make no foreign peer, so those decisions are held by injected values, and no seam may be added to drive them. ⏎ - **Stack reference:** `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, `serve_drops_a_refused_peer_before_any_frame`'
    sidecar: 'authentication-authorization: added that a foreign euid and a foreign owner are held by injected values, never driven.'
    rationale: 'Report, What the tests do not prove; Insufficient fixes: one account can make no foreign peer, and no seam may be added.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:130
    fact: F14
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 88
    section: '## Authentication & Authorization'
    change: 'Extend the row: `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` reads the eight frame verbs, `link` and `unlink` among them, refused with no frame over a 0755 directory and a symlink to a 0700 one, on Linux and macOS (`tests/security_negatives_socket_dir.rs`, joining the stack reference).'
    sidecar: 'authentication-authorization#3.9: the tests holding the controls of `link` and `unlink` gain the socket-directory negative.'
    rationale: 'Report, Coverage of new surfaces: the same check at the eight CLI frame verbs; New text, by line: that case over `VERBS`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 89
    section: '## Authentication & Authorization'
    change: 'New row: - **Requirement:** IPC client-side server verification, where the checks sit ⏎ - **Implementation:** No check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge, the checks sit at the seams, and `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design. ⏎ - **Stack reference:** `viola_channel::Client`; `tests/channel_links.rs`'
    sidecar: 'authentication-authorization: added why no check sits inside `Client::connect`.'
    rationale: 'Report, Reverted / negative API facts: no check inside `viola_channel::Client::connect`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:53
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 100
    section: '## Authentication & Authorization'
    change: 'In the IPC library line, after «none on macOS)» add inside the same list: `Stream::peer_creds()` `euid()` is the Unix read of the peer on both sides, the wrapper reading a client and a client reading the server, none on Windows.'
    sidecar: 'security-plan §Authentication & Authorization: the library line names the euid read of `peer_creds()`.'
    rationale: 'Report, Symbols: `peer_creds().ok()?.euid()` on Unix, none on Windows; the same read serves the wrapper reading a client (Deviations 1). The library stays on spec, no D-security-auth drift.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan.md
    near: 114
    section: '## Authentication & Authorization'
    change: 'Clause «the client euid check and macOS (no peer pid; its euid and verified socket-directory checks) are owed to "Unix endpoint and home hardening"» now reads: on Linux and macOS the socket-directory check before the connect and the peer euid decision after it hold too; macOS compares no peer pid or start time, the named carve-out, owned by no route entry.'
    sidecar: 'security-plan §Authentication & Authorization: the client euid and macOS clause owed to the route entry is discharged; the macOS pid gap stays named.'
    rationale: 'Report, Retired: the client euid check and the socket-directory check are built; Counts: the owner clause changes, macOS pid gap named but owned by no route entry.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
note: 'Each ⏎ in a change is a line break of the new row. security-plan.md:70 (Networking, «`$TMPDIR` or `/tmp`») reads the disproved socket-root claim, but its group is not in this listing, so no proposal is written for it. D-security-input, D-security-auth and D-security-deps found no drift.'
