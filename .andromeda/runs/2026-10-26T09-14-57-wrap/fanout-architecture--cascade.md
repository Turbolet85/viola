proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 15
    section: '## Stack and Technologies'
    change: 'Append to the Wrapper IPC row''s second cell, after its sentence ending «never interprocess''s default connect.», the sentence: «As landed (chunk 2026-10-26-server-verification-before-any-frame), both clients read the serving process''s pid from the connected stream through interprocess''s `StreamCommon::peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, none on macOS), the identity each frame-writing client checks before its first frame; the pin is unchanged.»'
    sidecar: 'stack Wrapper IPC row: the clients'' peer-pid read through interprocess `peer_creds().pid()` added, pin unchanged (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet: `Client::server_pid` (client.rs 57-61) and `server_pid_of` (132-139) read `peer_creds().pid()` (interprocess 2.4.4 `StreamCommon`): Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none; `TokioClient::server_pid` (tokio_client.rs 44-47); its Dependencies bullet: no pin moved.'
    basis: .andromeda/architecture-detail/stack-and-technologies.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 26
    section: '## Stack and Technologies'
    change: 'Append to the Process liveness row''s third cell: «; since chunk 2026-10-26-server-verification-before-any-frame sysinfo''s start time is also the observed side of the server check every frame-writing client makes before its first frame (the connected peer''s pid and its OS start time against the snapshot''s `pid` and `started_at`)»'
    sidecar: 'stack Process liveness row: sysinfo''s start time named as the observed side of the pre-frame server check (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet: the joined `verify_server` reads the observed start time through `process_start_time` (a zombie none), and the MCP reads it on `spawn_blocking` (tools.rs 351-353); sysinfo''s pin is unchanged (Dependencies).'
    basis: .andromeda/architecture-detail/stack-and-technologies.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 61
    section: '## Established Decisions'
    change: 'Append after the sentence ending «a non-overlapped handle adopted this way hangs (as measured at chunk 2026-09-25-security-prerequisites).»: «As landed (chunk 2026-10-26-server-verification-before-any-frame), `viola_channel::Client::server_pid` and `tokio_client::TokioClient::server_pid` return the connected stream''s peer pid through interprocess''s `peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end (the open unchanged), none on macOS, where `SERVER_PID_REPORTED` is false; they write and log nothing. Every frame-writing client compares that pid and the process''s OS start time with the snapshot''s before its first frame, so on Linux and Windows a squatted or rogue server is never sent a frame either.»'
    sidecar: 'dec Message Broker / IPC: the peer-pid read of both clients and the pre-frame server check added beside the SQOS open (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullets: `Client::server_pid` (client.rs 57-61), `TokioClient::server_pid` (tokio_client.rs 44-47), `SERVER_PID_REPORTED` false on macOS, re-exported at lib.rs:20, writing and logging nothing; the open stays `open_handle`''s SQOS + overlapped `CreateFileW` adopted by `Stream::try_from`; its Retired bullet: the six frame paths now verify the serving process before the first frame.'
    basis: .andromeda/architecture-detail/established-decisions.md:61
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 61
    section: '## Established Decisions'
    change: 'Append after the server-pid sentence added beside it: «On macOS no peer pid is reported, so `viola_state::liveness::server_verdict` returns `Matches` with no comparison (the recorded gap); the macOS client euid and socket-directory checks, and the 0755-directory exit-cause case, are owned by the working-route entry "Unix endpoint and home hardening".»'
    sidecar: 'dec Message Broker / IPC: the macOS recorded gap of the server check and its owner "Unix endpoint and home hardening" added (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet: `server_verdict` (liveness.rs 113-140) with `peer_pid_reported == false` returns `Matches` with no comparison, the macOS recorded gap; its Retired bullet and P5 card row 3 put the macOS euid and socket-directory checks and the 0755 case on "Unix endpoint and home hardening".'
    basis: .andromeda/architecture-detail/established-decisions.md:61
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 61
    section: '## Established Decisions'
    change: 'Append after the macOS-gap sentence added beside it: «What a verified client does with an unreadable or newer snapshot (fork 1 of chunk 2026-10-26-server-verification-before-any-frame) is answered PROVISIONALLY with option 1, today''s outcome kept: every reader stays on `read_snapshot`, and the readers'' move to `read_snapshot_or_replay` is re-pinned to the working-route entry "Home and code-bearing file integrity" until the founder''s own word (the overseer''s founder-delegated answer, 2026-10-26).»'
    sidecar: 'dec Message Broker / IPC: fork 1 (an unreadable or newer snapshot under a verified client) held PROVISIONAL on option 1, the readers'' move re-pinned to "Home and code-bearing file integrity" (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Decisions bullet: fork 1, option 1, every reader on `read_snapshot`, the overseer''s PROVISIONAL delegate answer (inputs#I3), standing until the founder''s own word; P5 card row 1 re-pins the move to "Home and code-bearing file integrity" (inputs#I4).'
    basis: .andromeda/architecture-detail/established-decisions.md:61
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 82
    section: '## Established Decisions'
    change: 'Append to the spine tier''s item, after «The event is lost rather than the turn delayed, and a driver recovers through `wait`''s `timeout_ms`.»: «As landed (chunk 2026-10-26-server-verification-before-any-frame), the event and dialog paths share one connect (`read_snapshot`, then `connect_by`, then `verify_server`): a serving process whose pid or OS start time is not the snapshot''s, or a check that cannot be made, gets no frame, and the hook records `server-verify-failed` and exits 0 with no body, the event lost as for an unreachable endpoint.»'
    sidecar: 'dec Hook Transport: the hook''s one shared connect and its server check added to the spine tier, a failed check failing open like an unreachable endpoint (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet for `src/cmd/hook.rs`: `enum NotSent` (181-187, `channel-unreachable` | `server-verify-failed`), a shared `connect` (`read_snapshot` → `connect_by` → `verify_server`) used by `deliver` and `ask`, one `read_snapshot(` at `:204`; its Outcome: the hook''s three impostor rows exit 0 with empty streams under 1.0 s, no frame.'
    basis: .andromeda/architecture-detail/established-decisions.md:82
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 83
    section: '## Established Decisions'
    change: 'In the SessionEnd item, after «else a direct `try_lock` append», add: «(taken too when the server check fails: the serving process is not the recorded wrapper, or the check cannot be made; `append_session_end`, chunk 2026-10-26-server-verification-before-any-frame)»'
    sidecar: 'dec Hook Transport: SessionEnd takes the direct append when its server check fails (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet for `src/cmd/hook.rs`: a SessionEnd whose check fails takes `append_session_end`; its Outcome: (arch) a SessionEnd whose check fails takes the direct append, MET by the hook unit case''s mismatch row.'
    basis: .andromeda/architecture-detail/established-decisions.md:83
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 93
    section: '## Established Decisions'
    change: 'After the sentence beginning «A sync dialog hook (`viola hook pre-tool-use|permission-request`) waits on `hook.dialog`», add: «A dialog hook whose server check fails (chunk 2026-10-26-server-verification-before-any-frame: the serving process is not the recorded wrapper, or the check cannot be made) sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human, logging `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`.»'
    sidecar: 'dec Hook Contract: a dialog hook''s failed server check named as a fail-open path, no hook.dialog sent (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet: `Asked::ServerUnverified` on the dialog path; its Schema / config bullet: the dialog path writes `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` at WARN; its Outcome: `pre-tool-use` and `permission-request` against the impostor exit 0 with empty streams and no frame.'
    basis: .andromeda/architecture-detail/established-decisions.md:93
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 125
    section: '## Established Decisions'
    change: 'After «(the wrapper serving two more methods, the two verbs writing their frame after the liveness-only pre-check, and their snapshot read without the strict-modes check)», add: «; since chunk 2026-10-26-server-verification-before-any-frame every CLI frame verb follows that pre-check with the server check on Linux and Windows (`cmd::client::connect`), so the liveness-only pre-check is no longer the whole pre-frame check there; the hold is unchanged»'
    sidecar: 'dec Links: the held verbs'' liveness-only crossing qualified, every CLI frame verb now adding the server check on Linux and Windows, the hold unchanged (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet: `cmd::client::connect` (client.rs 124-139) is the verified connection every CLI frame verb now uses; its Retired bullet: the server-identity half of the liveness-only gaps is closed on Linux and Windows, the strict-modes half standing until "Home and code-bearing file integrity".'
    basis: .andromeda/architecture-detail/established-decisions.md:125
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 175
    section: '## Established Decisions'
    change: 'After «a snapshot whose pid is dead, or alive with another start time, is `gone` whatever the heartbeat says (Session liveness).», add: «Since chunk 2026-10-26-server-verification-before-any-frame the same `pid` and `started_at`, read with the endpoint in one `read_snapshot` (`viola_state::liveness::live_endpoint` returns `LiveEndpoint {endpoint, pid, started_at}`), are also the server identity: every frame-writing client takes the liveness-only pre-check first, then compares the connected peer''s pid and its OS start time with them before its first frame (`verify_server`), on Linux and Windows.»'
    sidecar: 'dec Session Liveness: the snapshot''s pid and start time named as the server identity of the pre-frame check, read once through `live_endpoint`''s `LiveEndpoint` (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet for `viola_state::liveness`: `live_endpoint` now returns `LiveEndpoint { endpoint, pid, started_at }` from the one `read_snapshot` call, and `verify_server` compares the observed pid and start time; the liveness-only pre-check now precedes the server check (Expected amendments, pattern `liveep`).'
    basis: .andromeda/architecture-detail/established-decisions.md:175
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 34
    section: '## Conventions'
    change: 'Append to the `21` item: «; also, on `send`, `wait`, `last`, `answer`, `pause` and `release`, a server that is not the recorded wrapper (the connected peer''s pid or OS start time differs from the snapshot''s, or the check cannot be made; chunk 2026-10-26-server-verification-before-any-frame): no frame written, the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` under `--json` with stderr empty, or the `unable  <name>  instance-unreachable` line with the server-verify hint last on stderr; the client''s own refusals (exit 13) come first, and no exit code, `--json` code or `RefusalReason` is added»'
    sidecar: 'conv exit codes: exit 21 extended to a server-verification failure on the six CLI frame verbs, document unchanged, the hint last (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet for `src/cmd/{send,wait,last,answer,pause,release}.rs`: the client''s own refusals (exit 13) stay first; a mismatch or an unmakeable check is exit 21 with the unchanged document, or the `unable  <name>  instance-unreachable` line with the new hint last; its Schema / config bullet: no exit code, `--json` code or `RefusalReason` added.'
    basis: .andromeda/architecture-detail/conventions.md:34
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 40
    section: '## Conventions'
    change: 'After «An unreachable instance and a wrapper protocol fault are also `isError: true` tool results.», add: «As landed (chunk 2026-10-26-server-verification-before-any-frame), a frame tool (`send`, `wait`, `last`, `answer`) whose connected server is not the recorded wrapper (pid or OS start time not the snapshot''s, or the check cannot be made; the start-time read on `tokio::task::spawn_blocking`) returns that `instance-unreachable` result, text `refused: instance-unreachable`, with no frame and no log line.»'
    sidecar: 'conv MCP results: a frame tool facing a server that is not the recorded wrapper returns the instance-unreachable isError result, no frame, no line (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullet for `crates/viola-mcp/src/tools.rs`: `frame` verifies after `TokioClient::connect` through `server_verified` (348-357), the start-time read on `spawn_blocking` (351-353), a mismatch returning `result::unreachable()` with no frame and no new log line; its Outcome: `isError`, `{"error":"instance-unreachable","detail":null}`, text `refused: instance-unreachable`.'
    basis: .andromeda/architecture-detail/conventions.md:40
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 25
    section: '## Standard Contracts'
    change: 'Replace «its five product callers (`src/cmd/client.rs`, `run`''s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules)» with «its three product callers (`src/cmd/hook.rs`, one site shared by the event and dialog paths since chunk 2026-10-26-server-verification-before-any-frame; `run`''s collision check; `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules; the CLI frame verbs and `viola mcp` read it inside `viola_state::liveness::live_endpoint` since chunk 2026-10-17-mcp-server-for-drivers)»'
    sidecar: 'contract snapshot envelope: read_snapshot''s product callers re-measured five → three (hook.rs once, run''s collision check, hook/statusline.rs), src/cmd/client.rs dropped (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Counts bullet (pattern `readers`): `read_snapshot(` under `src/` outside test modules reads 3 product callers, `src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142`, `src/cmd/run.rs:379`; client.rs gone since chunk 2026-10-17-mcp-server-for-drivers moved the CLI read into `live_endpoint`; this chunk moved hook.rs''s two to one; this site is listed changed.'
    basis: .andromeda/architecture-detail/standard-contracts.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 47
    section: '## Standard Contracts'
    change: 'Append to the `send` item: «Before its first frame, as before every frame of `wait`, `last`, `answer`, `pause`, `release`, `hook.event`, `hook.dialog` and the MCP `send` / `wait` / `last` / `answer` tools, the client reads the connected stream''s peer pid and that process''s OS start time and compares both with the snapshot''s `pid` and `started_at` from the same one read; a mismatch or a check that cannot be made writes no frame (chunk 2026-10-26-server-verification-before-any-frame; macOS reports no peer pid and stays a recorded gap).»'
    sidecar: 'contract channel methods: the pre-frame server check of every frame-writing client added beside `send` (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Symbols bullets: `cmd::client::connect` for the six CLI verbs, the hook''s shared `connect` for `deliver` and `ask`, the MCP `frame` through `server_verified`, each comparing the peer pid and start time against the one snapshot read (`live_endpoint`, `verify_server`); a mismatch writes no frame (Outcome, three surfaces).'
    basis: .andromeda/architecture-detail/standard-contracts.md:47
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 59
    section: '## Standard Contracts'
    change: 'In «`hook.event`, sent id-less by `hook` (a notification, no reply)», after «sent id-less by `hook`» add: «once its server check has passed (chunk 2026-10-26-server-verification-before-any-frame; a failed check sends nothing and the hook fails open)»'
    sidecar: 'contract hook.event: sent only once the hook''s server check has passed (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Retired bullet: `hook.event`''s dated gap has its server-identity half retired on Linux and Windows; its Symbols bullet: `deliver` (event) goes through the shared `connect` with `verify_server`, `NotSent::server-verify-failed` writing no frame.'
    basis: .andromeda/architecture-detail/standard-contracts.md:59
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 91
    section: '## Occupied Resources'
    change: 'Replace «exit 21 with `detail:"instance-dead"` + `during` at WARN» with «exit 21 with `detail:"instance-dead"` + `during`, or since chunk 2026-10-26-server-verification-before-any-frame `detail:"server-verify-failed"` with `during:"connect"` when the serving process is not the recorded wrapper (no `channel-request` line, no frame), at WARN, no pid, start time, path or endpoint in any line»'
    sidecar: 'res diagnostics: cli-<name>.ndjson''s exit-21 process-exit gains detail server-verify-failed with during connect (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Schema / config bullet: this build first writes `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, no pid, start time, path or endpoint in any line (asserted); `Unreachable::during()` is always `connect` for the server cause (client.rs 75-82).'
    basis: .andromeda/architecture-detail/occupied-resources.md:91
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 93
    section: '## Occupied Resources'
    change: 'In «`hook-<name>.ndjson`: `hook-invoked`, `hook-decision`, `parse-rejected`», after `parse-rejected` add: «(a failed server check, since chunk 2026-10-26-server-verification-before-any-frame, writes `hook-decision{detail:"server-verify-failed"}` at WARN, the dialog path `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`; `mcp.ndjson` writes no line for an MCP frame tool''s mismatch)»'
    sidecar: 'res diagnostics: hook-<name>.ndjson''s hook-decision now writes server-verify-failed, mcp.ndjson none for the mismatch (chunk 2026-10-26-server-verification-before-any-frame)'
    rationale: 'The report''s Schema / config bullet: `hook-decision{detail:"server-verify-failed"}` at WARN into `hook-<name>.ndjson` (dialog path: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`); MCP writes no line for the mismatch.'
    basis: .andromeda/architecture-detail/occupied-resources.md:93
    dependent-of: D-new-detail
note: F2 (ServerVerdict, server_verdict, verify_server, LiveEndpoint) has no picked carrier (its home is the Session liveness contract or the Instance snapshot `pid`/`started_at` bullet), and the unpicked res-ipc-endpoints (endpoints bound by tests/security_negatives_server_identity.rs, tests/hook_fail_open.rs's Impostor and crates/viola-mcp/tests/handlers.rs, name form not stated by the report) and the cross-cutting Local endpoint trust boundary are left to the new-detail pass; D-arch-resources and D-arch-decisions found no new resource, library or contradicted decision.
