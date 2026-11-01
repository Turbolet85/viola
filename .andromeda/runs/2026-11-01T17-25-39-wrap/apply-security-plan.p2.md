# Apply — security-plan · part 2 of 4

## Verdicts
- fanout-security-plan--cascade-g1.md 6 · applied · .andromeda/security-plan-detail/authentication-authorization.md:18#1
- fanout-security-plan--cascade-g1.md 7 · applied · .andromeda/security-plan-detail/authentication-authorization.md:18#2
- fanout-security-plan--cascade-g1.md 8 · applied · .andromeda/security-plan-detail/authentication-authorization.md:19#1
- fanout-security-plan--cascade-g1.md 9 · applied · .andromeda/security-plan-detail/authentication-authorization.md:19+#1
- fanout-security-plan--cascade-g1.md 10 · applied · .andromeda/security-plan-detail/authentication-authorization.md:19+#2
- fanout-security-plan--cascade-g1.md 11 · applied · .andromeda/security-plan-detail/authentication-authorization.md:19+#3
- fanout-security-plan--cascade-g1.md 12 · applied · .andromeda/security-plan-detail/authentication-authorization.md:19+#4
- fanout-security-plan--cascade-g1.md 13 · applied · .andromeda/security-plan-detail/authentication-authorization.md:19+#5
- fanout-security-plan--cascade-g1.md 15 · applied · .andromeda/security-plan-detail/authentication-authorization.md:22#1
- fanout-security-plan--cascade-g1.md 16 · applied · .andromeda/security-plan-detail/authentication-authorization.md:22+#1
- fanout-security-plan--cascade-g1.md 17 · applied · .andromeda/security-plan-detail/authentication-authorization.md:24#1
- fanout-security-plan--cascade-g1.md 18 · applied · .andromeda/security-plan-detail/authentication-authorization.md:28#1, .andromeda/security-plan-detail/authentication-authorization.md:28#2
- fanout-security-plan--cascade-g1.md 19 · applied · .andromeda/security-plan-detail/authentication-authorization.md:28+#1
- fanout-security-plan--cascade-g1.md 20 · applied · .andromeda/security-plan-detail/authentication-authorization.md:35#1
- fanout-security-plan--cascade-g1.md 21 · applied · .andromeda/security-plan-detail/authentication-authorization.md:36#1
- fanout-security-plan--cascade-g1.md 22 · applied · .andromeda/security-plan-detail/authentication-authorization.md:43+#1
- fanout-security-plan--cascade-g1.md 23 · applied · .andromeda/security-plan-detail/authentication-authorization.md:65+#1
- fanout-security-plan--cascade-g1.md 24 · applied · .andromeda/security-plan-detail/authentication-authorization.md:73#1
- fanout-security-plan--cascade-g1.md 25 · applied · .andromeda/security-plan-detail/authentication-authorization.md:76#1
- fanout-security-plan--cascade-g1.md 26 · applied · .andromeda/security-plan-detail/authentication-authorization.md:77#1, .andromeda/security-plan-detail/authentication-authorization.md:77#2
- fanout-security-plan--cascade-g1.md 27 · applied · .andromeda/security-plan-detail/authentication-authorization.md:79#1
- fanout-security-plan--cascade-g1.md 28 · applied · .andromeda/security-plan-detail/authentication-authorization.md:85#1
- fanout-security-plan--cascade-g1.md 29 · applied · .andromeda/security-plan-detail/authentication-authorization.md:86#1
- fanout-security-plan--cascade-g1.md 30 · applied · .andromeda/security-plan-detail/authentication-authorization.md:86+#1
- fanout-security-plan--cascade-g1.md 31 · applied · .andromeda/security-plan-detail/authentication-authorization.md:86+#2
- fanout-security-plan--cascade-g1.md 32 · applied · .andromeda/security-plan-detail/authentication-authorization.md:86+#3
- fanout-security-plan--cascade-g1.md 33 · applied · .andromeda/security-plan-detail/authentication-authorization.md:88#1, .andromeda/security-plan-detail/authentication-authorization.md:89#1
- fanout-security-plan--cascade-g1.md 34 · applied · .andromeda/security-plan-detail/authentication-authorization.md:89+#1

