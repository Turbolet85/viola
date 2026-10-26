## 2026-10-26-server-verification-before-any-frame — exit 21 and the MCP unreachable result for a server that is not the recorded wrapper
**Section:** §Conventions (conventions.md: CLI exit codes; MCP results)
**Change:**
- CLI exit codes: exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server that is not the recorded wrapper (the connected peer's pid or OS start time is not the snapshot's, or the check cannot be made): no frame is written; under `--json` the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, else the `unable  <name>  instance-unreachable` line with the server-verify hint last on stderr (its wording owned by design-system, surface cli-send). The client's own refusals (exit 13) come first; the cause has no exit code, `--json` code or `RefusalReason` of its own.
- MCP results: a frame tool (`send`, `wait`, `last`, `answer`) whose connected server is not the recorded wrapper, or whose check cannot be made, returns the `instance-unreachable` `isError` result, text `refused: instance-unreachable`, with no frame written and no log line; the start time is read on `tokio::task::spawn_blocking`.
**Why:** the chunk built the check on the six CLI verbs (each through `cmd::client::connect`) and on the MCP `frame` (through `server_verified`); its Outcome reads both against an impostor server on Linux and Windows, and no config key, `--json` code, exit code, `RefusalReason` or MCP error word was added.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — both clients read the serving process's pid; the macOS recorded gap
**Section:** §Established Decisions [Message Broker / IPC] (established-decisions.md) · §Stack and Technologies (stack-and-technologies.md: Wrapper IPC, Process liveness)
**Change:**
- [Message Broker / IPC]: `viola_channel::Client::server_pid` and `tokio_client::TokioClient::server_pid` return the connected stream's peer pid through interprocess's `peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the client end the SQOS Identification open adopts, none on macOS, where `SERVER_PID_REPORTED` is false); they write and log nothing. Every frame-writing client compares that pid and the process's OS start time with the snapshot's before its first frame, so on Linux and Windows a squatted or rogue server is never sent a frame.
- [Message Broker / IPC]: on macOS `viola_state::liveness::server_verdict` returns `Matches` with no comparison, the recorded gap; the macOS client euid and socket-directory checks and the 0755-directory exit-cause case are owed to the working-route entry "Unix endpoint and home hardening".
- Stack, Wrapper IPC: both clients read the serving process's pid through `StreamCommon::peer_creds().pid()`, the identity each frame-writing client checks before its first frame. Process liveness: sysinfo's process start time is also the observed side of that check. No pin moved.
**Why:** the chunk added the peer-pid read to both clients and the check to every frame-writing client; no dependency was added or bumped. interprocess 2.4.4's Windows peer pid was read in its source and witnessed by the two `server_pid` tests on windows-2025. The macOS cases' owner is the P5 review card's re-pin, the word of the Viola overseer as the founder's delegate.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — fork 1: a verified client meets an unreadable or newer snapshot as an absent one
**Section:** §Established Decisions [Message Broker / IPC] (established-decisions.md)
**Change:**
- A verified client meets an unreadable or newer snapshot as an absent one: every reader stays on `read_snapshot` (§Standard Contracts, Snapshot envelope), and the readers' move onto `read_snapshot_or_replay` is owed to the working-route entry "Home and code-bearing file integrity".
**Why:** fork 1, option 1, is the founder's own ruling, given by dialog at his wake on 2026-10-26 and relayed by the overseer; it replaces the overseer's PROVISIONAL delegate answer the chunk report records. The classified read is taken only after `check_instance`, the check that route entry adds before these same readers' reads, so the move goes there with it.
**Kept:** option 1 changes no outcome: every reader reads such a snapshot as absent, as before the chunk.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the hook's one connect and its fail-open on a failed server check
**Section:** §Established Decisions [Hook Transport], [Hook Contract] (established-decisions.md) · §Standard Contracts, Channel methods (standard-contracts.md: `hook.event`)
**Change:**
- [Hook Transport]: the event and dialog paths share one connect: `read_snapshot`, then `Client::connect_by`, then `verify_server`. A serving process whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame: the hook records `server-verify-failed` and exits 0 with no body, the event lost as for an unreachable endpoint.
- [Hook Transport], SessionEnd: was the channel, else a direct `try_lock` append, else nothing; now the direct append is taken when the endpoint is not reached or its server check fails.
- [Hook Contract]: a dialog hook whose server check fails sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human.
- `hook.event`: sent id-less by `hook` only once its server check has passed; a failed check sends nothing and the hook fails open.
**Why:** the chunk built the shared connect in `src/cmd/hook.rs` for `deliver` and `ask`. Its Outcome: the hook's `stop`, `pre-tool-use` and `permission-request` against an impostor exit 0 with empty streams under 1.0 s and no frame, and a SessionEnd whose check fails takes the direct append. The `hook.event` and `hook.dialog` gaps' server-identity half is retired on Linux and Windows; their strict-modes half stands until "Home and code-bearing file integrity".
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the snapshot's pid and started_at as the server identity every frame-writing client checks
**Section:** §Established Decisions [Session Liveness] (established-decisions.md) · §Occupied Resources IPC endpoints (occupied-resources.md) · §Standard Contracts, Channel methods and Session liveness (standard-contracts.md: a new detail group), Instance snapshot (architecture.md)
**Change:**
- Instance snapshot: was "`pid` and `started_at` identify the wrapper process for the liveness check"; now also, on Linux and Windows, for the server check every frame-writing client makes before its first frame: the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame (macOS reports no peer pid, the recorded gap).
- Session liveness: `viola_state::liveness::live_endpoint(instance_dir)` returns `Option<LiveEndpoint>`, `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call, so the liveness-only pre-check comes first and the server check after it. The closed `ServerVerdict` (no crate error enum) is `Matches` · `Mismatch` · `Unverifiable`. The pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)` compares in whole seconds, `started_at` read through its RFC 3339 form; with `peer_pid_reported` false it returns `Matches` with no comparison. The joined `verify_server(recorded_pid, started_at, observed_pid, peer_pid_reported)` reads the start time through `process_start_time` (a zombie reads none).
- [Session Liveness]: the snapshot's `pid` and `started_at` are also the server identity, returned by `live_endpoint` and compared by `verify_server`.
- Channel methods: before its first frame the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `hook.event` and `hook.dialog`, and of the MCP `send` / `wait` / `last` / `answer` tools, makes that comparison from its one snapshot read; a mismatch or an unmakeable check writes no frame.
- IPC endpoints: every process connecting to the recorded `endpoint` but `run`'s collision check makes the check; a failure is CLI exit 21 `instance-unreachable`, MCP `instance-unreachable`, the hook failing open.
**Why:** the chunk retired the server-identity half of the first and the fourth to eighth dated gaps on Linux and Windows (`hook.event`; CLI `send`; `wait` / `last`; `answer` + `hook.dialog`; `pause` / `release`; `viola mcp`'s four frame tools); their strict-modes half stands until "Home and code-bearing file integrity". The collision check writes no frame and verifies no server.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the diagnostics lines a failed server check writes
**Section:** §Occupied Resources Filesystem (occupied-resources.md: `diagnostics/`)
**Change:**
- `cli-<name>.ndjson`: exit 21 writes `detail:"instance-dead"` + `during`, or `detail:"server-verify-failed"` + `during:"connect"` when the serving process is not the recorded wrapper (then no `channel-request` line), at WARN, with no pid, start time, path or endpoint.
- `mcp.ndjson`: no `channel-request` / `channel-response` lines for a frame tool whose server check fails.
- `hook-<name>.ndjson`: a failed server check writes `hook-decision{detail:"server-verify-failed"}` at WARN; on the dialog path after `hook-invoked{corr:null}`, with `corr:null, decision_emitted:false`.
**Why:** `server-verify-failed` already stood in `schemas/diag-line.v1.json`'s `process-exit` and `hook-decision` `detail` enums; this build is the first that writes it, and no pid, start time, path or endpoint reaches any line (asserted). The MCP writes no line for the mismatch, as on its unreachable path.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — read_snapshot's product callers re-measured at three
**Section:** §Standard Contracts, Snapshot envelope (standard-contracts.md)
**Change:**
- `read_snapshot` has three product callers (was five, per "2026-10-10-statusline-pass-through — counts, crate edges and one order finding: the fake agent, the seeds, the snapshot readers, revive's log before its check"): `src/cmd/hook.rs` once, the read its event and dialog paths share; `run`'s collision check; `src/cmd/hook/statusline.rs`. The rule: `read_snapshot(` under `src/`, outside test modules. `src/cmd/client.rs` leaves the list: the CLI frame verbs and `viola mcp` read the snapshot inside `viola_state::liveness::live_endpoint`.
**Why:** measured at the chunk's tree. The base read four, `src/cmd/client.rs` gone since chunk 2026-10-17-mcp-server-for-drivers moved the CLI read into `live_endpoint`, and this chunk joined hook.rs's two reads in one. The plan's "four" was counted before that join: a count carried from a plan is re-measured at the wrap, never copied.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — viola-state's server identity check and the verified CLI connect in the key files
**Section:** §Infrastructure Patterns → Crate dependency direction (crate-dependency-direction.md: `viola-state`) · §Infrastructure Patterns → Project directory structure (project-directory-structure.md: `src/cmd/client.rs`, `viola-state`)
**Change:**
- Crate dependency direction: `viola-state`'s sysinfo edge also serves the start-time read of `liveness::verify_server`, made before the first frame by CLI `send` / `wait` / `last` / `answer` / `pause` / `release`, `viola hook` and `viola-mcp`'s frame tools; `live_endpoint` returns `LiveEndpoint` (the endpoint, pid and start time) from one snapshot read (was the liveness-only endpoint read, "moved unchanged from `src/cmd/client.rs`"). No dependency added.
- Directory structure, `src/cmd/client.rs`: was the channel client `send` / `wait` / `last` share; now `connect`, the verified connection every CLI frame verb (send, wait, last, answer, pause, release) opens (the live endpoint, then the server identity check, then the client), the closed `Unreachable` (`instance-dead` · `server-verify-failed`) and `CallError`, and the channel client those verbs share.
- Directory structure, `viola-state`: was "the liveness-only endpoint read"; now the live-endpoint read (`live_endpoint` → `LiveEndpoint`) and the server identity check (`ServerVerdict`, `server_verdict`, `verify_server`).
**Why:** the chunk closed the frame verbs' liveness-only pre-check as their whole guard and built `LiveEndpoint`, `ServerVerdict`, `server_verdict`, `verify_server` and `cmd::client::connect`; no dependency was added or bumped.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/
