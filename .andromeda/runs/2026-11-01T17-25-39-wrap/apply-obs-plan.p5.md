# Apply — obs-plan · part 5 of 5

## Verdicts
- fanout-obs-plan--cascade-g4.md 1 · applied · .andromeda/obs-plan-detail/metric-coverage.md:36#1
- fanout-obs-plan--cascade-g4.md 2 · applied · .andromeda/obs-plan-detail/log-coverage.md:26#1
- fanout-obs-plan--cascade-g4.md 3 · applied · .andromeda/obs-plan-detail/log-coverage.md:29#1
- fanout-obs-plan--cascade-g4.md 4 · applied · .andromeda/obs-plan-detail/log-coverage.md:31#1
- fanout-obs-plan--cascade-g4.md 5 · applied · .andromeda/obs-plan-detail/log-coverage.md:32#1
- fanout-obs-plan--cascade-g4.md 6 · applied · .andromeda/obs-plan-detail/log-coverage.md:34#1, .andromeda/obs-plan-detail/log-coverage.md:34#2
- fanout-obs-plan--cascade-g4.md 7 · applied · .andromeda/obs-plan-detail/log-coverage.md:35#1
- fanout-obs-plan--cascade-g4.md 8 · applied · .andromeda/obs-plan-detail/log-coverage.md:41+#1
- fanout-obs-plan--cascade-g4.md 9 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:22#1
- fanout-obs-plan--cascade-g4.md 10 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:44+#1
- fanout-obs-plan--cascade-g4.md 11 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:45#1
- fanout-obs-plan--cascade-g4.md 12 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:55+#1
- fanout-obs-plan--cascade-g4.md 13 · applied · .andromeda/obs-plan-detail/error-capture-reporting.md:57#1
- fanout-obs-plan--cascade-g4.md 15 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40+#1
- fanout-obs-plan--cascade-g4.md 16 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:42#1
- fanout-obs-plan--cascade-g4.md 17 · applied · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:43#1
- fanout-obs-plan--cascade-g4.md 18 · applied · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:29#1
- fanout-obs-plan--cascade-g4.md 19 · applied · .andromeda/obs-plan-detail/obs-anti-patterns.md:8#1
- fanout-obs-plan.md 5 · applied · .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:25#1

## Edits
- line: .andromeda/obs-plan-detail/metric-coverage.md:36
  old: «The CLI's server-verification refusal (Linux and Windows) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, counted by the Counter row's `detail` grouping; MCP's `wait` tool writes no line for it.»
  new: «The CLI's server-verification refusal (Linux, Windows and macOS, where it means an euid mismatch alone) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, and its socket-directory refusal one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN there, each counted by the Counter row's `detail` grouping; MCP's `wait` tool writes no line for either.»
- line: .andromeda/obs-plan-detail/log-coverage.md:26
  old: «`batch-script-child`, `internal-error`, and from»
  new: «`batch-script-child`, `internal-error`, `socket-dir-refused` (the socket directory of `viola run` or `viola revive`'s start refused or not creatable, before any snapshot, child or bind), and from»
- line: .andromeda/obs-plan-detail/log-coverage.md:29
  old: «at 19 values (the enum's members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-10-31-exit-cause-code-catalogue)»
  new: «at 20 values (the enum's members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening)»
- line: .andromeda/obs-plan-detail/log-coverage.md:31
  old: «`strict-modes-failed`, `server-verify-failed`.»
  new: «`strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`.»
- line: .andromeda/obs-plan-detail/log-coverage.md:32
  old: «`unlink`) on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows):»
  new: «`unlink`) on a server of another euid (Unix), on a server that is not the recorded wrapper, or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone):»
- line: .andromeda/obs-plan-detail/log-coverage.md:34
  old: «`server-verify-failed`, `strict-modes-failed`, `unverified-cli`, `deadline`. `viola hook statusline` uses three of the seven (the rule»
  new: «`server-verify-failed`, `socket-dir-refused`, `strict-modes-failed`, `unverified-cli`, `deadline`. `viola hook statusline` uses three of the eight (the rule»
- line: .andromeda/obs-plan-detail/log-coverage.md:34
  old: «before it the arm wrote `strict-modes-failed`, four of the seven)»
  new: «before it the arm wrote `strict-modes-failed`)»
- line: .andromeda/obs-plan-detail/log-coverage.md:35
  old: «`viola hook` on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows):»
  new: «`viola hook` on a server of another euid (Unix), on a server that is not the recorded wrapper, or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone):»
