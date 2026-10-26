proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 54
    section: '## Threat Model Summary'
    change: 'After «A process holding that name while the real wrapper is gone could receive `hook.dialog` payloads and send back dialog decisions.» add «As built (chunk 2026-10-26-server-verification-before-any-frame), on Linux and Windows every frame-writing client (CLI `send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`''s event and dialog paths; `viola mcp`''s `send` / `wait` / `last` / `answer` tools) compares the connected stream''s peer pid and that process''s start time with the snapshot''s `pid` and `started_at` before its first frame, and sends no frame on a mismatch or a check that cannot be made; the snapshot is still read without the strict-modes check until "Home and code-bearing file integrity", and macOS (no peer pid) stays on the recorded gap.»'
    sidecar: 'threat-model-summary boundary-ipc: client-side server impersonation is now enforced at every frame-writing client on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame); the strict-modes half and macOS stay open.'
    rationale: 'Report **Retired**: the server-identity half of the dated gaps is closed on Linux and Windows, each client verifying the serving process before its first frame; strict-modes half stands until "Home and code-bearing file integrity"; macOS stays on the recorded gap. The task names boundary-ipc wherever it describes server impersonation as unenforced on the client side.'
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 24
    section: '## Authentication & Authorization'
    change: 'In **Windows identity check**, replace «`server_process_id()` must equal the snapshot `pid`» with «the connected stream''s peer pid, `peer_creds().pid()` (interprocess 2.4.4 `StreamCommon`, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end; read through `Client::server_pid` / `TokioClient::server_pid`), must equal the snapshot `pid`»; in **Unix identity check**, add after the euid clause «(as built, Linux checks the `SO_PEERCRED` peer pid with the start-time match; the client euid check is not built, owned by the route entry "Unix endpoint and home hardening")».'
    sidecar: 'authentication-authorization IPC client-side server verification: the Windows check reads the peer pid through `peer_creds().pid()` (`GetNamedPipeServerProcessId` on the client end), not `server_process_id()`; the Unix client euid check marked not built, owned by "Unix endpoint and home hardening".'
    rationale: 'Report Symbols: `Client::server_pid` / `server_pid_of` read `peer_creds().pid()` (interprocess 2.4.4 `StreamCommon`): Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none; Expected amendments, pattern `peerpid` names authentication-authorization.md:24 (the Windows check names `server_process_id()`; Unix client euid not built, Unix endpoint and home hardening).'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:24
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: '## Authentication & Authorization'
    change: 'Replace «**Interim gap (ratified):** until the Epoch 6 entry "Server verification before any frame" lands, `viola hook` sends `hook.event` with none of this verification (Decisions Log `2026-09-28`).» with «**Interim gap (ratified), its server-identity half closed:** `viola hook` sent `hook.event` with none of this verification (Decisions Log `2026-09-28`) until chunk 2026-10-26-server-verification-before-any-frame, which checks the serving process before the event frame on Linux and Windows (`src/cmd/hook.rs`''s shared `connect`: `read_snapshot`, then `verify_server`); the snapshot it reads still has no strict-modes check until "Home and code-bearing file integrity", and macOS stays on the recorded gap.»'
    sidecar: 'authentication-authorization: the first dated gap (`hook.event`) keeps only its strict-modes half; the serving process is verified before the event frame on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report **Retired** names `hook.event` first among the gaps whose server-identity half is closed on Linux and Windows; Symbols: `src/cmd/hook.rs` a shared `connect` (`read_snapshot` then `connect_by` then `verify_server`) used by `deliver` (event) and `ask` (dialog); pattern `gaps` lists authentication-authorization.md:25 changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:25
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: '## Authentication & Authorization'
    change: 'In the second interim gap (CLI `viola send`), replace «and checks neither the serving process''s identity nor the snapshot''s strict-modes» with «then, since chunk 2026-10-26-server-verification-before-any-frame, checks the serving process''s identity before the frame on Linux and Windows (`client::connect`: the peer pid and its start time against the snapshot''s `pid` and `started_at` from the one `live_endpoint` read; a mismatch or a check that cannot be made is exit 21 `instance-unreachable`), and still not the snapshot''s strict-modes»; qualify «The residual is a same-user process squatting a stale endpoint name» as closed on Linux and Windows and open on macOS; the gap stands only until "Home and code-bearing file integrity".'
    sidecar: 'authentication-authorization: the CLI `send` gap keeps only its strict-modes half; `send` verifies the serving process before its frame on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report **Retired** (CLI `send`) and Symbols: each verb''s `request` goes through `client::connect`, the verified connection; `live_endpoint` now returns `LiveEndpoint { endpoint, pid, started_at }` from one `read_snapshot` call; a mismatch is exit 21 with the unchanged `instance-unreachable` document.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:25
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: '## Authentication & Authorization'
    change: 'In the third interim gap here (the fifth overall, CLI `viola wait` / `viola last`), replace «and check neither the serving process''s identity nor the snapshot''s strict-modes. The residual is the same» with «then, since chunk 2026-10-26-server-verification-before-any-frame, check the serving process''s identity before the frame on Linux and Windows through the same `client::connect` (a mismatch exit 21 `instance-unreachable`, `during:"connect"`), and still not the snapshot''s strict-modes. The residual is the same, closed on Linux and Windows and open on macOS»; the gap stands only until "Home and code-bearing file integrity".'
    sidecar: 'authentication-authorization: the `wait` / `last` gap keeps only its strict-modes half; both verify the serving process before their frame on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report **Retired** (`wait` / `last`); Symbols: `src/cmd/{wait,last}.rs` go through `client::connect` inside `wait.block` / `last.client`; Schema: `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}`; pattern `gaps` lists authentication-authorization.md:25 changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:25
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 26
    section: '## Authentication & Authorization'
    change: 'In the fourth interim gap here (the sixth overall, F1), replace «; neither checks the serving process''s identity or the snapshot''s strict-modes. The residual: a process squatting the endpoint could answer a dialog, a PermissionRequest `allow` included.» with «; since chunk 2026-10-26-server-verification-before-any-frame both check the serving process''s identity before the frame on Linux and Windows (`answer` through `client::connect`, exit 21 on a mismatch; `hook.dialog` through `src/cmd/hook.rs`''s shared `connect`, failing open with `hook-decision` `server-verify-failed`), and neither checks the snapshot''s strict-modes. The residual, a process squatting the endpoint answering a dialog (a PermissionRequest `allow` included), is closed on Linux and Windows and open on macOS.»; the gap stands only until "Home and code-bearing file integrity".'
    sidecar: 'authentication-authorization: the `answer` + `hook.dialog` gap keeps only its strict-modes half; both verify the serving process before their frame on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report **Retired** (`answer` + `hook.dialog`); Symbols: `answer_of` maps `CallError::ServerUnverified` to `Unreachable::ServerUnverified`; `ask` (dialog) uses the hook''s shared `connect`, `Asked::ServerUnverified`; pattern `gaps` lists authentication-authorization.md:26 changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:26
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 26
    section: '## Authentication & Authorization'
    change: 'In the fifth interim gap here (the seventh overall, F-W1), replace «; neither checks the serving process''s identity or the snapshot''s strict-modes.» with «; since chunk 2026-10-26-server-verification-before-any-frame both check the serving process''s identity before the frame on Linux and Windows through `client::connect` (a mismatch exit 21 `instance-unreachable`), and neither checks the snapshot''s strict-modes.», and qualify the pause-swallow residual that follows («a process squatting the endpoint could swallow a `pause` … or answer a `release`») as closed on Linux and Windows and open on macOS; the gap stands only until "Home and code-bearing file integrity".'
    sidecar: 'authentication-authorization: the `pause` / `release` gap keeps only its strict-modes half; both verify the serving process before their frame on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report **Retired** (`pause` / `release`); Symbols: `src/cmd/{pause,release}.rs` go through `client::connect` inside `pause.client` / `release.client`; pattern `gaps` lists authentication-authorization.md:26 and :27 changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:26
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'In the sixth interim gap here (the eighth overall, `viola mcp`), replace «, checking neither the serving process''s identity nor the snapshot''s strict-modes.» with «; since chunk 2026-10-26-server-verification-before-any-frame each tool checks the serving process''s identity after `TokioClient::connect` and before its frame on Linux and Windows (`server_verified`, the start-time read on `tokio::task::spawn_blocking`; a mismatch returns `instance-unreachable` with no frame), still without the snapshot''s strict-modes check, which stands until "Home and code-bearing file integrity".»'
    sidecar: 'authentication-authorization: the `viola mcp` frame tools'' gap keeps only its strict-modes half; each tool verifies the serving process before its frame on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report **Retired** (`viola mcp`''s four frame tools); Symbols: `crates/viola-mcp/src/tools.rs` `frame` verifies after `TokioClient::connect` through `server_verified` (348-357), the start-time read on `spawn_blocking` (351-353), a mismatch returning `result::unreachable()` with no frame; pattern `gaps` lists authentication-authorization.md:27 changed.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Replace «**On mismatch:** CLI/MCP report `instance-unreachable`, and `hook` fails open (exit 0, no body, a note in `diagnostics/`).» with «**On mismatch** (or a check that cannot be made), as built: no frame is written; the CLI frame verbs exit 21 with the unchanged `instance-unreachable` document and one `process-exit{detail:"server-verify-failed", during:"connect"}` WARN line, MCP returns `instance-unreachable` with `detail: null` and writes no line, and `hook` fails open (exit 0, no body, its `hook-decision` line `server-verify-failed`); no line or output holds a pid, start time, path or endpoint.»'
    sidecar: 'authentication-authorization: the mismatch outcome stated as built — no frame, CLI exit 21 with `server-verify-failed` in `process-exit`, MCP `instance-unreachable`, hook fail-open with `hook-decision` `server-verify-failed`, no pid, start time, path or endpoint in any line.'
    rationale: 'Report Schema / config: this build first writes `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` into `cli-<name>.ndjson` and `hook-decision{detail:"server-verify-failed"}` into `hook-<name>.ndjson`, no pid, start time, path or endpoint in any line (asserted); MCP writes no line for the mismatch.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Replace «**Collision check:** `run` uses the same verification.» with «**Collision check:** not built as written — `run`''s collision check (`src/cmd/run.rs:379`) reads `read_snapshot` and verifies no server; it opens no connection and writes no frame (measured at chunk 2026-10-26-server-verification-before-any-frame).»'
    sidecar: 'authentication-authorization: the Collision check clause corrected — `run`''s collision check reads `read_snapshot` and verifies no server, with no connection and no frame (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report Spec claims disproved: authentication-authorization.md:27 «**Collision check:** `run` uses the same verification.» — standing before this chunk and not built: `run`''s collision check (`src/cmd/run.rs:379`) reads `read_snapshot` and verifies no server (plan Graph impact: no frame, no connection); the report leaves its disposition to Validate.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Add after line 27, in the Implementation: «**Observed server pid (as built, chunk 2026-10-26-server-verification-before-any-frame):** the connected stream''s peer pid, `Client::server_pid` / `TokioClient::server_pid` over interprocess `peer_creds().pid()`; `SERVER_PID_REPORTED`, re-exported by `viola-channel`, is false on macOS; reading it writes and logs nothing.»'
    sidecar: 'authentication-authorization: the observed server pid recorded as built — `Client::server_pid` / `TokioClient::server_pid` over `peer_creds().pid()`, `SERVER_PID_REPORTED` false on macOS.'
    rationale: 'Report Symbols: `Client::server_pid` (client.rs 57-61), `server_pid_of` (132-139), `TokioClient::server_pid` (tokio_client.rs 44-47), `SERVER_PID_REPORTED` re-exported at `crates/viola-channel/src/lib.rs:20`; writes nothing, logs nothing.'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Add after line 27, in the Implementation: «**Verdict (as built):** the closed `viola_state::liveness::ServerVerdict { Matches, Mismatch, Unverifiable }` comes from the pure `server_verdict` and the joined `verify_server`, which compare the peer pid and its OS start time with the snapshot''s `pid` and `started_at` from one `read_snapshot` call (`live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, the liveness pre-check now coming before the server check); where no peer pid is reported (macOS) the verdict is `Matches` with no comparison.»'
    sidecar: 'authentication-authorization: the server verdict recorded as built — `ServerVerdict { Matches, Mismatch, Unverifiable }` from `server_verdict` / `verify_server`, pid + OS start time against one snapshot read; macOS `Matches` with no comparison.'
    rationale: 'Report Symbols: `viola_state::liveness` `ServerVerdict` (102-111), `server_verdict` (113-140; `peer_pid_reported == false` returns `Matches` with no comparison), `verify_server` (the start time through `process_start_time`), `live_endpoint` returning `LiveEndpoint { endpoint, pid, started_at }` from the one `read_snapshot` call.'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Add after line 27, in the Implementation: «**CLI refusal (as built):** a CLI frame verb meeting a mismatch, or a check that cannot be made, sends no frame and exits 21 with the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` document, or in human mode the `unable` line with the hint `the process on <name>''s endpoint is not its recorded wrapper; it was not contacted` last on stderr; the client''s own exit-13 refusals stay first.»'
    sidecar: 'authentication-authorization: the CLI frame verbs'' server-verification refusal recorded as built — no frame, exit 21 `instance-unreachable`, the new hint in human mode, exit-13 refusals first.'
    rationale: 'Report Symbols: `src/cmd/{send,wait,last,answer,pause,release}.rs` — the client''s own refusals (exit 13) stay first; a mismatch or an unmakeable check is exit 21 with the unchanged document, or the `unable` line with the new hint last on stderr; `src/human/hints.rs` `server-verify` cause (50-52) prints the hint byte for byte.'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Add after line 27, in the Implementation: «**Exiting wrapper (Windows, CARRY):** a hook reaching an exiting wrapper''s own Windows pipe passes the check by design, that wrapper being the recorded process; whether sysinfo still lists such a terminated process, and SessionEnd''s outcome in that window, is a CARRY on the route entry "Windows-only live measurements".»'
    sidecar: 'authentication-authorization: the exiting-wrapper Windows window recorded — the check passes by design; the sysinfo and SessionEnd measurement a CARRY on "Windows-only live measurements".'
    rationale: 'Report P5 card Beyond the plan row 2 (inputs#I4): a hook reaching an exiting wrapper''s own Windows pipe passes the check by design; the measurement moves as one CARRY to "Windows-only live measurements".'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: '## Authentication & Authorization'
    change: 'Add after line 27, in the Implementation: «**macOS cases (CARRY):** the macOS server-verification cases — the per-user socket directory loosened to 0755 giving exit 21, and the client euid check — are a CARRY on the route entry "Unix endpoint and home hardening"; the new negatives compile out on macOS.»'
    sidecar: 'authentication-authorization: the macOS server-verification cases (0755 socket directory, client euid) recorded as a CARRY on "Unix endpoint and home hardening"; the new negatives compile out on macOS.'
    rationale: 'Report P5 card Beyond the plan row 3 (inputs#I4): the macOS exit-cause case and the macOS client euid check join "Unix endpoint and home hardening" as one CARRY; this chunk''s negatives compile out on macOS (`tests/security_negatives_server_identity.rs` is `#![cfg(not(target_os = "macos"))]`).'
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 28
    section: '## Authentication & Authorization'
    change: 'Replace the Stack reference «`viola-channel` client (sync + `tokio` feature); windows-sys 0.61.2; sysinfo 0.39.6» with «`viola-channel` client (sync + `tokio` feature: the SQOS open and the peer pid, `Client::server_pid` / `TokioClient::server_pid`, interprocess 2.4.4 `peer_creds().pid()`); `viola-state` liveness (`live_endpoint`, `server_verdict`, `verify_server`), called by the root bin''s `cmd::client::connect` and `cmd::hook` and by `viola-mcp`''s `frame`; windows-sys 0.61.2; sysinfo 0.39.6».'
    sidecar: 'authentication-authorization: the server-verification row''s stack reference names `viola-state` liveness as the verdict''s home and the peer-pid API.'
    rationale: 'Report Symbols and Crates: `viola-channel` supplies the peer pid only; the verdict is `viola_state::liveness` (`server_verdict`, `verify_server`), called from `src/cmd/client.rs` `connect`, `src/cmd/hook.rs`''s shared `connect` and `crates/viola-mcp/src/tools.rs` `frame`; no dependency added or bumped.'
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 30
    section: '## Input Validation'
    change: 'In the fourth interim gap, replace «until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity": `viola send` reads `snapshot.json` without the strict-modes check and resolves the endpoint after a liveness-only pre-check (IPC client-side server verification row).» with «until the Epoch 6 entry "Home and code-bearing file integrity" (its server-identity half closed by chunk 2026-10-26-server-verification-before-any-frame): `viola send` reads `snapshot.json` without the strict-modes check, resolves the endpoint, `pid` and `started_at` through the liveness pre-check `live_endpoint`, and then, on Linux and Windows, verifies the serving process against them before the frame (IPC client-side server verification row).»; the fifth to eighth gaps («until the same entries … read it the same way») then read as the strict-modes half only.'
    sidecar: 'input-validation CLI arguments / stdin: the snapshot-read gaps now wait only on "Home and code-bearing file integrity"; the frame verbs verify the serving process before the frame on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report **Retired**: the server-identity half of the fourth to eighth dated gaps is closed on Linux and Windows, their strict-modes half standing until "Home and code-bearing file integrity"; patterns `owner`, `gaps` and `liveep` list input-validation.md:30 (the liveness-only pre-check now precedes the server check).'
    basis: .andromeda/security-plan-detail/input-validation.md:30
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 52
    section: '## Input Validation'
    change: 'Replace «The five other snapshot readers (`src/cmd/client.rs`, `run`''s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still call `read_snapshot`» with «The three other snapshot readers (`src/cmd/hook.rs`, one read shared by the event and dialog paths; `src/cmd/hook/statusline.rs`; `run`''s collision check in `src/cmd/run.rs`; the rule: `read_snapshot(` under `src/`, outside test modules, measured at chunk 2026-10-26-server-verification-before-any-frame) still call `read_snapshot`, and so does `viola_state::liveness::live_endpoint`, the one read of the CLI frame verbs and `viola mcp`''s frame tools»'
    sidecar: 'input-validation own state files: the snapshot readers are three (`src/cmd/hook.rs` once, `src/cmd/hook/statusline.rs`, `src/cmd/run.rs`), not five; `src/cmd/client.rs` holds no read (chunk 2026-10-26-server-verification-before-any-frame).'
    rationale: 'Report Counts / qualifiers moved, pattern `readers`: `read_snapshot(` under `src/` outside test modules reads 3 product callers — `src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142`, `src/cmd/run.rs:379`; security-plan-detail/input-validation.md:52 names five and `src/cmd/client.rs`, gone since chunk 2026-10-17-mcp-server-for-drivers (the CLI read moved into `live_endpoint`); this chunk moved hook.rs''s two to one.'
    basis: .andromeda/security-plan-detail/input-validation.md:52
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 52
    section: '## Input Validation'
    change: 'Add after the readers sentence: «**Fork 1 (PROVISIONAL):** what a verified client does with an unreadable or newer snapshot, and the readers'' move onto `read_snapshot_or_replay`, has the delegate''s PROVISIONAL answer option 1, every reader staying on `read_snapshot` (chunk 2026-10-26-server-verification-before-any-frame, `inputs#I3`); the move is re-pinned as a CARRY on the route entry "Home and code-bearing file integrity", held for the founder''s own word.»'
    sidecar: 'input-validation own state files: fork 1 recorded PROVISIONAL — every reader stays on `read_snapshot`; the move onto `read_snapshot_or_replay` re-pinned as a CARRY on "Home and code-bearing file integrity", held for the founder''s word.'
    rationale: 'Report Decisions & corrections: fork 1, option 1, the overseer''s PROVISIONAL delegate answer (inputs#I3), standing only until the founder''s own word; P5 card Beyond the plan row 1 re-pins the readers'' move to "Home and code-bearing file integrity".'
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 15
    section: '## Bootstrap phases (derive for route / setup-project)'
    change: 'Replace «`viola-channel` client: server verification (windows-sys SQOS `CreateFileW` + `Stream::try_from(OwnedHandle)`, then pid + sysinfo start time against a snapshot read after the strict-modes check).» with «`viola-channel` client: the SQOS open (windows-sys `CreateFileW` + `Stream::try_from(OwnedHandle)`) and the peer pid (`Client::server_pid`, `TokioClient::server_pid`, interprocess `peer_creds().pid()`); the verdict, pid + sysinfo start time against the snapshot''s `pid` and `started_at`, lives in `viola-state` liveness (`verify_server`, `ServerVerdict`), called by the root bin and `viola-mcp`, landed on Linux and Windows with chunk 2026-10-26-server-verification-before-any-frame; the snapshot is read after the strict-modes check only once "Home and code-bearing file integrity" lands.»'
    sidecar: 'bootstrap-phases auth-scaffolding-baseline: `viola-channel` supplies the SQOS open and the peer pid only; the server verdict lives in `viola-state` liveness (chunk 2026-10-26-server-verification-before-any-frame); the strict-modes read still owed.'
    rationale: 'Report Symbols: `Client::server_pid` / `TokioClient::server_pid` over `peer_creds().pid()`, the open staying `open_handle`''s SQOS + overlapped `CreateFileW` adopted by `Stream::try_from`; `ServerVerdict` / `verify_server` in `viola_state::liveness`; **Retired**: strict-modes half stands until "Home and code-bearing file integrity"; pattern `gaps` lists bootstrap-phases.md:15 changed.'
    basis: .andromeda/security-plan-detail/bootstrap-phases.md:15
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 8
    section: '## Error Handling'
    change: 'Replace «and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable`; server verification and strict-modes join at Epoch 6 (Decisions Log `2026-09-28`).» with «and, since chunk 2026-10-26-server-verification-before-any-frame, the server verification (Linux and Windows; macOS the recorded gap), and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable` · `server-verify-failed`; strict-modes joins with the Epoch 6 entry "Home and code-bearing file integrity" (Decisions Log `2026-09-28`).»'
    sidecar: 'error-handling hook row: server verification has joined the hook (fail-open detail `server-verify-failed`, chunk 2026-10-26-server-verification-before-any-frame); strict-modes still to join with "Home and code-bearing file integrity".'
    rationale: 'Report Expected amendments, pattern `hookjoin`: error-handling.md:8 «server verification and strict-modes join at Epoch 6» — server verification joined, `server-verify-failed` a fail-open code this build writes; **Retired** names `hook.event`''s server-identity half.'
    basis: .andromeda/security-plan-detail/error-handling.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 8
    section: '## Error Handling'
    change: 'After the fail-open detail list add: «A `viola hook` event or dialog whose server check fails writes no frame and exits 0 with empty streams; its `hook-decision` line carries `server-verify-failed` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`), and a SessionEnd whose check fails takes the direct `append_session_end`.»'
    sidecar: 'error-handling hook row: a failed server check on the event or dialog path recorded — no frame, exit 0, empty streams, `hook-decision` `server-verify-failed`, SessionEnd through `append_session_end`.'
    rationale: 'Report Symbols: `src/cmd/hook.rs` `NotSent` (`channel-unreachable` | `server-verify-failed`), `Asked::ServerUnverified`, a SessionEnd whose check fails takes `append_session_end`; Schema: dialog path `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`; Outcome: exit 0, empty streams, `< 1.0 s`, no frame.'
    fact: F4
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 8
    section: '## Security Anti-Patterns (NEVER do these) › ### Authentication'
    change: 'Replace «verifying the server''s pid (Windows `server_process_id()`) or euid + pid (Linux/FreeBSD `peer_creds()`)» with «verifying the server''s pid (Windows: the connected stream''s peer pid, interprocess `peer_creds().pid()`, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end) or euid + pid (Linux/FreeBSD `peer_creds()`; as built, the Linux pid with the start-time match, the client euid check not yet built and owned by the route entry "Unix endpoint and home hardening")».'
    sidecar: 'security-anti-patterns Authentication: the Windows server pid named as `peer_creds().pid()` (`GetNamedPipeServerProcessId` on the client end), not `server_process_id()`; the Unix client euid check marked owed to "Unix endpoint and home hardening".'
    rationale: 'Report Symbols: `server_pid_of` reads `peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none; Expected amendments, pattern `peerpid` names security-anti-patterns.md:8 (Unix client euid not built, Unix endpoint and home hardening).'
    basis: .andromeda/security-plan-detail/security-anti-patterns.md:8
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/security-anti-patterns.md
    near: 8
    section: '## Security Anti-Patterns (NEVER do these) › ### Authentication'
    change: 'Replace «The dated exceptions, until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity" land:» with «The dated exceptions, their server-identity half closed on Linux and Windows by chunk 2026-10-26-server-verification-before-any-frame (each frame below now verifies the serving process first; macOS stays on the recorded gap), hold only the strict-modes half (the reference values taken from a `snapshot.json` read without the strict-modes check) until the Epoch 6 entry "Home and code-bearing file integrity" lands:», keeping the frame list, its rulings and «no other frame or process may borrow any of them» as written, with «each after the liveness-only pre-check» read as «each after the liveness pre-check and the server check».'
    sidecar: 'security-anti-patterns Authentication: the dated frame exceptions keep only their strict-modes half on Linux and Windows (chunk 2026-10-26-server-verification-before-any-frame); macOS stays on the recorded gap.'
    rationale: 'Report **Retired**: the server-identity half of the first and the fourth to eighth dated gaps is closed on Linux and Windows; their strict-modes half stands until "Home and code-bearing file integrity"; pattern `gaps` and `owner` list security-anti-patterns.md:8 changed.'
    basis: .andromeda/security-plan-detail/security-anti-patterns.md:8
note: F5 (MCP isError `instance-unreachable` for an unverified server) has no picked carrier — the Error Handling MCP row was not picked; the report's other server-verification sites outside the picked entities (security-plan.md:100 Library row naming `PipeStream::server_process_id()`, :114 and :439 broad lines, authentication-authorization.md:21) are the body/category passes'; the authentication-authorization category detector proposes overlapping D-retired edits on :25-:27, so dedupe at apply.
