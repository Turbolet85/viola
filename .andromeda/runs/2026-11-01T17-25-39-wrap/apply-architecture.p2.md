# Apply — architecture · part 2 of 4

## Verdicts
- fanout-architecture--cascade-g2.md 1 · applied · .andromeda/architecture-detail/conventions.md:33#1
- fanout-architecture--cascade-g2.md 2 · applied · .andromeda/architecture-detail/conventions.md:42#1, .andromeda/architecture-detail/conventions.md:42#2, .andromeda/architecture-detail/conventions.md:42+#1
- fanout-architecture--cascade-g2.md 3 · applied · .andromeda/architecture-detail/conventions.md:64#1
- fanout-architecture--cascade-g2.md 4 · applied · .andromeda/architecture-detail/conventions.md:88#1, .andromeda/architecture-detail/conventions.md:88#2
- fanout-architecture--cascade-g2.md 5 · applied · .andromeda/architecture-detail/conventions.md:94#1, .andromeda/architecture-detail/conventions.md:94#2
- fanout-architecture--cascade-g2.md 6 · applied · .andromeda/architecture-detail/conventions.md:100#1
- fanout-architecture--cascade-g2.md 7 · applied · .andromeda/architecture-detail/conventions.md:114+#1
- fanout-architecture--cascade-g2.md 8 · applied · .andromeda/architecture-detail/conventions.md:118#1
- fanout-architecture--cascade-g2.md 9 · applied · .andromeda/architecture-detail/conventions.md:125#1
- fanout-architecture--cascade-g2.md 10 · applied · .andromeda/architecture-detail/conventions.md:127+#1
- fanout-architecture--cascade-g2.md 11 · applied · .andromeda/architecture-detail/standard-contracts.md:57#1
- fanout-architecture--cascade-g2.md 12 · applied · .andromeda/architecture-detail/standard-contracts.md:63#1
- fanout-architecture--cascade-g2.md 13 · applied · .andromeda/architecture-detail/standard-contracts.md:93#1
- fanout-architecture--cascade-g2.md 14 · applied · .andromeda/architecture-detail/standard-contracts.md:111#1
- fanout-architecture--cascade-g2.md 15 · applied · .andromeda/architecture-detail/standard-contracts.md:112#1
- fanout-architecture--cascade-g2.md 16 · applied · .andromeda/architecture-detail/standard-contracts.md:118#1
- fanout-architecture--cascade-g2.md 17 · applied · .andromeda/architecture-detail/standard-contracts.md:122#1
- cascade-compare-architecture.md 3 · applied · .andromeda/architecture-detail/standard-contracts.md:139#1
- cascade-compare-architecture.md 4 · applied · .andromeda/architecture-detail/standard-contracts.md:140+#1

## Edits
- line: .andromeda/architecture-detail/conventions.md:33
  old: «`EndpointLost` · `ServerVerify`, each word through `as_str`)»
  new: «`EndpointLost` · `ServerVerify` · `SocketDir`, each word through `as_str`, `SocketDir`'s `socket-dir-refused`)»
- line: .andromeda/architecture-detail/conventions.md:42
  old: «`squatted-name` · `internal-error`;»
  new: «`squatted-name` · `socket-dir-refused` · `internal-error`;»
