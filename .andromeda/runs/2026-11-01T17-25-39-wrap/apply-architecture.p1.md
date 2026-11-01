# Apply — architecture · part 1 of 4

## Verdicts
- fanout-architecture--cascade-g2.md 18 · applied · .andromeda/architecture.md:100#1, .andromeda/architecture.md:100#2
- fanout-architecture--cascade-g2.md 19 · applied · .andromeda/architecture.md:226#1
- fanout-architecture.md 1 · folded into fanout-architecture--cascade-g2.md 19
- fanout-architecture--cascade-g2.md 20 · applied · .andromeda/architecture.md:229#1
- fanout-architecture.md 4 · folded into fanout-architecture--cascade-g2.md 20
- fanout-architecture--cascade-g3.md 3 · applied · .andromeda/architecture.md:265#1
- fanout-architecture.md 5 · folded into fanout-architecture--cascade-g3.md 3
- fanout-architecture--cascade-g1.md 1 · applied · .andromeda/architecture-detail/stack-and-technologies.md:39#1
- cascade-compare-architecture.md 2 · applied · .andromeda/architecture-detail/stack-and-technologies.md:15#1
- fanout-architecture--cascade-g1.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:77+#1
- fanout-architecture--cascade-g1.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:81+#1
- fanout-architecture--cascade-g1.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:83+#1
- fanout-architecture--cascade-g1.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:87#1
- fanout-architecture--cascade-g1.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:89+#1
- fanout-architecture--cascade-g1.md 8 · applied · .andromeda/architecture-detail/established-decisions.md:125#1
- fanout-architecture--cascade-g1.md 9 · applied · .andromeda/architecture-detail/established-decisions.md:156#1
- fanout-architecture--cascade-g1.md 10 · applied · .andromeda/architecture-detail/established-decisions.md:171+#1
- fanout-architecture--cascade-g1.md 11 · applied · .andromeda/architecture-detail/established-decisions.md:198#1
- fanout-architecture--cascade-g1.md 12 · applied · .andromeda/architecture-detail/established-decisions.md:258#1
- fanout-architecture--cascade-g1.md 13 · applied · .andromeda/architecture-detail/established-decisions.md:259#1
- fanout-architecture--cascade-g1.md 14 · applied · .andromeda/architecture-detail/established-decisions.md:262#1
- fanout-architecture--cascade-g1.md 15 · applied · .andromeda/architecture-detail/established-decisions.md:266+#1
- fanout-architecture--cascade-g1.md 16 · applied · .andromeda/architecture-detail/established-decisions.md:309#1

## Edits
- line: .andromeda/architecture.md:100
  old: «(a refused home-tree check of `run`, `viola revive`'s start or `verify` among them)»
  new: «(a refused home-tree check of `run`, `viola revive`'s start or `verify`, and a refused socket directory of `run` or `viola revive`'s start, among them)»
- line: .andromeda/architecture.md:100
  old: «or a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`).»
  new: «or a CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`), and a refused socket directory of a CLI frame verb, `viola link` or `viola unlink`.»
- line: .andromeda/architecture.md:226
  old: «`server-verify-failed` (`viola_core::UnreachableCause`)»
  new: «`server-verify-failed` · `socket-dir-refused` (`viola_core::UnreachableCause`)»
- line: .andromeda/architecture.md:229
  old: «`squatted-name` · `internal-error`»
  new: «`squatted-name` · `socket-dir-refused` · `internal-error`»
- line: .andromeda/architecture.md:265
  old: «never the `/tmp` or `$TMPDIR` root, chmod 0600 after the bind; every other process connects to the `endpoint` recorded in the instance snapshot.»
  new: «never the `/tmp` or `$TMPDIR` root, checked before the bind and before every client's connect and, when refused (`socket-dir-refused`), left as found, never narrowed; the socket is chmod 0600 after the bind; every other process connects to the `endpoint` recorded in the instance snapshot, and both sides admit only a peer of the owning user's euid, the wrapper dropping another before any frame.»