## Edits
- line: .andromeda/security-plan-detail/authentication-authorization.md:18
  old: «CLI/MCP clients report `instance-unreachable` (exit 21), and `hook` fails open (exit 0, no body, a note in `diagnostics/`) without writing any frame»
  new: «CLI/MCP clients report `instance-unreachable` with `detail` `socket-dir-refused` (the CLI exit 21), and `hook` fails open (exit 0, empty streams, one `hook-decision` of `detail` `socket-dir-refused` at WARN) without writing any frame»
- line: .andromeda/security-plan-detail/authentication-authorization.md:18
  old: «a failure at `bind_endpoint`'s `create_private_dir` has no refusal code: it is an anyhow error at the catch site, and its code is owed to the route entry "Unix endpoint and home hardening".»
  new: «a failed create and a refused check read one code: `run` and the start of `revive` exit 1 with `socket-dir-refused` and one `process-exit` of that `detail`; a `Server::bind` failure is outside this code.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:19
  old: «`#[cfg(unix)]`; interprocess 2.4.4»
  new: «`#[cfg(unix)]`; interprocess 2.4.4; `viola-state` `strict` (`check_socket_dir`, `socket_dir_verdict`, and `check_endpoint_dir`, called by `src/cmd/run.rs`, `src/cmd/client.rs`, `src/cmd/hook.rs` and `crates/viola-mcp/src/tools.rs`)»
- after: .andromeda/security-plan-detail/authentication-authorization.md:19
  anchor: «  - **Stack reference:** `viola-channel` server, `#[cfg(unix)]`; interprocess 2.4.4»
  add: «- **Requirement:** IPC access control (Unix), the directory verdict»
  add: «  - **Implementation:** `viola_state::strict::check_socket_dir` makes one `symlink_metadata` read of the path: an absent path passes and an unstattable one is `Refused::Unreadable`; then the plain `socket_dir_verdict` reads a symlink as `Refused::Symlink`, a non-directory as `Refused::NotADirectory` ("a trusted state path is not a directory"), and anything else through `unix_read_verdict`. Off Unix `check_socket_dir` returns `Ok(())`.»
  add: «  - **Stack reference:** `crates/viola-state/src/strict.rs` (`check_socket_dir`, `socket_dir_verdict`)»
- after: .andromeda/security-plan-detail/authentication-authorization.md:19
  anchor: «  - **Stack reference:** `viola-channel` server, `#[cfg(unix)]`; interprocess 2.4.4»
  add: «- **Requirement:** IPC access control (Unix), a refused directory»
  add: «  - **Implementation:** The socket-directory check never chmods: a refused directory stays as found.»
  add: «  - **Stack reference:** `crates/viola-state/src/strict.rs` (`check_socket_dir`); `tests/security_negatives_socket_dir.rs`»
- after: .andromeda/security-plan-detail/authentication-authorization.md:19
  anchor: «  - **Stack reference:** `viola-channel` server, `#[cfg(unix)]`; interprocess 2.4.4»
  add: «- **Requirement:** IPC access control (Unix), the recorded endpoint»
  add: «  - **Implementation:** `check_endpoint_dir` is the check on a recorded endpoint's parent, the one helper the bind and the three client seams share.»
  add: «  - **Stack reference:** `crates/viola-state/src/strict.rs` (`check_endpoint_dir`); `src/cmd/run.rs`, `src/cmd/client.rs`, `src/cmd/hook.rs`, `crates/viola-mcp/src/tools.rs`»
- after: .andromeda/security-plan-detail/authentication-authorization.md:19
  anchor: «  - **Stack reference:** `viola-channel` server, `#[cfg(unix)]`; interprocess 2.4.4»
  add: «- **Requirement:** IPC access control (Unix), at a start»
  add: «  - **Implementation:** `socket_dir_ready` runs `check_endpoint_dir`, creates the parent only when absent, then checks again, so a race's loser reads the same. It runs before any snapshot, child or `Server::bind`, and the start of `revive` shares it; its refusal is the **Socket-directory failure at bind:** clause of IPC access control (Unix).»
  add: «  - **Stack reference:** `src/cmd/run.rs` (`socket_dir_ready`, `refuse_socket_dir`, `enum Bound`)»
