# Task — security-plan · 2026-10-26-server-verification-before-any-frame

## The session
Every frame-writing client now checks the serving process before its first frame on Linux and Windows: the six CLI verbs `send`, `wait`, `last`, `answer`, `pause` and `release`, `viola hook`'s event and dialog paths, and `viola mcp`'s four frame tools `send`, `wait`, `last` and `answer`.
The check compares the connected stream's peer pid and that process's start time with the pid and `started_at` from the one `snapshot.json` read. A mismatch, or a check that cannot be made, sends no frame: the CLI exits 21 `instance-unreachable` with a new hint, the hook fails open with `server-verify-failed`, and MCP returns `instance-unreachable`.
Retired: the server-identity half of the dated gaps for `hook.event`, CLI `send`, `wait` / `last`, `answer` + `hook.dialog`, `pause` / `release` and `viola mcp`'s frame tools. Their strict-modes half stands until "Home and code-bearing file integrity". macOS stays on the recorded gap (no peer pid).
Measured: the snapshot readers are three product callers, not five. `run`'s collision check verifies no server. The MCP and CLI impostor cases landed outside the planned `cross_exit_causes.rs`, which does not exist.
No IPC method, endpoint, socket, env var, config key, exit code, `--json` code, MCP error word or dependency was added or changed. This build is the first that writes `server-verify-failed` in `process-exit` and `hook-decision` lines.

## Reached files
- tests/security_negatives_server_identity.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/lib.rs
- crates/viola-state/src/liveness.rs
- src/cmd/client.rs
- src/human/hints.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/contract_fake_agent_drift.rs

## Touches
- The IPC client-side server verification row (anti-impersonation) in §Authentication & Authorization and its detail. On Linux and Windows the pid + start-time check is now built for every frame verb. The clause that the dated interim gaps stand until their two route entries land is half stale: "Server verification before any frame" has landed, and "Home and code-bearing file integrity" is still owed.
- The detail's list of dated gaps (`hook.event`, CLI `send`, `wait` / `last`, `answer`, `hook.dialog`, `pause` / `release`, MCP frame tools). Each one's server-identity half is closed on Linux and Windows. Each one's strict-modes half still stands.
- The detail's liveness-only pre-check wording for the CLI and MCP clients. The liveness pre-check (`live_endpoint`) now comes before the server check, it is no longer the only check, and it now returns `LiveEndpoint { endpoint, pid, started_at }` instead of a bare endpoint.
- The Library row and every site naming the Windows check as `PipeStream::server_process_id()`. The build reads `peer_creds().pid()` instead, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end. On Linux it is `SO_PEERCRED`, and macOS has none. Check this in the detail's Windows verification wording, in bootstrap-phases and in the Authentication anti-pattern.
- The Unix half of the verification row (euid, then pid with start time where available). Linux now checks pid + start time. The Unix client euid check is not built and is owned by "Unix endpoint and home hardening".
- The detail's Collision check line (`run` uses the same verification). It is contradicted: `run`'s collision check reads `read_snapshot`, verifies no server, and makes no frame and no connection.
- The `~/.viola/` access control row's dated interim gaps. Only the strict-modes half remains for these clients, and that half is unchanged.
- The Authentication anti-pattern (NEVER write any channel frame before verifying the server…), in the body and in security-anti-patterns detail. Its gap list and its "until the Epoch 6 entries … land" clause now hold only the strict-modes half on Linux and Windows. macOS is still a gap.
- Input Validation › CLI arguments / stdin row and its detail. The frame verbs now verify the server before the frame. The strict-modes interim gap for snapshot reads stands.
- Input Validation detail, the snapshot-readers count. It says five product callers, naming `src/cmd/client.rs` and `src/cmd/hook.rs` twice. The measured count is three: `src/cmd/hook.rs` (one shared read), `src/cmd/hook/statusline.rs` and `src/cmd/run.rs`. `src/cmd/client.rs` holds no read.
- Input Validation › Own state files on read. The client readers stay on `read_snapshot`. Moving them to the classified replay read is no longer owed to this entry: it is re-pinned to "Home and code-bearing file integrity" (fork 1 PROVISIONAL).
- Error Handling detail, the `hook` row. It says server verification and strict-modes join at Epoch 6. Server verification has now joined, and `server-verify-failed` is a fail-open decision the hook writes, with exit 0 and no body. Strict-modes is still to join.
- Error Handling › MCP `isError`. `instance-unreachable` with `detail: null` is now also the result for a server that is not the recorded wrapper. No new error word was added.
- Bootstrap phases › auth-scaffolding-baseline. It names client server verification in `viola-channel`. That crate now supplies only the peer pid (`Client::server_pid`, `TokioClient::server_pid`). The verdict lives in `viola-state` liveness, and the root bin and `viola-mcp` call it.
- Bootstrap phases › logging-redaction-wire, indirectly through the new refusal lines. The `process-exit` and `hook-decision` lines with `server-verify-failed` carry no pid, start time, path or endpoint, and neither does any CLI output; this is asserted. The floor's clause allowing pids in `diagnostics/` is unchanged.
- Threat Model › boundary-ipc, boundary-cli, boundary-mcp and boundary-hook-stdin wherever they describe server impersonation as unenforced on the client side. On Linux and Windows it is now enforced at every frame-writing client.
- Any security test inventory for the IPC client-side check. The new homes are `tests/security_negatives_server_identity.rs`, the impostor rows in `tests/hook_fail_open.rs`, and `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` in `crates/viola-mcp/tests/handlers.rs`. There is no `cross_exit_causes.rs`.

