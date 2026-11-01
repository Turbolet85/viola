# Apply — obs-plan · part 4 of 5

## Verdicts
- fanout-obs-plan--cascade-g3.md 1 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:21+#1
- fanout-obs-plan--cascade-g3.md 2 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:28#1, .andromeda/obs-plan-detail/span-trace-coverage.md:28#2, .andromeda/obs-plan-detail/span-trace-coverage.md:28#3
- fanout-obs-plan--cascade-g3.md 3 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:28+#1
- fanout-obs-plan--cascade-g3.md 4 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:29+#1
- fanout-obs-plan--cascade-g3.md 5 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:30+#1
- fanout-obs-plan--cascade-g3.md 6 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:30+#2
- fanout-obs-plan--cascade-g3.md 7 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:31#1, .andromeda/obs-plan-detail/span-trace-coverage.md:31#2, .andromeda/obs-plan-detail/span-trace-coverage.md:31#3
- fanout-obs-plan--cascade-g3.md 8 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:31+#1
- fanout-obs-plan--cascade-g3.md 9 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:33#1, .andromeda/obs-plan-detail/span-trace-coverage.md:33#2, .andromeda/obs-plan-detail/span-trace-coverage.md:33#3
- fanout-obs-plan--cascade-g3.md 10 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:42#1
- fanout-obs-plan--cascade-g3.md 11 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:57#1
- fanout-obs-plan--cascade-g3.md 12 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:78#1, .andromeda/obs-plan-detail/span-trace-coverage.md:78#2
- fanout-obs-plan--cascade-g3.md 13 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:93#1, .andromeda/obs-plan-detail/span-trace-coverage.md:93+#1
- fanout-obs-plan--cascade-g3.md 14 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:96#1
- fanout-obs-plan--cascade-g3.md 15 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:98#1
- fanout-obs-plan--cascade-g3.md 16 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:107#1, .andromeda/obs-plan-detail/span-trace-coverage.md:107#2
- fanout-obs-plan--cascade-g3.md 18 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:114#1, .andromeda/obs-plan-detail/span-trace-coverage.md:114#2
- fanout-obs-plan--cascade-g3.md 19 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:116#1
- fanout-obs-plan--cascade-g3.md 20 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:125#1, .andromeda/obs-plan-detail/span-trace-coverage.md:125#2
- fanout-obs-plan--cascade-g3.md 21 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:134+#1
- fanout-obs-plan--cascade-g3.md 22 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:137+#1
- fanout-obs-plan--cascade-g3.md 23 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:149#1, .andromeda/obs-plan-detail/span-trace-coverage.md:149#2
- fanout-obs-plan--cascade-g3.md 24 · applied · .andromeda/obs-plan-detail/span-trace-coverage.md:154+#1

