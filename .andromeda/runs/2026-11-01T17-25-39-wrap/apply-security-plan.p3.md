# Apply — security-plan · part 3 of 4

## Verdicts
- fanout-security-plan--cascade-g2.md 1 · applied · .andromeda/security-plan-detail/input-validation.md:37#1
- fanout-security-plan--cascade-g2.md 2 · applied · .andromeda/security-plan-detail/input-validation.md:37#2
- fanout-security-plan--cascade-g2.md 3 · applied · .andromeda/security-plan-detail/input-validation.md:39#1
- fanout-security-plan--cascade-g2.md 4 · applied · .andromeda/security-plan-detail/input-validation.md:42#1
- fanout-security-plan--cascade-g2.md 5 · applied · .andromeda/security-plan-detail/bootstrap-phases.md:14#1
- fanout-security-plan--cascade-g2.md 6 · applied · .andromeda/security-plan-detail/bootstrap-phases.md:15#1, .andromeda/security-plan-detail/bootstrap-phases.md:15#2
- fanout-security-plan--cascade-g2.md 7 · applied · .andromeda/security-plan-detail/bootstrap-phases.md:16#1
- fanout-security-plan--cascade-g3.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:8#1, .andromeda/security-plan-detail/error-handling.md:8#2, .andromeda/security-plan-detail/error-handling.md:8#3
- fanout-security-plan--cascade-g3.md 2 · applied · .andromeda/security-plan-detail/error-handling.md:9+#1
- fanout-security-plan--cascade-g3.md 3 · applied · .andromeda/security-plan-detail/error-handling.md:12#1
- fanout-security-plan--cascade-g3.md 4 · applied · .andromeda/security-plan-detail/error-handling.md:13#1
- fanout-security-plan--cascade-g3.md 5 · applied · .andromeda/security-plan-detail/error-handling.md:23#1
- fanout-security-plan--cascade-g3.md 6 · applied · .andromeda/security-plan-detail/error-handling.md:23+#1
- fanout-security-plan--cascade-g3.md 7 · applied · .andromeda/security-plan-detail/error-handling.md:24+#1
- fanout-security-plan--cascade-g3.md 8 · applied · .andromeda/security-plan-detail/error-handling.md:25+#1
- fanout-security-plan--cascade-g3.md 9 · applied · .andromeda/security-plan-detail/error-handling.md:27+#1
- cascade-compare-security-plan.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:20+#1
- cascade-compare-security-plan.md 3 · applied · .andromeda/security-plan-detail/error-handling.md:30+#1

## Edits
- line: .andromeda/security-plan-detail/input-validation.md:37
  old: «(the endpoint, `pid` and `started_at`) and then, on Linux and Windows, the server check before the frame»
  new: «(the endpoint, `pid` and `started_at`), `check_endpoint_dir` on the parent of that endpoint (a refusal is exit 21, `socket-dir-refused`, no frame), the connect, on Unix the peer euid decision `peer_admitted` (a refusal reads `server-verify-failed`) and then, on Linux and Windows, the server check before the frame»
- line: .andromeda/security-plan-detail/input-validation.md:37
  old: «with `detail` `strict-modes-failed`, no frame and no line.»
  new: «with `detail` `strict-modes-failed`, no frame and no line. After `live_endpoint`, `frame` runs `check_endpoint_dir` (a refusal `socket-dir-refused`) and, beside the server check, the peer euid decision (a refusal `server-verify-failed`), each returning `instance-unreachable`, no frame and no line.»
- line: .andromeda/security-plan-detail/input-validation.md:39
  old: «`squatted-name` · `internal-error`»
  new: «`squatted-name` · `socket-dir-refused` · `internal-error`»