## New facts
- F1 · authentication-authorization · The observed server pid is the connected stream's peer pid (`Client::server_pid`, `TokioClient::server_pid` over interprocess `peer_creds().pid()`). `SERVER_PID_REPORTED`, re-exported by `viola-channel`, is false on macOS. Reading it writes and logs nothing.
- F2 · authentication-authorization · The closed verdict `viola_state::liveness::ServerVerdict { Matches, Mismatch, Unverifiable }` comes from the pure `server_verdict` and the joined `verify_server`. They compare the peer pid and its OS start time with the snapshot's `pid` and `started_at` from one `read_snapshot` call. Where no peer pid is reported (macOS), the verdict is `Matches` with no comparison.
- F3 · authentication-authorization · When a CLI frame verb meets a mismatch, or a check that cannot be made, it sends no frame. It exits 21 with the unchanged `instance-unreachable` document, or in human mode the `unable` line with the hint that the process on the endpoint is not its recorded wrapper and was not contacted. The client's own exit-13 refusals stay first.
- F4 · error-handling · A `viola hook` event or dialog whose server check fails writes no frame and exits 0 with empty streams. Its `hook-decision` line carries `server-verify-failed`. A SessionEnd whose check fails takes the direct `append_session_end`.
- F5 · error-handling · A `viola mcp` frame tool whose server check fails returns `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`. It sends no frame and writes no log line. The start-time read runs on `spawn_blocking`.
- F6 · authentication-authorization · A hook reaching an exiting wrapper's own Windows pipe passes the check by design, since that wrapper is the recorded process. Whether sysinfo still lists such a terminated process, and SessionEnd's outcome in that window, is a CARRY on "Windows-only live measurements".
- F7 · authentication-authorization · The macOS server-verification cases are a CARRY on "Unix endpoint and home hardening": the per-user socket directory loosened to 0755 giving exit 21, and the client euid check. The new negatives compile out on macOS.
- F8 · input-validation · Fork 1 (what a verified client does with an unreadable or newer snapshot, and the readers' move onto `read_snapshot_or_replay`) has a PROVISIONAL delegate answer: option 1, every reader stays on `read_snapshot`. It is re-pinned as a CARRY on "Home and code-bearing file integrity", held for the founder's own word.

## Untouched
- The Windows listener's protected user + SYSTEM DACL, `accept_remote(false)`, first-instance and non-inheritable handle: the server side was not changed.
- The Unix server-side peer euid check on accept, and the per-user 0700 socket directory with chmod 0600 after bind: not changed.
- The home strict-modes check, `replace_private` and the 0700 / 0600 modes: the mechanism and its call sites did not move. Only the gap wording around the clients changes.
- Paste text and dialog free-text validation, `control-character` and `empty-text`: not reached.
- Dependency Security: no dependency was added or bumped. The interprocess 2.4.4, sysinfo and tokio pins, `deny.toml`, the sync-crate tokio bans and `channel-tokio-free.sh` are unchanged, and the macOS std target added on the host is no CI or toolchain pin.
- API Security (rmcp features `server` + `transport-io`): unchanged.
- Secret Management and the R8 strip: not reached.
- The statusline source and shell-out, the budget gate, `revive`'s preflight and `viola verify`'s runs: not reached.
- Windows DLL search restriction and the ConPTY companion re-hash: not reached.
- The driver-originated `release` refusal (`-32602` `release-from-driver`) and `from` as self-reported: the wrapper-side rule was not changed.
- The Universal no-switch-off ban: still true, since no flag, env var or config key switches the server check off.
- Data Protection: repository fixtures, verify's accepted risks and residuals, and `events.ndjson` retention are not reached.
- Threat Model data classification, tier, auth model and compliance triggers: not reached.