- line: .andromeda/architecture-detail/conventions.md:42
  old: «prints none (`viola revive --list` has two refusals: the strict-modes check, the stderr pair or its document and exit 1 with no log line, and `no-session`, the stderr pair or its document and exit 1 with, under a parsed `VIOLA_NAME`, `process-exit{subject:"self", exit_code:1, detail:"no-session"}` in the caller's `cli-<VIOLA_NAME>.ndjson`);»
  new: «prints none»
- after: .andromeda/architecture-detail/conventions.md:42
  anchor: « chunk 2026-10-16-unwrapped-sessions-on-the-board added the third) are taken in a fixed order and stop at the first refusal: `detail:"already-live"` or `detail:"stale-heartbeat"` (`run`'s own collision check and its two pairs, unchanged), `detail:"no-session"` (two pairs: `unable: <name> has no logged session to resume`, and `unable: <name> has no logged session with that id` for an `--id` the log does not hold), `detail:"session-live"` (`unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`, the founder's wording, his own answer to P4 fork 4 of chunk 2026-10-16-unwrapped-sessions-on-the-board, `inputs#I1`; one `claude agents --json` read lists the logged id live in another `claude`; skipped under `--fork`; a failed read passes) and `detail:"cwd-missing"` (`unable: <name>'s recorded directory is missing`), each one `unable:` / `hint:` pair on stderr that holds no recorded directory, pid or logged id and one `process-exit{subject:"self", exit_code:1, detail}` in `run-<name>.ndjson`; under `--json` each of these revive refusals, the strict-modes one included, and every refusal of the start it shares with `run` is instead one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`; `client::refused_document`) with stderr empty, the same exit 1 and the same log lines (chunk 2026-10-17-cli-machine-contract), and a `pump_child` error, the child having held the terminal, prints none (`viola revive --list` has two refusals: the strict-modes check, the stderr pair or its document and exit 1 with no log line, and `no-session`, the stderr pair or its document and exit 1 with, under a parsed `VIOLA_NAME`, `process-exit{subject:"self", exit_code:1, detail:"no-session"}` in the caller's `cli-<VIOLA_NAME>.ndjson`);»
  add: « (`viola revive --list` has two refusals: the strict-modes check, the stderr pair or its document and exit 1 with no log line, and `no-session`, the stderr pair or its document and exit 1 with, under a parsed `VIOLA_NAME`, `process-exit{subject:"self", exit_code:1, detail:"no-session"}` in the caller's `cli-<VIOLA_NAME>.ndjson`);»
- line: .andromeda/architecture-detail/conventions.md:64
  old: «An absence: the socket-directory refusal has no code; `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site. OPEN — the socket-directory refusal's code · owner: the route entry "Unix endpoint and home hardening".»
  new: «Exit `1` also covers `viola run` and `viola revive`'s start refusing the socket directory (`socket_dir_ready` in `src/cmd/run.rs`, before any snapshot, child or `Server::bind`): the stderr `unable:` / `hint:` pair (its wording: design-system, Surface: cli), or under `viola revive --json` the document with code `socket-dir-refused`, and one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`; no path, pid or uid. A failed create and a refused check read this one code; a `Server::bind` failure does not.»
- line: .andromeda/architecture-detail/conventions.md:88
  old: «(the connected peer's pid or OS start time is not the snapshot's, or the check cannot be made)»
  new: «(on Unix a connected peer whose euid is not this user's or cannot be read, `peer_admitted`; on Linux and Windows the connected peer's pid or OS start time not the snapshot's, or that check not made)»
- line: .andromeda/architecture-detail/conventions.md:88
  old: «`link` / `unlink`) come first, before any connect,»
  new: «`link` / `unlink`) and then the socket-directory check on the recorded endpoint (`check_endpoint_dir`, exit 21, `socket-dir-refused`) come first, before any connect,»
- line: .andromeda/architecture-detail/conventions.md:94
  old: «holds the unwrapped-name hint and the endpoint-lost hint, each last»
  new: «holds the unwrapped-name hint, the endpoint-lost hint and the socket-directory hint, each last»
- line: .andromeda/architecture-detail/conventions.md:94
  old: «`endpoint-lost` · `server-verify`; `not-running`»
  new: «`endpoint-lost` · `server-verify` · `socket-dir`; `not-running`»
- line: .andromeda/architecture-detail/conventions.md:100
  old: «is claimed off macOS only; `v1-18`'s named carve-out leaves macOS's server check with the route entry "Unix endpoint and home hardening".»
  new: «is claimed on Linux, Windows and macOS, where on macOS it means an euid mismatch alone; macOS compares no peer pid, `v1-18`'s named carve-out, owned by no route entry.»