- line: .andromeda/architecture-detail/stack-and-technologies.md:39
  old: «the identity each frame-writing client checks before its first frame (§Standard Contracts, Session liveness).»
  new: «the identity each frame-writing client checks before its first frame (§Standard Contracts, Session liveness). On Unix both clients also read the serving process's euid from the same creds, `peer_creds().ok()?.euid()` (`Client::server_euid`, `TokioClient::server_euid`), none on Windows, where the DACL and the pid comparison stay the controls.»
- line: .andromeda/architecture-detail/stack-and-technologies.md:15
  old: «and the socket is chmod 0600 after the bind»
  new: «and the socket is chmod 0600 after the bind; the directory itself is read by `viola_state::strict::check_endpoint_dir` (the recorded endpoint's parent, through `check_socket_dir`, one std `symlink_metadata`) before a bind (`viola run`, `revive`'s start) and before each client's connect (the CLI frame verbs, `hook`, MCP's `frame`), a symlink, a non-directory, another owner or a group or other bit refused and never chmodded, an absent path passing»
- after: .andromeda/architecture-detail/established-decisions.md:77
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-message-broker-ipc -->»
  add: «»
  add: «- No check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge, and `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design; the socket-directory check stands at the channel seam's callers.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/established-decisions.md:81
  anchor: «- `viola_channel::Client::server_pid` and `tokio_client::TokioClient::server_pid` return the connected stream's peer pid through interprocess's `peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the client end the SQOS Identification open adopts, none on macOS, where `SERVER_PID_REPORTED` is false; they write and log nothing. Every frame-writing client compares that pid and the process's OS start time with the snapshot's before its first frame (§Standard Contracts, Channel methods), so on Linux and Windows a squatted or rogue server is never sent a frame either.»
  add: «- `viola_channel` exports `peer_euid_admitted`, `peer_admitted` and `PEER_EUID_REPORTED`, the last `cfg!(unix)`: `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer whose euid cannot be read is refused where it is reported, and on Windows the DACL and the pid comparison stay the controls. `Client::server_euid` and `TokioClient::server_euid` supply the observed euid, writing and logging nothing.»
- after: .andromeda/architecture-detail/established-decisions.md:83
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-message-broker-ipc -->»
  add: «»
  add: «- The wrapper's `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))`, so a refused stream is dropped on the accept thread, before its worker thread and any frame, reading end-of-file, writing no line, nothing on the terminal and moving no wheel; `Server::serve` injects the decision, `peer_admitted`, as a `type Admission` through the private `serve_admitting`, with no env, flag or feature seam.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/established-decisions.md:87
  old: «the recorded gap. The macOS client euid and socket-directory checks, and the 0755-directory exit-cause case, are owed to the working-route entry "Unix endpoint and home hardening".»
  new: «the recorded gap, named and owned by no route entry. The macOS client euid and socket-directory checks, and the 0755-directory exit-cause case, stand built, so on macOS `server-verify-failed` means an euid mismatch alone.»
- after: .andromeda/architecture-detail/established-decisions.md:89
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-message-broker-ipc -->»
  add: «»
  add: «- `viola_state::strict::check_socket_dir` takes one `symlink_metadata`: an absent path passes, an unstattable one is `Refused::Unreadable`, then `socket_dir_verdict` reads a symlink as `Refused::Symlink`, a non-directory as the fixed-message `Refused::NotADirectory` and anything else by the read rule; it never chmods, a refused directory stays as found, and off Unix it returns `Ok(())`. `check_endpoint_dir` is that check on a recorded endpoint's parent.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/established-decisions.md:125
  old: «no `live_endpoint` by `without_endpoint` ([Session Liveness]) and a failed `server_verified` as `ServerVerify`,»
  new: «no `live_endpoint` by `without_endpoint` ([Session Liveness]), a refused `check_endpoint_dir` after `live_endpoint` as `SocketDir`, and a refused `peer_admitted(client.server_euid())` or a failed `server_verified` as `ServerVerify`,»
- line: .andromeda/architecture-detail/established-decisions.md:156
  old: «then `Client::connect_by`, then `verify_server` (§Standard Contracts, Session liveness). A serving process whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame:»
  new: «then `check_endpoint_dir` on the recorded endpoint, then `Client::connect_by`, then `peer_admitted(client.server_euid())`, then `verify_server` (§Standard Contracts, Session liveness). A refused directory gets no frame: the hook records `socket-dir-refused` and exits 0 with no body. A serving process of another euid, or whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame:»
- after: .andromeda/architecture-detail/established-decisions.md:171
  anchor: «- A dialog hook whose server check fails (the serving process is not the recorded wrapper, or the check cannot be made) sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human (its log lines: §Occupied Resources, `diagnostics/`).»
  add: «- `viola hook` meeting a refused socket directory (`NotSent::SocketDir`, `Asked::SocketDir`) exits 0 with empty streams and no frame and writes one `hook-decision` at WARN with `detail` `socket-dir-refused`, on the dialog path after `hook-invoked{corr:null}` with `decision_emitted:false`; a refused euid is `NotSent::ServerUnverified` in the same fail-open form, reading `server-verify-failed`.»
- line: .andromeda/architecture-detail/established-decisions.md:198
  old: «Then come the liveness-only pre-check (`live_endpoint`), the server identity check through `cmd::client::connect`, and one request.»
  new: «Then come the liveness-only pre-check (`live_endpoint`), then through `cmd::client::connect` the socket-directory check (`check_endpoint_dir`, a refusal exit 21, `socket-dir-refused`), the connect, the peer euid check and the server identity check (either refusal exit 21, `server-verify-failed`), and one request.»
- line: .andromeda/architecture-detail/established-decisions.md:258
  old: «exclusive bind of the endpoint (a bind that finds the name taken»
  new: «exclusive bind of the endpoint (`socket_dir_ready` first, a refused or uncreatable directory the exit-1 refusal `socket-dir-refused` before any snapshot, child or `Server::bind`; a bind that finds the name taken»
- line: .andromeda/architecture-detail/established-decisions.md:259
  old: «→ endpoint bind → the budget gate at start»
  new: «→ endpoint bind (`socket_dir_ready` first, a refusal `socket-dir-refused`, exit 1, with no bind) → the budget gate at start»
- line: .andromeda/architecture-detail/established-decisions.md:262
  old: «then the liveness-only pre-check, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness).»
  new: «then the liveness-only pre-check, then `check_endpoint_dir` on the recorded endpoint, then connects and reads `peer_admitted(client.server_euid())`, then, on Linux and Windows, compares the connected peer's pid and its OS start time with them before its first frame (`verify_server`; §Standard Contracts, Session liveness); the seams are `src/cmd/client.rs` `connect` with its 8 callers, `src/cmd/hook.rs` `connect` and `crates/viola-mcp/src/tools.rs` `frame`.»
- after: .andromeda/architecture-detail/established-decisions.md:266
  anchor: «- Before its wrapper log opens, every start (`run`'s and a revived one) checks the home tree: `checked_exe` reads the exe's bytes once and takes their pin key, then `viola_state::strict::check_home_tree` checks the home, the instance's set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists; a refusal is exit 1 with the home pair, or the instance pair for an instance-level refusal, or under `viola revive --json` the `strict-modes-failed` document in place of either pair, no log opened, no line and no child.»
  add: «- Before any snapshot, child or `Server::bind`, `viola run` and `viola revive`'s start run `socket_dir_ready`: `check_endpoint_dir`, create the parent only when absent, check again, so a race's loser reads the same; a failed create and a refused check read one code, `socket-dir-refused`, exit 1, and a `Server::bind` failure does not read it.»
- line: .andromeda/architecture-detail/established-decisions.md:309
  old: «`squatted-name` · `internal-error`,»
  new: «`squatted-name` · `socket-dir-refused` · `internal-error`,»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = '`server-verify-failed` (`viola_core::UnreachableCause`)'

[[pattern]]
id = "arch1-02"
fixed = '`squatted-name` · `internal-error`'

[[pattern]]
id = "arch1-03"
fixed = 'are owed to the working-route entry "Unix endpoint and home hardening"'