- after: .andromeda/security-plan-detail/authentication-authorization.md:19
  anchor: «  - **Stack reference:** `viola-channel` server, `#[cfg(unix)]`; interprocess 2.4.4»
  add: «- **Requirement:** IPC access control (Unix), a leftover socket file»
  add: «  - **Implementation:** A killed wrapper's socket file stays in the per-user directory until a next bind of that name takes it over: nothing removes it, by choice, a harmless file in a 0700 directory.»
  add: «  - **Stack reference:** `crates/viola-e2e/src/harness/cleanup.rs` (`unconnectable`, which removes no leftover file)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:22
  old: «`viola-channel` sync + Tokio server»
  new: «`viola-channel` sync server (`crates/viola-channel/src/server.rs`, `accept_loop`), the one server; `TokioClient` is a client»
- after: .andromeda/security-plan-detail/authentication-authorization.md:22
  anchor: «  - **Stack reference:** `viola-channel` sync + Tokio server»
  add: «- **Requirement:** IPC server-side peer check (Unix), the drop»
  add: «  - **Implementation:** The drop filters `incoming()` in `accept_loop` by the peer's euid decision on the accept thread, before the stream's worker thread and before any frame: the peer reads end-of-file, and no line and nothing on the terminal is written.»
  add: «  - **Stack reference:** `crates/viola-channel/src/server.rs` (`accept_loop`, `serve_admitting`), the decision `peer_admitted`»