- after: .andromeda/architecture-detail/conventions.md:114
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-exit-codes -->»
  add: «»
  add: «- Exit `21`'s refused socket directory: on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, `src/cmd/client.rs` `connect`'s refused check is `CallError::SocketDirRefused` to `Unreachable::SocketDir`, writing one `process-exit` at WARN with `exit_code` 21, `detail` `socket-dir-refused` and `during` `connect`, and no `channel-request`. The start's refusal (exit `1`, above) is a step with no span and one line.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/conventions.md:118
  old: «`endpoint-lost` · `server-verify-failed`; `wrapper-fault`»
  new: «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`; `wrapper-fault`»
- line: .andromeda/architecture-detail/conventions.md:125
  old: «(its pid or OS start time not the snapshot's, or the check cannot be made; the start time read on `tokio::task::spawn_blocking`)»
  new: «(on Unix its peer euid not this user's or unreadable, `peer_admitted`; else its pid or OS start time not the snapshot's, or the check cannot be made; either reads `UnreachableCause::ServerVerify`; the start time read on `tokio::task::spawn_blocking`)»
- after: .andromeda/architecture-detail/conventions.md:127
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · conv-mcp-results -->»
  add: «»
  add: «- A frame tool (`send`, `wait`, `last`, `answer`) meeting a refused socket directory: MCP's `frame`, after `live_endpoint`, runs `check_endpoint_dir` and answers `result::unreachable(UnreachableCause::SocketDir)`, the `instance-unreachable` `isError` result with `detail: "socket-dir-refused"`, with no frame written and no line by rule, `isError` the record.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/standard-contracts.md:57
  old: «compares the connected stream's peer pid and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read (Session liveness, below); a mismatch, or a check that cannot be made, writes no frame. macOS reports no peer pid and stays a recorded gap.»
  new: «reads the connected stream's peer euid (`Client::server_euid`, `TokioClient::server_euid`) and, where it is reported (`PEER_EUID_REPORTED`, Unix), refuses another euid or one it cannot read (`peer_admitted`); it then compares the peer pid and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read (Session liveness, below); a refusal, a mismatch, or a check that cannot be made, writes no frame. macOS reports no peer pid and holds the euid check alone, the named carve-out; Windows reads no euid, its DACL and the pid comparison the controls.»
- line: .andromeda/architecture-detail/standard-contracts.md:63
  old: «and only a real snapshot goes on to the comparison.»
  new: «and only a real snapshot goes on to the comparison. Between that read and the connect each client checks the recorded endpoint's directory (`viola_state::strict::check_endpoint_dir`), and a refused directory writes no frame: `src/cmd/client.rs` `connect` (`CallError::SocketDirRefused`, one `process-exit`), the hook's `connect` (`NotSent::SocketDir`, one `hook-decision`) and MCP's `frame` (`UnreachableCause::SocketDir`, no line).»
- line: .andromeda/architecture-detail/standard-contracts.md:93
  old: «once its instance check (`trusted`) and its server check have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot or a failed server check sends nothing and the hook fails open)»
  new: «once its instance check (`trusted`), its socket-directory check and its server check, the peer euid's included, have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot, a refused socket directory or a failed server check sends nothing and the hook fails open)»
- line: .andromeda/architecture-detail/standard-contracts.md:111
  old: «on stdout (exit 21), with stderr empty and no line in any log.»
  new: «on stdout (exit 21), with stderr empty and no line in any log. A refused socket directory at either verb, met in `connect` after that check, prints the same document with `detail` `socket-dir-refused` (exit 21), with one `process-exit` at WARN.»
- line: .andromeda/architecture-detail/standard-contracts.md:112
  old: «one of five closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`; its `v` is 1 and its `error` `instance-unreachable` on all five.»
  new: «one of six closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`; its `v` is 1 and its `error` `instance-unreachable` on all six.»
- line: .andromeda/architecture-detail/standard-contracts.md:118
  old: «before anything under the home is opened, with stderr empty and no line in any log.»
  new: «before anything under the home is opened, with stderr empty and no line in any log. A refused socket directory at any of the six, met in `connect` after that check, prints the same document with `detail` `socket-dir-refused` (exit 21), with one `process-exit` at WARN and no `channel-request`.»
- line: .andromeda/architecture-detail/standard-contracts.md:122
  old: «for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and a program»
  new: «for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and a program»
- line: .andromeda/architecture-detail/standard-contracts.md:139
  old: «so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the server check.»
  new: «so a frame-writing client takes the instance's strict-modes check first, then the liveness-only pre-check, then the socket-directory check on the recorded endpoint, then on its connected stream the peer euid decision, then the server check.»
- after: .andromeda/architecture-detail/standard-contracts.md:140
  anchor: «- Where `live_endpoint` gives none, `viola_state::liveness::without_endpoint(instance_dir) -> UnreachableCause` reads the cause through `symlink_metadata`: a directory reads `InstanceDead`, anything else (absent, a regular file, a symlink) `UnwrappedName`. It opens nothing and writes no line, and the strict-modes check comes first at every caller: `client::Unreachable::without_endpoint` and `frame` in `viola-mcp`'s `tools.rs`. `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.»
  add: «- The peer euid decision is `viola_channel`'s: `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer it cannot read is refused where reported; `peer_admitted(observed)` is that decision over this process's `own_euid()` and `PEER_EUID_REPORTED` (`cfg!(unix)`). `Client::server_euid` and `TokioClient::server_euid` read the connected peer through `peer_euid_of` (`peer_creds().ok()?.euid()` on Unix, none on Windows), writing and logging nothing.»

## Patterns
[[pattern]]
id = "arch2-01"
fixed = 'the socket-directory refusal has no code'

[[pattern]]
id = "arch2-02"
fixed = 'claimed off macOS only'

[[pattern]]
id = "arch2-03"
fixed = 'reports no peer pid and stays a recorded gap'

[[pattern]]
id = "arch2-04"
fixed = 'one of five closed causes'

[[pattern]]
id = "arch2-05"
fixed = '`squatted-name` · `internal-error`'

[[pattern]]
id = "arch2-06"
fixed = '`endpoint-lost` · `server-verify-failed`; `wrapper-fault`'

[[pattern]]
id = "arch2-07"
fixed = 'holds the unwrapped-name hint and the endpoint-lost hint'