- after: .andromeda/obs-plan-detail/log-coverage.md:41
  anchor: « The other refused strict-modes checks write no line either — no `process-start`, no `process-exit`, no `strict-modes-failed`: `send`, `wait`, `last`, `answer`, `pause` and `release` through `client::refuse_untrusted` (exit 21, the instance pair, or under `--json` the `instance-unreachable` document); `viola mcp`'s frame tools (`instance-unreachable`, `detail: "strict-modes-failed"`); `viola verify`'s home-tree check (exit 1, the home pair); and the home-tree check of `viola run` and `viola revive`'s start arm (exit 1, the home or the instance pair).»
  add: « `socket-dir-refused` is a closed `detail` code on `process-exit` and on `hook-decision`: the socket directory of a start, or a recorded endpoint's directory, is refused as a symlink, as no directory, for another owner or for a foreign mode bit. The check takes the read verdict of the strict-modes checks above yet writes lines under its own word: `viola run` and `viola revive`'s start one `process-exit{subject:"self", exit_code:1}`, the eight CLI frame verbs one `process-exit{exit_code:21, during:"connect"}` at WARN with no `channel-request`, `viola hook` one `hook-decision` at WARN; only `viola mcp`'s `frame` writes no line, `isError` its record.»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:22
  old: «for `batch-script-child`, `pinned-hash-mismatch` and `squatted-name`, and the»
  new: «for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and the»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:44
  anchor: «- MCP rejects on a refused instance check: `viola mcp`'s frame tools (`send`, `wait`, `last`, `answer`) run `check_instance` on the home and the instance directory in `frame` (`crates/viola-mcp/src/tools.rs`) before `live_endpoint`; a refusal returns `instance-unreachable` with `detail: "strict-modes-failed"`, the `isError` code of the liveness pre-check, and writes no frame and no line.»
  add: «- MCP rejects on a refused socket directory: `frame` runs `check_endpoint_dir` on the recorded endpoint after `live_endpoint`; a refusal returns `instance-unreachable` with `detail: "socket-dir-refused"` through `UnreachableCause::SocketDir`, which no `RefusalReason` carries, and writes no frame and no line, the `isError` result its only record.»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:45
  old: «no `live_endpoint` is `without_endpoint`'s reading and a failed `server_verified` is `ServerVerify`;»
  new: «no `live_endpoint` is `without_endpoint`'s reading, a refused `check_endpoint_dir` is `SocketDir`, and a refused `peer_admitted` or a failed `server_verified` is `ServerVerify`;»
- after: .andromeda/obs-plan-detail/error-capture-reporting.md:55
  anchor: «- The product's `thiserror::Error` derives read 9: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (as measured at chunk 2026-10-31-sanitised-error-surfaces, a grep of `thiserror::Error` over `src` and `crates`). `AgentError`'s variants include `BatchScriptChild` and `ProgramNotFound`, their two messages fixed.»
  add: «- `Refused::NotADirectory`'s message is fixed, `a trusted state path is not a directory`, and names no path; it is a variant of `Refused`, no derive of its own, so it adds nothing to the 9.»
- line: .andromeda/obs-plan-detail/error-capture-reporting.md:57
  old: «`detail` is one of the five closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`.»
  new: «`detail` is one of the six closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`.»
- after: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40
  anchor: «   - A strict-modes refusal of the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, through `client::refuse_untrusted`) carries no pid, path or endpoint and writes no line: it prints only the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; `tests/security_negatives_strict_modes.rs` reads every output with `digit_runs` and finds no line, on Linux, macOS and Windows. The `viola mcp` frame tools refused by the same check return `instance-unreachable` with `detail: "strict-modes-failed"` and write no line (`binary(handlers)`).»
  add: «   - A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed `ViolaName`: `tests/security_negatives_socket_dir.rs` reads them with `assert_names_nothing` and `digit_runs`, on Linux and macOS (the file is `cfg(unix)`, with no case on Windows).»
- line: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:42
  old: «the three live witnesses»
  new: «the four live witnesses»
- line: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:43
  old: «`cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`)»
  new: «`cli_error_documents_hold_no_path_or_upstream_text` and `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`)»
- line: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:29
  old: «the timed path holds the server check a frame-writing hook makes before its first frame (the connected endpoint's peer pid and its OS start time against the one `snapshot.json` read; macOS the recorded gap, no comparison), bounded by the same gate, and a failed check fails open (`hook-decision{detail:"server-verify-failed"}`, exit 0)»
  new: «the timed path holds the checks a frame-writing hook makes before its first frame: the socket-directory check on the recorded endpoint's directory, the peer euid decision on Unix, then the server check (the connected endpoint's peer pid and its OS start time against the one `snapshot.json` read; macOS the recorded gap, no pid comparison), bounded by the same gate, and a failed check fails open (`hook-decision{detail:"socket-dir-refused"}` on the directory, `hook-decision{detail:"server-verify-failed"}` on the euid or the server, exit 0)»
- line: .andromeda/obs-plan-detail/obs-anti-patterns.md:8
  old: «callers `run` 7 ·»
  new: «callers `run` 8 ·»
- line: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:25
  old: «the liveness pre-check and the server check following.»
  new: «the liveness pre-check, the socket-directory check on the recorded endpoint (refused: `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, no frame), the peer euid decision and the server check following.»

## Patterns
[[pattern]]
id = "obs5-01"
fixed = 'server-verification refusal (Linux and Windows)'

[[pattern]]
id = "obs5-02"
fixed = '`batch-script-child`, `internal-error`, and from'

[[pattern]]
id = "obs5-03"
fixed = 'closes the `process-exit` `detail` at 19 values'

[[pattern]]
id = "obs5-04"
fixed = '`endpoint-lost`, `strict-modes-failed`, `server-verify-failed`.'

[[pattern]]
id = "obs5-05"
fixed = 'or a check that cannot be made (Linux and Windows)'

[[pattern]]
id = "obs5-06"
fixed = 'statusline` uses three of the seven'

[[pattern]]
id = "obs5-07"
fixed = '`strict-modes-failed`, four of the seven'

[[pattern]]
id = "obs5-08"
fixed = '`pinned-hash-mismatch` and `squatted-name`, and the'

[[pattern]]
id = "obs5-09"
fixed = 'reading and a failed `server_verified` is `ServerVerify`'

[[pattern]]
id = "obs5-10"
fixed = 'is one of the five closed words of'

[[pattern]]
id = "obs5-11"
fixed = 'the three live witnesses'

[[pattern]]
id = "obs5-12"
fixed = 'the timed path holds the server check a frame-writing hook makes'

[[pattern]]
id = "obs5-13"
fixed = 'callers `run` 7'

[[pattern]]
id = "obs5-14"
fixed = 'the liveness pre-check and the server check following'
