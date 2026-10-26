# Task — architecture · 2026-10-26-server-verification-before-any-frame

## The session
Every frame-writing client now verifies the wrapper before its first frame: CLI send, wait, last, answer, pause and release, the hook's event and dialog paths, and the MCP send, wait, last and answer tools compare the connected stream's peer pid and that process's OS start time against the one snapshot read.
A mismatch or a check that cannot be made writes no frame. The CLI exits 21 with the unchanged instance-unreachable document, the hook fails open with no body, and MCP returns the instance-unreachable isError result. A SessionEnd whose check fails falls back to the direct append.
Added: the peer-pid read in viola-channel (sync and Tokio clients), the closed ServerVerdict and LiveEndpoint in viola-state liveness, the verified connect in the root bin's cmd::client, and the new server-verify hint.
Retired on Linux and Windows: the server-identity half of the dated liveness-only gaps (hook.event; CLI send; wait / last; answer + hook.dialog; pause / release; the MCP frame tools). Their strict-modes half stands until "Home and code-bearing file integrity". macOS stays on the recorded gap.
Measured: three read_snapshot product callers remain (src/cmd/hook.rs:204, src/cmd/hook/statusline.rs:142, src/cmd/run.rs:379). The masters state five. The hook's two read sites became one.
Also folded in: a test-only ETXTBSY fix in the fake-agent drift test. No dependency, IPC method, endpoint, env var, config key, exit code, RefusalReason or schema change.

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
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- Snapshot envelope readers list: any text naming five product callers of read_snapshot (src/cmd/client.rs, run's collision check, src/cmd/hook.rs twice, src/cmd/hook/statusline.rs) is stale. The measured count is three: hook.rs:204, hook/statusline.rs:142, run.rs:379. src/cmd/client.rs holds no read_snapshot call. hook.rs now reads once, where it read twice at the base.
- The owner of the readers' move to read_snapshot_or_replay (and of what a client does with an unreadable or newer snapshot): text saying it is owed to "Server verification before any frame" is stale. The move is re-pinned to "Home and code-bearing file integrity", held PROVISIONAL (fork 1, option 1: every reader stays on read_snapshot) until the founder's own word.
- Instance snapshot pid and started_at: described as identifying the wrapper for the liveness check. They are now also the server identity every frame-writing client compares before its first frame.
- Session liveness contract and decision: viola_state::liveness::live_endpoint now returns LiveEndpoint (endpoint, pid, started_at) from the one snapshot read instead of a bare endpoint. The liveness-only pre-check now comes before the server check, not in place of it.
- Message Broker / IPC decision: the Windows client still connects only through the SQOS Identification open. The adopted client end now also reads the server's pid (peer credentials). Any statement that clients write frames on liveness alone is stale on Linux and Windows.
- Any dated exception or Epoch 6 owner text in architecture saying CLI, hook or MCP frames go unverified until "Server verification before any frame": the server-identity half is closed on Linux and Windows. The strict-modes half stays owed to "Home and code-bearing file integrity". macOS stays a recorded gap.
- Hook Contract / Hook Transport: hooks fail open, and a failed server check is one more fail-open cause (exit 0, empty streams, no frame). "A lost endpoint loses the event, not the turn" now also covers an endpoint served by a process that is not the recorded wrapper. A SessionEnd whose check fails takes the direct append.
- CLI exit codes: exit 21 (instance unreachable) now also covers a server-verification mismatch or a check that cannot be made. The code set is unchanged.
- CLI --json output: the instance-unreachable document (exit 21) is unchanged and is now also printed for a server mismatch. Human mode prints the new server-verify hint last on stderr.
- MCP results convention: the instance-unreachable isError result is unchanged in shape and is now also returned on a server mismatch, with no frame written.
- Occupied Resources diagnostics: cli-name.ndjson's exit-21 process-exit now carries detail server-verify-failed with during connect. hook-name.ndjson's hook-decision now writes server-verify-failed. mcp.ndjson writes no line for it.
- Test-only IPC endpoints list (res-ipc-endpoints): it names the test files that create channel endpoints. tests/security_negatives_server_identity.rs, the Impostor server in tests/hook_fail_open.rs and the new impostor case in crates/viola-mcp/tests/handlers.rs each bind their own serving endpoint. Check whether the list must name them. The report does not state their name form.
- Local endpoint trust boundary (cross-cutting): the per-connection proof beyond same-user is now built on Linux and Windows as a client-side server pid + start-time check. On macOS there is no peer pid, so the check passes without comparison, and the euid and socket-directory checks are owned by "Unix endpoint and home hardening".
- Tokio containment (re-read, unmoved): the MCP's start-time read runs on tokio spawn_blocking inside viola-mcp. No sync crate gains tokio. viola-channel's normal graph stays tokio-free (channel-tokio-free.sh green).
- Stack (re-read, unmoved): interprocess 2.4.4's StreamCommon peer_creds().pid() is now relied on (Linux SO_PEERCRED, Windows GetNamedPipeServerProcessId). The pin is unchanged and sysinfo now supplies the observed start time. No version moved.
- Error Handling decision (indirect): the root bin gains pub(crate) Unreachable and CallError, and viola-state gains the ServerVerdict verdict enum. Check that one-enum-per-crate text is not read as broken: these are not crate error enums.
- Repository target dirs (indirect): a host-only macOS-target check lint (rustup target aarch64-apple-darwin, CARGO_TARGET_DIR=target/maccheck) was used for the first time. If architecture lists target/wincheck, the maccheck sibling is a candidate. It is not a plan gate entry.
- Delivery to an exiting wrapper on Windows (an OPEN item): a hook reaching an exiting wrapper's own pipe passes the check by design. Whether sysinfo still lists the terminated process, and what SessionEnd does in that window, is moved to "Windows-only live measurements".

## New facts
- F1 · standard-contracts · Before its first frame, every frame-writing client (CLI send, wait, last, answer, pause, release; viola hook event and dialog; MCP send, wait, last, answer) reads the connected stream's peer pid. It reads that process's OS start time and compares both with the snapshot's pid and started_at from the same one read. A mismatch or a check that cannot be made writes no frame.
- F2 · standard-contracts · viola_state::liveness holds the closed ServerVerdict (Matches, Mismatch, Unverifiable), the pure server_verdict and the joined verify_server (start time through process_start_time, a zombie none). It also holds live_endpoint returning LiveEndpoint (endpoint, pid, started_at) from one read_snapshot.
- F3 · established-decisions · viola_channel::Client::server_pid and TokioClient::server_pid return the connected stream's peer pid through interprocess peer_creds().pid(). That is Linux SO_PEERCRED, Windows GetNamedPipeServerProcessId on the SQOS-adopted client end, and none on macOS. SERVER_PID_REPORTED is false on macOS. They write and log nothing.
- F4 · established-decisions · On macOS no peer pid is reported, so server_verdict returns Matches without comparison (the recorded gap). The macOS euid and socket-directory checks, and the 0755-directory exit-cause case, are owned by "Unix endpoint and home hardening".
- F5 · conventions · A CLI server-verification failure is exit 21 with the unchanged instance-unreachable document under --json (stderr empty). In human mode it is the unable name instance-unreachable line followed by the server-verify hint. The client's own refusals (exit 13) come first. No exit code, --json code or RefusalReason is added.
- F6 · conventions · An MCP frame tool facing a server that is not the recorded wrapper returns the instance-unreachable isError result (text refused: instance-unreachable). It writes no frame and no log line.
- F7 · established-decisions · viola hook's event and dialog paths share one connect: read_snapshot, connect, verify_server. A failed check fails open with no body. A SessionEnd whose check fails takes the direct append (append_session_end).
- F8 · occupied-resources · server-verify-failed is now written. cli-name.ndjson gets process-exit with subject self, exit_code 21, detail server-verify-failed and during connect, at WARN. hook-name.ndjson gets hook-invoked (corr null), then hook-decision (decision_emitted false, detail server-verify-failed), at WARN. No pid, start time, path or endpoint appears in any line.
- F9 · established-decisions · Fork 1 (what a verified client does with an unreadable or newer snapshot) is answered PROVISIONALLY with option 1: keep today's outcome, every reader stays on read_snapshot. The readers' move to the replay is re-pinned to "Home and code-bearing file integrity" until the founder's own word.

## Untouched
- RefusalReason, its closed details and the per-verb refusal order: nothing added. A server failure is an unreachable outcome, not a refusal.
- Channel methods, frame envelope, MAX_FRAME, the JSON-RPC error codes and every method's params and ok payload: unchanged. No method or endpoint was added.
- Normalised event kinds and event data per kind: no kind or field added. A mismatch appends no event line.
- The list payload and the board: list writes no frame and does no server check.
- Endpoint naming (FNV-1a h12), the Unix socket directory and its 0600 mode, and the Windows listener SDDL: the listener side is not changed.
- Workspace crate membership, the sync-crates list and the viola-channel tokio feature layout: no crate added or removed.
- Dependency pins (tokio 1.53.3, interprocess 2.4.4, rmcp 3.4.1, sysinfo) and Cargo.lock: nothing bumped.
- run's collision check, revive, verify, plugin install and the hook statusline arm: not changed. run's collision check still reads read_snapshot and verifies no server.
- Capability ledger rows and the relied set: no row added. The server check rests on no claude CLI behaviour.
- Ledger stamps envelope, budget.json, config.json keys and the environment variables registry: unchanged.
- Wheel, links, budget governor, dialog hold and delivery confirmation: unchanged.