- line: .andromeda/security-plan-detail/authentication-authorization.md:24
  old: «(the client-side euid check is not built, owed to the route entry "Unix endpoint and home hardening")»
  new: «(checked at `cmd::client::connect`, the `connect` of `cmd::hook` and the `frame` of `viola-mcp`, through `peer_admitted`; a peer whose euid cannot be read is refused, and a refused euid reads `server-verify-failed`)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:28
  old: «`peer_creds().pid()`); `viola-state` liveness (`live_endpoint`»
  new: «`peer_creds().pid()`; the peer euid and its decision, `Client::server_euid` / `TokioClient::server_euid` over `peer_creds().euid()`, `peer_admitted`); `viola-state` liveness (`live_endpoint`»
- line: .andromeda/security-plan-detail/authentication-authorization.md:28
  old: «; windows-sys 0.61.2; sysinfo 0.39.6»
  new: «; `viola-state` `strict::check_endpoint_dir` (called by `cmd::client::connect`, the `connect` of `cmd::hook` and the `frame` of `viola-mcp`); windows-sys 0.61.2; sysinfo 0.39.6»
- after: .andromeda/security-plan-detail/authentication-authorization.md:28
  anchor: «  - **Stack reference:** `viola-channel` client (sync + `tokio` feature: the SQOS open and the peer pid, `Client::server_pid` / `TokioClient::server_pid` over interprocess 2.4.4 `peer_creds().pid()`); `viola-state` liveness (`live_endpoint`, `server_verdict`, `verify_server`), called by the root bin's `cmd::client::connect` and `cmd::hook` and by `viola-mcp`'s `frame`; `viola-state` `strict::check_instance` (called by the root bin's `cmd::client::refuse_untrusted` and `cmd::hook`'s `trusted`, and by `viola-mcp`'s `frame`) and `replay::read_snapshot_or_replay` (read by `live_endpoint` and `cmd::hook`'s `connect`); windows-sys 0.61.2; sysinfo 0.39.6»
  add: «- **Requirement:** IPC client-side server verification, the order at a seam»
  add: «  - **Implementation:** On every client seam the order is the directory check, the connect, the peer euid decision, then the server verification against the snapshot.»
  add: «  - **Stack reference:** `src/cmd/client.rs` (`connect`), `src/cmd/hook.rs` (`connect`), `crates/viola-mcp/src/tools.rs` (`frame`)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:35
  old: «`viola_state::board::scan` runs `check_path` on the home and on `instances/`, each that exists»
  new: «`viola_state::board::scan` runs `check_read` on the home and `check_path` on `instances/`, each that exists»
- line: .andromeda/security-plan-detail/authentication-authorization.md:36
  old: «proven by `cfg(unix)` cases only; `check_path` holds the home to the write rule while `check_instance` holds it to the read rule, so a 0750 home passes a board with no instance and is refused, naming the instance (`ScanRefused::Instance`), once it holds one, an inconsistency that is a CARRY owed to the route entry "Unix endpoint and home hardening"»
  new: «proven on all three OSes; the board holds the home to the read rule on every OS, so a home another user may read (a 0750 home on Unix) is refused as `ScanRefused::Home` with or without an instance»
- after: .andromeda/security-plan-detail/authentication-authorization.md:43
  anchor: «  - **Stack reference:** `viola-state` home resolution; tempfile 3.27.0 `persist` via `viola_state::fs::replace_private`; `into_temp_path` + `persist_noclobber` via `viola_state::fs::place_private_new` (the pinned `viola`); std `OpenOptionsExt`; windows-sys 0.61.2»
  add: «- **Requirement:** `~/.viola/` access control, the board»
  add: «  - **Implementation:** `scan` holds the home to `check_read` and `instances/` to `check_path` on every OS, the first failure refusing the whole board (`ScanRefused::Home` on the home) before anything under the home is opened; `read_budget_stamped`, the start of `mcp` and `plugin install` hold the home to the write rule.»
  add: «  - **Stack reference:** `crates/viola-state/src/board.rs` (`scan`, `check_if_present`)»
  add: «  - OPEN — the board's read rule on every OS stands on a delegate's answer · owner: the Epoch 7 boundary batch»
- after: .andromeda/security-plan-detail/authentication-authorization.md:65
  anchor: «  - **Stack reference:** `viola-channel` `client.rs` and `tokio_client.rs` (behind the `tokio` feature); interprocess 2.4.4»
  add: «- **Requirement:** IPC client-side server verification, the observed peer euid»
  add: «  - **Implementation:** `peer_euid_admitted(observed, own, reported)` admits when not reported or when observed equals own (`!reported || observed == Some(own)`); `PEER_EUID_REPORTED` is `cfg!(unix)`, so a peer whose euid cannot be read is refused on Unix, and on Windows no euid is read and the DACL and the pid comparison stay the controls. `Client::server_euid` / `TokioClient::server_euid` read it (`peer_creds().ok()?.euid()`), writing and logging nothing.»
  add: «  - **Stack reference:** `viola-channel` `client.rs` (`peer_euid_of`, `peer_euid_admitted`, `peer_admitted`, `Client::server_euid`) and `tokio_client.rs` (`TokioClient::server_euid`); interprocess 2.4.4»
- line: .andromeda/security-plan-detail/authentication-authorization.md:73
  old: «then, on Linux and Windows, the server check through `cmd::client::connect` before their one `link` / `unlink` frame (macOS the recorded gap)»
  new: «then `cmd::client::connect` before their one `link` / `unlink` frame: the socket-directory check on the recorded endpoint, the connect, the peer euid decision on Unix and, on Linux and Windows, the server check (macOS compares no pid or start time, the named carve-out)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:76
  old: «last on stderr. The client's own exit-13 refusals come first»
  new: «last on stderr. A refused peer euid is this same mismatch, with the same document and hint. A socket-directory refusal, before the connect, writes no frame and exits 21: under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` on stdout with stderr empty; in human mode the same `unable` line with the hint of cause `socket-dir` last on stderr. The client's own exit-13 refusals come first»
- line: .andromeda/security-plan-detail/authentication-authorization.md:77
  old: «`Unreachable::ServerUnverified`, `refuse_untrusted`)»
  new: «`Unreachable::ServerUnverified`, `CallError::ServerUnverified`, `CallError::SocketDirRefused`, `Unreachable::SocketDir`, `refuse_untrusted`)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:77
  old: «(the `server-verify` cause)»
  new: «(the `server-verify` and `socket-dir` causes)»
- line: .andromeda/security-plan-detail/authentication-authorization.md:79
  old: «`without_endpoint`'s reading and a failed `server_verified` to `server-verify-failed`, each the»
  new: «`without_endpoint`'s reading, a refused `check_endpoint_dir` after `live_endpoint` to `socket-dir-refused`, and a refused `peer_admitted` or a failed `server_verified` to `server-verify-failed`, each the»
- line: .andromeda/security-plan-detail/authentication-authorization.md:85
  old: «The macOS server-verification cases, the per-user socket directory loosened to 0755 giving exit 21, the client euid check and the socket-directory refusal's code, are a CARRY owed to the route entry "Unix endpoint and home hardening". The server-identity negatives compile out on macOS.»
  new: «On `macos-latest` a per-user socket directory at 0755 or a symlink to a 0700 one gives exit 21 on a frame verb, with `detail` `socket-dir-refused`, and the client euid read holds there. The server-identity negatives compile out on macOS, the named carve-out (below).»
- line: .andromeda/security-plan-detail/authentication-authorization.md:86
  old: «`tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`)»
  new: «`tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`); `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`)»
- after: .andromeda/security-plan-detail/authentication-authorization.md:86
  anchor: «  - **Stack reference:** `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`)»
  add: «- **Requirement:** IPC client-side server verification, the macOS carve-out»
  add: «  - **Implementation:** The absent pid and start-time comparison of macOS is a named carve-out owned by no route entry: `server_verdict_rows` keeps its row and the server-identity negatives stay compiled out there.»
  add: «  - **Stack reference:** `server_verdict_rows`; `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`)»
- after: .andromeda/security-plan-detail/authentication-authorization.md:86
  anchor: «  - **Stack reference:** `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`)»
  add: «- **Requirement:** IPC access control (Unix), the tests holding the directory check»
  add: «  - **Implementation:** `tests/security_negatives_socket_dir.rs`, `#![cfg(unix)]`, holds the start, frame-verb and hook negatives over a 0755 directory and a symlink to a 0700 one, on `ubuntu-latest` and `macos-latest`, none on Windows.»
  add: «  - **Stack reference:** `tests/security_negatives_socket_dir.rs`»
- after: .andromeda/security-plan-detail/authentication-authorization.md:86
  anchor: «  - **Stack reference:** `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`)»
  add: «- **Requirement:** IPC peer and directory checks, what no test drives»
  add: «  - **Implementation:** A foreign euid and a foreign owner are not driven by any test: one account can make no foreign peer, so those decisions are held by injected values, and no seam may be added to drive them.»
  add: «  - **Stack reference:** `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, `serve_drops_a_refused_peer_before_any_frame`»
- line: .andromeda/security-plan-detail/authentication-authorization.md:88
  old: «compiled out on macOS with the file.»
  new: «compiled out on macOS with the file; `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` reads the eight frame verbs, `link` and `unlink` among them, refused with no frame over a 0755 directory and a symlink to a 0700 one, on Linux and macOS.»
- line: .andromeda/security-plan-detail/authentication-authorization.md:89
  old: «`tests/channel_links.rs`, `tests/cli_links.rs`, `tests/security_negatives_server_identity.rs`»
  new: «`tests/channel_links.rs`, `tests/cli_links.rs`, `tests/security_negatives_server_identity.rs`, `tests/security_negatives_socket_dir.rs`»
- after: .andromeda/security-plan-detail/authentication-authorization.md:89
  anchor: «  - **Stack reference:** `tests/channel_links.rs`, `tests/cli_links.rs`, `tests/security_negatives_server_identity.rs`»
  add: «- **Requirement:** IPC client-side server verification, where the checks sit»
  add: «  - **Implementation:** No check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge, the checks sit at the seams, and `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design.»
  add: «  - **Stack reference:** `viola_channel::Client`; `tests/channel_links.rs`»

## Patterns
[[pattern]]
id = "sec2-01"
fixed = 'a note in `diagnostics/`'

[[pattern]]
id = "sec2-02"
fixed = 'has no refusal code'

[[pattern]]
id = "sec2-03"
fixed = 'sync + Tokio server'

[[pattern]]
id = "sec2-04"
fixed = 'client-side euid check is not built'

[[pattern]]
id = "sec2-05"
fixed = 'runs `check_path` on the home and on `instances/`'

[[pattern]]
id = "sec2-06"
fixed = 'a 0750 home passes a board with no instance'

[[pattern]]
id = "sec2-07"
fixed = 'on Linux and Windows, the server check through `cmd::client::connect`'

[[pattern]]
id = "sec2-08"
fixed = 'the client euid check and the socket-directory'