- line: .andromeda/security-plan-detail/input-validation.md:42
  old: «then runs `check_path` on the home and on `instances/` (each through»
  new: «then runs `check_read` on the home, on every OS, and `check_path` on `instances/` (each through»
- line: .andromeda/security-plan-detail/bootstrap-phases.md:14
  old: «the `peer_creds` decision and the directory verification stay with their route entries.»
  new: «the `peer_creds` decision is `peer_admitted`, the wrapper dropping a refused peer in `accept_loop` before any frame, and the directory verification is `check_socket_dir` and `check_endpoint_dir` in `viola-state`, called at the bind and the three client seams, none inside `Client::connect`.»
- line: .andromeda/security-plan-detail/bootstrap-phases.md:15
  old: «`Stream::try_from(OwnedHandle)`) and the peer pid (`Client::server_pid`, `TokioClient::server_pid`, interprocess `peer_creds().pid()`);»
  new: «`Stream::try_from(OwnedHandle)`), the peer pid (`Client::server_pid`, `TokioClient::server_pid`, interprocess `peer_creds().pid()`) and the peer euid (`Client::server_euid`, `TokioClient::server_euid`: `peer_creds().ok()?.euid()` on Unix, none on Windows; the decision `peer_admitted`, on Linux and macOS, a refusal reading `server-verify-failed`);»
- line: .andromeda/security-plan-detail/bootstrap-phases.md:15
  old: «then makes the liveness pre-check and the server check.»
  new: «then makes the liveness pre-check, the socket-directory check on the parent of the recorded endpoint (`check_endpoint_dir`), the connect, the peer euid decision and the server check.»
- line: .andromeda/security-plan-detail/bootstrap-phases.md:16
  old: «`check_present` (the `conpty/` companions); and the `Readable` and `Symlink` refusals;»
  new: «`check_present` (the `conpty/` companions); `check_socket_dir` and `check_endpoint_dir` (the socket directory, one `symlink_metadata` read, never a chmod, off Unix `Ok(())`); and the `Readable`, `Symlink` and `NotADirectory` refusals;»
- line: .andromeda/security-plan-detail/error-handling.md:8
  old: «(server verification mismatch, oversized stdin, strict-modes failure)»
  new: «(server verification mismatch, refused socket directory, oversized stdin, strict-modes failure)»
- line: .andromeda/security-plan-detail/error-handling.md:8
  old: «the one-object parse and, on Linux and Windows, the server verification (macOS the recorded gap)»
  new: «the one-object parse and, on Unix, the socket-directory and peer euid checks and, on Linux and Windows, the server verification (macOS the recorded gap)»
- line: .andromeda/security-plan-detail/error-handling.md:8
  old: «`channel-unreachable` · `server-verify-failed`. Before»
  new: «`channel-unreachable` · `server-verify-failed` · `socket-dir-refused`. Before»
- after: .andromeda/security-plan-detail/error-handling.md:9
  anchor: «- A `viola hook` event or dialog whose server check fails writes no frame and exits 0 with empty streams; its `hook-decision` line carries `server-verify-failed` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`), and a SessionEnd whose check fails is appended through the direct `append_session_end`.»
  add: «- A `viola hook` event or dialog meeting a refused socket directory (`NotSent::SocketDir`, `Asked::SocketDir`) writes no frame and exits 0 with empty streams; its one `hook-decision` line, at WARN, carries `socket-dir-refused` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`).»