## Edits
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:21
  anchor: «| ipc-internal (viola-channel) | none (interprocess 2.4.4 emits no `log`/`tracing`) | `channel.request` (client) / `channel.dispatch` (server) spans. `channel-request` / `channel-response` with `method`, `corr`, `conn`, `result_class`, `error_code`, `sender` and, on a `-32602`, `detail` (below). Never `params` or result bodies |»
  add: «| ipc-internal (viola-channel), the wrapper's accept | none | No span and no line: `accept_loop` filters each accepted stream by `peer_admitted` over its peer euid, so a refused stream is dropped on the accept thread, before its worker thread and before any frame. The peer reads end-of-file; no frame exists to key a line on; nothing reaches the outer terminal and the wheel does not move |»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:28
  old: «and before any frame: the connected stream's peer pid»
  new: «and before any frame: the socket-directory check `check_endpoint_dir` on the recorded endpoint, the connect, the euid decision `peer_admitted(client.server_euid())`, then `verify_server`, the connected stream's peer pid»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:28
  old: «A mismatch or a check that cannot be made opens no `channel.request` span,»
  new: «A refused euid, a mismatch or a check that cannot be made opens no `channel.request` span,»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:28
  old: «Linux and Windows; macOS reports no peer pid and stays on the recorded gap.»
  new: «The peer pid and start-time comparison runs on Linux and Windows. `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone: macOS reports no peer pid, its named carve-out.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:28
  anchor: «- **cli (short-lived verbs), the server check:** each of the eight frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) verifies the wrapper through `cmd::client::connect` inside its dispatch span, after its own client-side refusals (for each of the eight, the instance check `client::refuse_untrusted` first, of the driven instance for `link` / `unlink`, then the liveness pre-check) and before any frame: the connected stream's peer pid and that process's OS start time against the one snapshot read, a closed `ServerVerdict` (`Matches|Mismatch|Unverifiable`). A mismatch or a check that cannot be made opens no `channel.request` span, writes no `channel-request` line and ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}`. Linux and Windows; macOS reports no peer pid and stays on the recorded gap.»
  add: «- **The client euid check:** each client seam (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) reads the server's euid after the connect and before the pid comparison (`verify_server`, MCP's `server_verified`), through `Client::server_euid` / `TokioClient::server_euid`, which write and log nothing; `peer_admitted` refuses a peer of another euid, and a peer it cannot read, where euids are reported (`PEER_EUID_REPORTED`, Unix). On Windows no euid is read, and the DACL and the pid comparison stay the controls.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:29
  anchor: «- The refusal's place is asserted by captured-span cases, one per CLI frame verb (`request_refuses_an_unverified_server_inside_its_dispatch_span` in each verb's module) and one for the hook (`hook_refuses_an_unverified_server_inside_hook_handle`), each through the shared test-only helper `cmd::client::refused_inside` (`assert_refused_inside`): the refusal happens inside its scenario's root span, and each case reads red with the check removed.»
  add: «- The socket-directory refusal's place is asserted by two captured-span cases: `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` (`src/cmd/client.rs`), which opens the span by hand (`wait.block`) around `connect`, where the check sits, and `hook_refuses_a_loosened_socket_directory_inside_hook_handle` (`src/cmd/hook.rs`). Their span recorder `Seen` stands in the test module `refused_seen` of `src/cmd/client.rs`, out of the macOS-gated `refused_inside`.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:30
  anchor: «- **cli (short-lived verbs), the instance check:** the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) run `client::refuse_untrusted`, the instance check `link` / `unlink` share, on their named instance before any role file opens; a refused check (exit 21, the instance pair on stderr, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`) opens no process log and writes no line.»
  add: «- **cli (short-lived verbs), the socket-directory check:** a frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting a refused socket directory writes one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN inside its dispatch span, no `channel-request` line and no frame. The check lives in `cmd::client::connect`, on the recorded endpoint's parent, and changes no mode: a refused directory stays as found.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:30
  anchor: «- **cli (short-lived verbs), the instance check:** the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) run `client::refuse_untrusted`, the instance check `link` / `unlink` share, on their named instance before any role file opens; a refused check (exit 21, the instance pair on stderr, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`) opens no process log and writes no line.»
  add: «- The socket-directory refusal's human pair (design-system §Surface: cli) is printed on start and frame-verb refusals in plain ASCII with `hint:` last and no SGR, and names only the parsed `ViolaName`.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:31
  old: «under `hook.handle`, then `connect_by`, then `verify_server`)»
  new: «under `hook.handle`, then the socket-directory check `check_endpoint_dir`, then `connect_by`, then the euid decision `peer_admitted(client.server_euid())`, then `verify_server`)»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:31
  old: «A server that is not the recorded wrapper, or a check that cannot be made, gets no frame:»
  new: «A refused euid, a server that is not the recorded wrapper, or a check that cannot be made gets no frame:»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:31
  old: «`append_session_end`. Linux and Windows; macOS on the recorded gap.»
  new: «`append_session_end`. The peer pid and start-time comparison runs on Linux and Windows; `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone (macOS compares no peer pid, its named carve-out).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:31
  anchor: «- **cli (`viola hook`), the server check:** the event and dialog paths share one `connect` inside `hook.handle` (`read_snapshot_or_replay`, its span `state.snapshot_recover` under `hook.handle`, then `connect_by`, then `verify_server`) before any frame. Only a snapshot read whole goes on: a replayed snapshot (it carries no `endpoint`, `pid` or `started_at`) or an absent one is unreachable, so no frame is sent and the hook fails open with `hook-decision{detail:"channel-unreachable"}`, a SessionEnd taking the direct `append_session_end`. A server that is not the recorded wrapper, or a check that cannot be made, gets no frame: no `channel.request` opens, and the hook fails open (exit 0, empty streams) with `hook-decision{detail:"server-verify-failed"}`; a SessionEnd whose check fails takes the direct `append_session_end`. Linux and Windows; macOS on the recorded gap.»
  add: «- **cli (`viola hook`), the socket-directory check:** a hook meeting a refused socket directory writes one `hook-decision{detail:"socket-dir-refused"}` at WARN inside `hook.handle`, on the dialog path after `hook-invoked{corr:null}` and with `decision_emitted:false`; exit 0, empty streams, no frame.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:33
  old: «no frame and no line), then verify the wrapper after `TokioClient::connect`, inside `mcp.tool_call` (`server_verified`, the start time»
  new: «no frame and no line), then after `live_endpoint` run the socket-directory check `check_endpoint_dir` (a refusal returns `instance-unreachable` with `detail` `socket-dir-refused`, no frame and no line), then verify the wrapper after `TokioClient::connect`, inside `mcp.tool_call` (the euid decision `peer_admitted(client.server_euid())`, then `server_verified`, the start time»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:33
  old: «A mismatch or a check that cannot be made returns the codes-only»
  new: «A refused euid, a mismatch or a check that cannot be made returns the codes-only»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:33
  old: «with its own cause. Linux and Windows; macOS on the recorded gap.»
  new: «with its own cause. The peer pid and start-time comparison runs on Linux and Windows; `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone (macOS compares no peer pid, its named carve-out).»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:42
  old: «`viola revive` starts through the same function.»
  new: «`viola revive` starts through the same function. A fourth start step carries no span and writes a line only when it refuses: `socket_dir_ready` checks the endpoint's parent, creates it only when absent, then checks again, so a failed create, a lost race and a refused check read one code; a refusal is one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, before any snapshot, child or `Server::bind`, on `viola revive`'s start too. A `Server::bind` failure is not this refusal: it keeps its own outcome.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:57
  old: «`already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child` (`already-live` a live holder or a fresh replayed beat, `stale-heartbeat` a stale holder; Founder Direction 4;»
  new: «`already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child|socket-dir-refused` (`already-live` a live holder or a fresh replayed beat, `stale-heartbeat` a stale holder, `socket-dir-refused` a socket directory the start refuses, on `viola revive`'s start too; Founder Direction 4;»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:78
  old: «so the path ends as unreachable). A server that is not»
  new: «so the path ends as unreachable). Before it come the socket-directory check on the recorded endpoint, the connect and the euid decision, a refused directory or euid opening no `channel.request` either. A server that is not»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:78
  old: «and the path ends there (Linux and Windows; macOS on the recorded gap).»
  new: «and the path ends there (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:93
  old: «refusals and before any frame (Linux and Windows):»
  new: «refusals and before any frame (Linux, Windows and macOS, on macOS an euid mismatch alone):»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:93
  anchor: «  - client-side server-verification refusal, made after the client's own refusals and before any frame (Linux and Windows): no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN, and MCP `send` writes no line;»
  add: «  - client-side socket-directory refusal, made after the client's own refusals and before the connect: no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, and MCP `send` writes no line;»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:96
  old: «or `detail:"server-verify-failed"` with `during:"connect"` (the serving process is not the recorded wrapper, or the check cannot be made; no frame sent; Linux and Windows, macOS on the recorded gap).»
  new: «`detail:"server-verify-failed"` with `during:"connect"` (the serving process is of another euid or is not the recorded wrapper, or the check cannot be made; no frame sent; Linux, Windows and macOS, on macOS an euid mismatch alone), or `detail:"socket-dir-refused"` with `during:"connect"` (the recorded endpoint's socket directory refused; no frame sent).»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:98
  old: «`instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`.»
  new: «`instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during` `connect`.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:107
  old: «after the client's own refusals and before `channel.request`; a server that is not»
  new: «after the client's own refusals, the socket-directory check, the connect and the euid decision, and before `channel.request`; a refused directory or euid, a server that is not»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:107
  old: «opens no `channel.request` (Linux and Windows; macOS on the recorded gap).»
  new: «opens no `channel.request` (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:114
  old: «or `detail:"server-verify-failed"` with `during:"connect"` when the serving process is not the recorded wrapper or the check cannot be made (Linux and Windows; macOS on the recorded gap).»
  new: «`detail:"server-verify-failed"` with `during:"connect"` when the serving process is of another euid or not the recorded wrapper, or the check cannot be made (Linux, Windows and macOS, on macOS an euid mismatch alone), or `detail:"socket-dir-refused"` with `during:"connect"` when the recorded endpoint's socket directory is refused.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:114
  old: «then the liveness-only pre-check and then the server check;»
  new: «then the liveness-only pre-check, the socket-directory check, the connect, the euid decision and then the server check;»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:116
  old: «or on a server-verification refusal made before any frame»
  new: «or on a socket-directory or server-verification refusal made before any frame»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:125
  old: «before `channel.request(answer)`. A server that is not the recorded wrapper, or a check that cannot be made, opens neither:»
  new: «before `channel.request(answer)`. The socket-directory check, the connect and the euid decision come before each server check. A refused directory or euid, a server that is not the recorded wrapper, or a check that cannot be made opens neither:»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:125
  old: «and `answer` sends no frame (Linux and Windows; macOS on the recorded gap).»
  new: «and `answer` sends no frame (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:134
  anchor: «  - A failed server check is one of those fail-open paths before `dialog_id`: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`, with no `hook.dialog` frame and so no `dialog-raised`.»
  add: «  - A refused socket directory is another of those fail-open paths: `hook-invoked{corr:null}` then `hook-decision{decision_emitted:false, detail:"socket-dir-refused"}` at WARN, with no `hook.dialog` frame and so no `dialog-raised`.»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:137
  anchor: «  - a server-verification refusal on CLI `answer`, made before any frame: no `channel-request` / `channel-response` line, `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN; MCP `answer` writes no line.»
  add: «  - a socket-directory refusal on CLI `answer`, made before any frame: no `channel-request` / `channel-response` line, `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN; MCP `answer` writes no line.»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:149
  old: «run the server check before `channel.request(pause|release)`; a server that is not»
  new: «run the socket-directory check, the connect, the euid decision and then the server check before `channel.request(pause|release)`; a refused directory or euid, a server that is not»
- line: .andromeda/obs-plan-detail/span-trace-coverage.md:149
  old: «opens no `channel.request` and sends no frame (Linux and Windows; macOS on the recorded gap).»
  new: «opens no `channel.request` and sends no frame (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).»
- after: .andromeda/obs-plan-detail/span-trace-coverage.md:154
  anchor: «  - a server-verification refusal on `pause` / `release`, made before any frame: no `channel-request` / `channel-response` line, the client's `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN;»
  add: «  - a socket-directory refusal on `pause` / `release`, made before any frame: no `channel-request` / `channel-response` line, the client's `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN;»

## Patterns
[[pattern]]
id = "obs4-01"
fixed = 'macOS reports no peer pid and stays on the recorded gap'

[[pattern]]
id = "obs4-02"
fixed = 'Linux and Windows; macOS on the recorded gap'

[[pattern]]
id = "obs4-03"
fixed = 'Linux and Windows, macOS on the recorded gap'

[[pattern]]
id = "obs4-04"
fixed = 'before any frame (Linux and Windows)'

[[pattern]]
id = "obs4-05"
fixed = 'pinned-hash-mismatch|batch-script-child` ('

[[pattern]]
id = "obs4-06"
fixed = '`unwrapped-name` and `server-verify-failed` are `during` `connect`'

[[pattern]]
id = "obs4-07"
fixed = 'then the liveness-only pre-check and then the server check'

[[pattern]]
id = "obs4-08"
fixed = 'or on a server-verification refusal made before any frame'

[[pattern]]
id = "obs4-09"
fixed = 'then `connect_by`, then `verify_server`'
