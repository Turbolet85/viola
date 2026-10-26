## 2026-10-26-server-verification-before-any-frame — the client's server-pid read and the Unix client euid check as built
**Section:** §Authentication & Authorization (authentication-authorization.md: the IPC client-side server verification row's Windows and Unix identity checks; security-plan.md: the Library row) · §Security Anti-Patterns (security-anti-patterns.md: the frame-before-verification ban's pid clause)
**Change:**
- Windows identity check: was `server_process_id()` must equal the snapshot `pid`; now the connected stream's peer pid (the observed server pid) must, beside the sysinfo start time against `started_at`.
- Library row, IPC: was interprocess `PipeStream::server_process_id()`; now `Stream::peer_creds()`, its `pid()` also the client's read of the serving process: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-opened client end, none on macOS.
- Unix identity check: the client-side `peer_creds().euid()` = `geteuid()` check is marked not built, owed to the route entry "Unix endpoint and home hardening"; the Linux/FreeBSD pid and start-time match is marked built on Linux (the `SO_PEERCRED` peer pid).
- The ban's pid clause: the Windows pid was `server_process_id()`; now the connected stream's peer pid, interprocess `peer_creds().pid()`, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end; the Linux/FreeBSD client euid check is owed to "Unix endpoint and home hardening".
**Why:** the build reads the server pid through interprocess 2.4.4's `peer_creds().pid()`, read in that crate's source and witnessed on `test (windows-2025)` by the two client server-pid tests. The chunk built no client euid check, and the working route's line for "Unix endpoint and home hardening" holds the peer euid check on both sides.
**Kept:** the SQOS identification-level open and its never-fall-back rule, and the macOS clause (euid plus the verified 0700 socket directory, the euid check never skipped, a recorded known gap).
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the dated gaps keep only their strict-modes half
**Section:** §Authentication & Authorization (authentication-authorization.md: the IPC client-side server verification row's interim gaps, the first and the fourth to eighth dated gaps) · §Input Validation (input-validation.md: the CLI arguments / stdin row's fourth to eighth gaps)
**Change:**
- Each of the six gaps now holds only its strict-modes half, owed to "Home and code-bearing file integrity"; was no server-identity check until that entry and "Server verification before any frame" landed, per 2026-09-27-hooks-to-normalised-events (the hook's interim gap), the fourth to seventh dated-gap entries of 2026-10-04 and 2026-10-17-mcp-server-for-drivers — the eighth dated gap; their strict-modes halves stand.
- On Linux and Windows each frame now verifies the serving process first: `viola hook`'s `hook.event` and `hook.dialog` (after its shape check) through `src/cmd/hook.rs`'s shared `connect` (`read_snapshot`, `connect_by`, `verify_server`); CLI `send`, `wait` / `last`, `answer`, `pause` / `release` after the liveness pre-check through `client::connect` (the peer pid and its start time against `pid` and `started_at` from the same `live_endpoint` read); `viola mcp`'s four tools after `TokioClient::connect` (`server_verified`, the start-time read on `tokio::task::spawn_blocking`). None reads the snapshot under strict-modes.
- Each residual (a squatted endpoint taking a frame, answering a dialog, swallowing a `pause` or answering a `release`) is closed on Linux and Windows and open on macOS (was open).
- The CLI arguments row: the five gaps are owed to "Home and code-bearing file integrity" alone; `viola send` resolves the endpoint, `pid` and `started_at` through `live_endpoint` and verifies the serving process against them before the frame on Linux and Windows.
**Why:** the report's Retired line: the server-identity half of the first and the fourth to eighth dated gaps is closed on Linux and Windows. Each gap's ratification stands as recorded. macOS reports no peer pid and stays on the recorded gap.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — a failed server check: no frame, and what each client reports
**Section:** §Authentication & Authorization (authentication-authorization.md: the server verification row's On mismatch clause; the new `values · auth` group's CLI refusal item) · §Error Handling (error-handling.md: the `hook` item; two new items)
**Change:**
- On mismatch, or a check that cannot be made: no frame is written; the CLI frame verbs exit 21 `instance-unreachable` with one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` WARN line; MCP returns `instance-unreachable` with `detail: null` and writes no line; `hook` fails open with exit 0, no body and its `hook-decision` line carrying `server-verify-failed` (was CLI/MCP `instance-unreachable` and a hook note in `diagnostics/`). No line or output holds a pid, start time, path or endpoint.
- CLI refusal (new): `send`, `wait`, `last`, `answer`, `pause`, `release` print under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, in human mode the `unable  <name>  instance-unreachable` line with the hint `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` last on stderr; the client's exit-13 refusals come first.
- The `hook` item: as landed it runs the server verification on Linux and Windows (macOS the recorded gap), and its fail-open details add `server-verify-failed`; strict-modes is owed to "Home and code-bearing file integrity" (was server verification and strict-modes joining at Epoch 6, per 2026-09-27-hooks-to-normalised-events, the hook's interim gap).
- New: a hook event or dialog whose check fails writes no frame and exits 0 with empty streams (the dialog path after `hook-invoked{corr:null}`, `decision_emitted:false`); a SessionEnd whose check fails goes through `append_session_end`.
- New: an MCP frame tool whose check fails sends no frame and returns `isError` `instance-unreachable`, `detail: null`, text `refused: instance-unreachable`, with no error word added and no log line.
**Why:** the report's built outcome: `server-verify-failed` was already in the `process-exit` and `hook-decision` detail enums, and this build is the first that writes it.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — run's collision check: the server check retired as moot
**Section:** §Authentication & Authorization (authentication-authorization.md: the server verification row's Collision check clause)
**Change:**
- Was "`run` uses the same verification"; now `run`'s collision check reads the snapshot through `read_snapshot` and judges the recorded wrapper by its `pid` and OS start time and the heartbeat; it opens no connection and writes no frame, so there is no serving process to verify and the peer-pid server check does not apply to it (retired as moot).
**Why:** the clause stood before this chunk and was never built: the collision check verifies no server. Rewriting it as built rather than giving it an owner is the overseer's founder-delegated answer at this wrap's halt, 2026-10-26, consistent with the founder's own ruling on fork 1, which keeps the collision check unchanged.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the observed server pid and the verdict recorded as built
**Section:** §Authentication & Authorization (authentication-authorization.md: the server verification row's Stack reference; the new `values · auth` group's server-pid, verdict, exiting-wrapper and macOS items) · §Bootstrap Phases (bootstrap-phases.md: the `viola-channel` client bullet)
**Change:**
- Server pid (new): `Client::server_pid` / `TokioClient::server_pid` (`server_pid_of`, interprocess 2.4.4 `StreamCommon` `peer_creds().pid()`); `SERVER_PID_REPORTED`, a `const bool` re-exported by `viola-channel`, false on macOS; the read writes and logs nothing.
- Verdict (new): the closed `viola_state::liveness::ServerVerdict { Matches, Mismatch, Unverifiable }` from the pure `server_verdict` and the joined `verify_server` (the start time through `process_start_time`), comparing the peer pid and its start time with `pid` and `started_at` taken with the `endpoint` from one `read_snapshot` call — the CLI verbs and `viola mcp` through `live_endpoint` (now `LiveEndpoint { endpoint, pid, started_at }`, its liveness pre-check first), `viola hook` through its shared `connect`. Only `Matches` writes a frame; no peer pid reported is `Matches` with no comparison.
- Exiting wrapper's Windows pipe (new): passes by design; whether sysinfo still lists a terminated process whose pipe handle is open, and SessionEnd's outcome there, is a CARRY owed to "Windows-only live measurements".
- macOS cases (new): the socket directory loosened to 0755 giving exit 21 and the client euid check are a CARRY owed to "Unix endpoint and home hardening"; the server-identity negatives compile out on macOS.
- Stack reference: adds the peer-pid API and `viola-state` liveness (`live_endpoint`, `server_verdict`, `verify_server`), called by the root bin's `cmd::client::connect` and `cmd::hook` and by `viola-mcp`'s `frame`.
- Bootstrap: was the client's server verification against a snapshot read after strict-modes; now `viola-channel` gives the SQOS open and the peer pid, the verdict lives in `viola-state` liveness, built on Linux and Windows, and the strict-modes read is owed to "Home and code-bearing file integrity".
**Why:** the report's Symbols as built; the two CARRYs are re-pinned by the P5 review card's word (delegate the Viola overseer, founder-delegated).
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the snapshot readers are three, and stay on read_snapshot
**Section:** §Input Validation (input-validation.md: the Own state files on read row)
**Change:**
- The other snapshot readers are three: `src/cmd/hook.rs` (one read shared by the event and dialog paths), `src/cmd/hook/statusline.rs` and `run`'s collision check in `src/cmd/run.rs` (was five, `src/cmd/client.rs` and `src/cmd/hook.rs` twice among them, per 2026-10-10-statusline-pass-through — revive: the session id ratified, its arguments, its log before its check, the fifth snapshot reader; its other claims stand). The rule: `read_snapshot(` under `src/`, outside test modules. `viola_state::liveness::live_endpoint`, the CLI frame verbs' and `viola mcp`'s one read, also calls `read_snapshot`.
- Every snapshot reader stays on `read_snapshot` (option 1); their move onto `read_snapshot_or_replay` (`v1-41`, test-plan §6 E5 steps 1–3) is a CARRY owed to the route entry "Home and code-bearing file integrity".
**Why:** measured at the chunk: `src/cmd/client.rs` holds no read since `live_endpoint` took it, and the hook's two reads joined into one. Option 1 is the founder's own ruling on fork 1, given by dialog at his wake on 2026-10-26 and relayed by the overseer; it replaces the delegate's provisional answer.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the frame ban's exceptions and the threat narrowed to the strict-modes half
**Section:** §Security Anti-Patterns (security-anti-patterns.md: the frame-before-verification ban's dated exceptions; security-plan.md: the Authentication ban) · §Threat Model Summary (threat-model-summary.md: the IPC endpoint boundary) · §Authentication & Authorization (security-plan.md: the server verification summary)
**Change:**
- The detail ban: the dated exceptions hold only the strict-modes half (reference values from a `snapshot.json` read without the check), owed to "Home and code-bearing file integrity"; on Linux and Windows each excepted frame verifies the serving process first (macOS the recorded gap), the CLI and MCP frames after the liveness pre-check and the server check (was every exception after the liveness-only pre-check, until both Epoch 6 entries landed).
- The master ban: the excepted frames verify the server against a snapshot read without strict-modes, owed to "Home and code-bearing file integrity"; every client's euid check and macOS's pid and start-time check are owed to "Unix endpoint and home hardening" (was the exceptions standing until both Epoch 6 entries landed).
- The summary: the client verifies against its one snapshot read, on Linux and Windows by the peer pid (Windows over the SQOS open) and its start time, a mismatch or an unmakeable check sending no frame; the strict-modes gap and the euid / macOS owners named as in the ban (was Windows the serving pid, Unix euid and pid, macOS euid and directory, the gaps until their two entries).
- Threat model, new: as built, on Linux and Windows every frame-writing client compares the peer's pid and start time with the snapshot's before its first frame and sends none on a mismatch or an unmakeable check; the snapshot read without strict-modes and macOS stay open.
**Why:** the report's Retired line; the no-borrowing rule stands in both bans.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/