- line: .andromeda/security-plan-detail/error-handling.md:12
  old: «(the connected peer is not the recorded wrapper, or the check cannot be made)»
  new: «(the connected peer's euid is refused, on macOS the whole of the check; the peer is not the recorded wrapper; or the check cannot be made)»
- line: .andromeda/security-plan-detail/error-handling.md:13
  old: «`unwrapped-name` for anything else.»
  new: «`unwrapped-name` for anything else. With a live endpoint it then runs `check_endpoint_dir` on that endpoint's parent: a refusal sends no frame and returns the same unreachable result, `detail` `socket-dir-refused`, and no log line.»
- after: .andromeda/security-plan-detail/error-handling.md:20
  anchor: «- `ProtocolError::cause` gives `unsupported-version`, `invalid-params` and `release-from-driver` for the three `-32602` faults and nothing for the other four; `ResponseLine`'s `Drop` writes it as `channel-response`'s `detail`, a closed enum (`schemas/diag-line.v1.json`), on both the server's and the client's line.»
  add: «- A stream `accept_loop` refuses by `admitted(peer_euid_of(stream))` is dropped on the accept thread, before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no reply, no line and nothing on the terminal, there being no frame to key a `channel-response` on.»
- line: .andromeda/security-plan-detail/error-handling.md:23
  old: «a closed set of five causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`:»
  new: «a closed set of six causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`:»
- after: .andromeda/security-plan-detail/error-handling.md:23
  anchor: «- Exit 21's `detail` is a closed set of five causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`: the CLI's `--json` document reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, and MCP's `isError` result the same less `v`, its text `refused: instance-unreachable`.»
  add: «- A peer euid that `peer_admitted` refuses has no word of its own: it is `CallError::ServerUnverified` in `src/cmd/client.rs`, `NotSent::ServerUnverified` in `src/cmd/hook.rs` and `UnreachableCause::ServerVerify` in `crates/viola-mcp/src/tools.rs`, each reading `server-verify-failed`.»
- after: .andromeda/security-plan-detail/error-handling.md:24
  anchor: «- `viola_core::UnreachableCause` has no serde path; every cause word is written by the two clients: `client::unreachable_document` is the one writer of the CLI's exit-21 document (`refuse_untrusted` and `src/cmd/plugin.rs` through it; `src/cmd/list.rs`'s `refused_document` builds the board's `strict-modes-failed` as a `Value` for `board_document`), and `unreachable` in `crates/viola-mcp/src/result.rs` is MCP's.»
  add: «- `viola_core::UnreachableCause::SocketDir`, its word `socket-dir-refused`, has no `RefusalReason`; on a CLI frame verb it is `CallError::SocketDirRefused` to `Unreachable::SocketDir`: exit 21, `during` `connect`, hint cause `socket-dir`, one `process-exit` at WARN and no `channel-request`.»
- after: .andromeda/security-plan-detail/error-handling.md:25
  anchor: «- The `unwrapped-name` and `endpoint-lost` hints come last on stderr in plain ASCII, their `<name>` the parsed `ViolaName`.»
  add: «- The socket-directory refusal names only the parsed `ViolaName`, and its lines hold no path, pid or uid.»
- after: .andromeda/security-plan-detail/error-handling.md:27
  anchor: «- `viola run` and `viola revive`'s start run the home tree's strict-modes check (`check_home_tree`, over the instance and the pinned copy's key) before the wrapper log opens: a refusal exits 1 with the home pair for a home-level refusal or the instance pair for an instance-level one, and opens no log, writes no line and starts no child (`revive --json` prints `strict-modes-failed`). `viola verify` runs the check on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log, and a refusal there exits 1 with the home pair, no step line, no summary and no line; `update_stamps`'s `StampsError::StrictModes` exits 1 with the home pair, no step line and no summary, and its `process-exit`, under a parsed `VIOLA_NAME`, carries `strict-modes-failed`. On Unix a home under a regular file (unstattable) is such a refusal (the frame verbs: exit 21, the instance pair); on Windows that path stats `NotFound` and the `cli` catch-site line stands.»
  add: «- `viola run` and `viola revive`'s start, before any snapshot, child or `Server::bind`, refuse a socket directory through `refused_with` and `refuse_socket_dir`: exit 1 and one `process-exit` of `detail` `socket-dir-refused`, a step with no span.»
- after: .andromeda/security-plan-detail/error-handling.md:30
  anchor: «- `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.»
  add: «- `Refused::NotADirectory` carries the fixed `Display` message `a trusted state path is not a directory`; `check_socket_dir` returns it for a path that is no directory, `Refused::Symlink` for a symlink and `Refused::Unreadable` for an unstattable path.»

## Patterns
[[pattern]]
id = "sec3-01"
fixed = '`started_at`) and then, on Linux and Windows, the server check'

[[pattern]]
id = "sec3-02"
fixed = '`squatted-name` · `internal-error`'

[[pattern]]
id = "sec3-03"
fixed = 'runs `check_path` on the home and on `instances/`'

[[pattern]]
id = "sec3-04"
fixed = 'the directory verification stay with their route entries'

[[pattern]]
id = "sec3-05"
fixed = 'then makes the liveness pre-check and the server check'

[[pattern]]
id = "sec3-06"
fixed = 'the `Readable` and `Symlink` refusals'

[[pattern]]
id = "sec3-07"
fixed = '(server verification mismatch, oversized stdin'

[[pattern]]
id = "sec3-08"
fixed = 'the one-object parse and, on Linux and Windows, the server verification'

[[pattern]]
id = "sec3-09"
regex = '`channel-unreachable` · `server-verify-failed`\.'

[[pattern]]
id = "sec3-10"
fixed = '(the connected peer is not the recorded wrapper, or the check cannot be made)'

[[pattern]]
id = "sec3-11"
fixed = 'a closed set of five causes'
