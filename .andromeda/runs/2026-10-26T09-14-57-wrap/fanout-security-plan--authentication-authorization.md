proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: "## Authentication & Authorization"
    change: "Replace the first gap's sentence «**Interim gap (ratified):** until the Epoch 6 entry \"Server verification before any frame\" lands, `viola hook` sends `hook.event` with none of this verification (Decisions Log `2026-09-28`).» with «**Interim gap (ratified), its server half closed:** `viola hook` sent `hook.event` with none of this verification (Decisions Log `2026-09-28`) until chunk 2026-10-26-server-verification-before-any-frame, which verifies the serving process before the first frame on Linux and Windows; its strict-modes half stands (the first strict-modes gap below) until \"Home and code-bearing file integrity\", and macOS stays on the recorded gap.»"
    sidecar: "authentication-authorization: the first dated gap (`hook.event`) loses its server-identity half — verified before the first frame on Linux and Windows since chunk 2026-10-26-server-verification-before-any-frame; strict-modes half stands."
    rationale: "Report **Retired**: «the server-identity half of the first and the fourth to eighth dated gaps on Linux and Windows (`hook.event`; …) — each now verifies the serving process before its first frame. Their strict-modes half stands until \"Home and code-bearing file integrity\"; macOS stays on the recorded gap». Expected amendments, pattern `gaps`: authentication-authorization.md:25 changed."
    basis: .andromeda/security-plan-detail/authentication-authorization.md:25
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: "## Authentication & Authorization"
    change: "In the second interim gap (CLI `viola send`), replace «— and checks neither the serving process's identity nor the snapshot's strict-modes. The residual is a same-user process squatting a stale endpoint name; the pipe DACL and the 0700 socket directory still apply.» with «, then, since chunk 2026-10-26-server-verification-before-any-frame, the serving process's identity (`client::connect`, Linux and Windows; a mismatch exit 21 `instance-unreachable`), and still not the snapshot's strict-modes, which stands until \"Home and code-bearing file integrity\". The squatting residual is closed on Linux and Windows and stands on macOS (the recorded gap); the pipe DACL and the 0700 socket directory still apply.»"
    sidecar: "authentication-authorization: the `send` gap keeps its strict-modes half only — the serving process is verified before the `send` frame since chunk 2026-10-26-server-verification-before-any-frame."
    rationale: "Report **Retired** (CLI `send` among the fourth to eighth dated gaps) and Symbols: `src/cmd/{send,…}.rs` each `request` goes through `client::connect`; a mismatch is exit 21 with the unchanged `instance-unreachable` document."
    basis: .andromeda/security-plan-detail/authentication-authorization.md:25
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: "## Authentication & Authorization"
    change: "In the fifth dated gap (CLI `viola wait` / `viola last`), replace «and check neither the serving process's identity nor the snapshot's strict-modes. The residual is the same;» with «then, since chunk 2026-10-26-server-verification-before-any-frame, the serving process's identity (Linux and Windows), and still not the snapshot's strict-modes, which stands until \"Home and code-bearing file integrity\"; macOS keeps the residual;»."
    sidecar: "authentication-authorization: the `wait` / `last` gap keeps its strict-modes half only — server verified before the frame since chunk 2026-10-26-server-verification-before-any-frame."
    rationale: "Report **Retired** (`wait` / `last`); Spec claims disproved: `server-verify-failed` now occurs on `wait` and `last` on Linux and Windows (`tests/security_negatives_server_identity.rs`)."
    basis: .andromeda/security-plan-detail/authentication-authorization.md:25
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 26
    section: "## Authentication & Authorization"
    change: "In the sixth dated gap (CLI `viola answer` + `hook.dialog`), replace «; neither checks the serving process's identity or the snapshot's strict-modes. The residual: a process squatting the endpoint could answer a dialog, a PermissionRequest `allow` included.» with «, and since chunk 2026-10-26-server-verification-before-any-frame each verifies the serving process first (Linux and Windows; the hook through its shared `connect`, a mismatch failing open with `hook-decision{detail:\"server-verify-failed\"}`); neither checks the snapshot's strict-modes, which stands until \"Home and code-bearing file integrity\". The residual (a squatter answering a dialog, a PermissionRequest `allow` included) stands on macOS only.»"
    sidecar: "authentication-authorization: the `answer` / `hook.dialog` gap keeps its strict-modes half only — server verified before the frame since chunk 2026-10-26-server-verification-before-any-frame."
    rationale: "Report **Retired** (`answer` + `hook.dialog`); Symbols `src/cmd/hook.rs`: a shared `connect` (`read_snapshot` → `connect_by` → `verify_server`) used by `deliver` and `ask`; Schema/config: `hook-decision{corr:null, decision_emitted:false, detail:\"server-verify-failed\"}`."
    basis: .andromeda/security-plan-detail/authentication-authorization.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 26
    section: "## Authentication & Authorization"
    change: "In the seventh dated gap (CLI `viola pause` / `viola release`), replace «; neither checks the serving process's identity or the snapshot's strict-modes.» with «, then, since chunk 2026-10-26-server-verification-before-any-frame, the serving process's identity (Linux and Windows); neither checks the snapshot's strict-modes, which stands until \"Home and code-bearing file integrity\".», and on line 27 qualify «The residual: a process squatting the endpoint could swallow a `pause` … or answer a `release`.» as standing on macOS only."
    sidecar: "authentication-authorization: the `pause` / `release` gap keeps its strict-modes half only — server verified before the frame since chunk 2026-10-26-server-verification-before-any-frame; the pause-swallow residual stands on macOS only."
    rationale: "Report **Retired** (`pause` / `release`); Symbols: `src/cmd/{pause,release}.rs` request through `client::connect` inside `pause.client` / `release.client`."
    basis: .andromeda/security-plan-detail/authentication-authorization.md:26
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: "In the eighth dated gap (`viola mcp`'s tools), replace «, checking neither the serving process's identity nor the snapshot's strict-modes.» with «, then, since chunk 2026-10-26-server-verification-before-any-frame, the serving process's identity (`server_verified` after `TokioClient::connect`, the start-time read on `spawn_blocking`; a mismatch `instance-unreachable`, no frame, no line), and still not the snapshot's strict-modes, which stands until \"Home and code-bearing file integrity\".»"
    sidecar: "authentication-authorization: the `viola mcp` frame-tools gap keeps its strict-modes half only — server verified before the frame since chunk 2026-10-26-server-verification-before-any-frame."
    rationale: "Report **Retired** (`viola mcp`'s four frame tools); Symbols `crates/viola-mcp/src/tools.rs`: `frame` verifies after `TokioClient::connect` through `server_verified` (348-357), a mismatch returning `result::unreachable()` with no frame, no new log line."
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
    dependent-of: D-retired
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 24
    section: "## Authentication & Authorization"
    change: "In **Windows identity check**, replace «`server_process_id()` must equal the snapshot `pid`» with «the serving pid, read through interprocess 2.4.4's `peer_creds().pid()` (`GetNamedPipeServerProcessId` on the SQOS-adopted client end; `viola_channel::Client::server_pid`), must equal the snapshot `pid`»; in **Unix identity check** and **macOS**, mark the client-side `peer_creds().euid() == geteuid()` check and macOS's euid + socket-directory verification as not built, owed to the route entry \"Unix endpoint and home hardening\" (on macOS `SERVER_PID_REPORTED` is false and `server_verdict` returns `Matches` with no comparison)."
    sidecar: "authentication-authorization: the client identity check named as built — `peer_creds().pid()` (Windows `GetNamedPipeServerProcessId`, Linux `SO_PEERCRED`) + sysinfo start time; client euid and macOS directory verification owed to \"Unix endpoint and home hardening\"."
    rationale: "Report Symbols: `Client::server_pid` reads `peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none), `server_verdict`'s `peer_pid_reported == false` → `Matches` with no comparison; Expected amendments, pattern `peerpid`: «the Windows check names `server_process_id()` while the build reads `peer_creds().pid()` …; Unix client euid not built (Unix endpoint and home hardening)», authentication-authorization.md:24 listed; **Retired**: «the euid and socket-directory checks owned by \"Unix endpoint and home hardening\"»."
    basis: .andromeda/security-plan-detail/authentication-authorization.md:24
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: "Replace «**Collision check:** `run` uses the same verification.» with «**Collision check:** `run`'s collision check reads `read_snapshot` (`src/cmd/run.rs:379`) and opens no connection and writes no frame, so it makes no server verification (chunk 2026-10-26-server-verification-before-any-frame, plan Graph impact).» — disposition owed at Validate."
    sidecar: "authentication-authorization: the collision-check clause corrected — `run`'s collision check reads the snapshot and verifies no server (no connection, no frame)."
    rationale: "Report Spec claims disproved: «authentication-authorization.md:27 «**Collision check:** `run` uses the same verification.» — standing before this chunk and not built: `run`'s collision check (`src/cmd/run.rs:379`) reads `read_snapshot` and verifies no server (plan Graph impact: «no change: no frame, no connection»). Pattern `gaps` (the row at :27). Disposition owed at Validate.»"
    basis: .andromeda/security-plan-detail/authentication-authorization.md:27
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 27
    section: "## Authentication & Authorization"
    change: "Add after line 27, as one continuation line of the Implementation: « **As landed (chunk 2026-10-26-server-verification-before-any-frame), Linux and Windows:** every frame-writing client verifies the serving process before its first frame: `viola_state::liveness::live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }` from its one `read_snapshot` (the liveness-only pre-check first), the client reads the connected stream's peer pid, and `verify_server` (sysinfo start time through `process_start_time`) yields the closed `ServerVerdict` (`Matches` · `Mismatch` · `Unverifiable`); the six CLI verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) connect through `src/cmd/client.rs`'s `connect` inside their dispatch span, a `Mismatch` or `Unverifiable` exiting 21 with the unchanged `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}` or, in human mode, the `server-verify` hint last on stderr, and writing `process-exit{exit_code:21, detail:\"server-verify-failed\", during:\"connect\"}`; `viola hook`'s event and dialog paths share one `connect` (`read_snapshot` → `connect_by` → `verify_server`), a failure exiting 0 with no body and `hook-decision{detail:\"server-verify-failed\"}`, a SessionEnd taking `append_session_end`; `viola mcp`'s four frame tools verify after `TokioClient::connect` (`server_verified`), a failure `instance-unreachable` with no frame and no line. No pid, start time, path or endpoint reaches any output or line. Proven by `tests/security_negatives_server_identity.rs` (compiled out on macOS), `hook_fail_open`'s impostor rows and `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`.»; and extend the Stack reference at line 28 with «`viola-state` liveness (`verify_server`, `ServerVerdict`); interprocess 2.4.4 `StreamCommon::peer_creds`»."
    sidecar: "authentication-authorization: the built client-side server verification recorded — `LiveEndpoint` + peer pid + `verify_server` / `ServerVerdict`, the six CLI verbs via `client::connect`, the hook's shared `connect`, MCP `server_verified`; outcomes and tests."
    rationale: "Report Symbols / APIs (`Client::server_pid`, `LiveEndpoint`, `ServerVerdict`, `server_verdict`, `verify_server`, `client::connect`, `Unreachable::ServerUnverified`, hook `NotSent` + shared `connect`, MCP `server_verified`), Schema / config (`process-exit` / `hook-decision` `server-verify-failed`, no pid/start/path/endpoint, MCP no line), Coverage of new surfaces and Outcome; the file states the requirement but holds no as-built record of it."
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:12
note: "Body sites outside this file the same change reaches (not proposed here): security-plan.md:100 (`PipeStream::server_process_id()` in the Library row, pattern `peerpid`), :114 (server-verification row, dated gaps) and :439 (anti-pattern gap list); this file's strict-modes gaps at :35-:37 stay true (strict-modes half stands), and :37 (1873 chars) takes no `LiveEndpoint` qualifier because the D-new-detail line carries it within the 2 000-char entity limit; D-security-input and D-security-deps found no drift (no new unvalidated input surface, no dependency added)."
