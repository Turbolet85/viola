proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 91
    section: "## Occupied Resources"
    change: "In the `cli-<name>.ndjson` producers clause, after «exit 21 with `detail:\"instance-dead\"` + `during` at WARN», add: «or, on Linux and Windows, when the process serving the recorded endpoint is not the recorded wrapper or the check cannot be made, exit 21 with `detail:\"server-verify-failed\"` and `during:\"connect\"` at WARN, with no client `channel-request` / `channel-response` line (no pid, start time, path or endpoint on the line; chunk 2026-10-26-server-verification-before-any-frame)»."
    sidecar: "occupied-resources.md: cli-<name>.ndjson's exit-21 process-exit gains detail server-verify-failed, during connect, written by the six CLI frame verbs since chunk 2026-10-26-server-verification-before-any-frame."
    rationale: "Report Schema / config: this build is the first that WRITES `process-exit{subject:\"self\", exit_code:21, detail:\"server-verify-failed\", during:\"connect\"}` at WARN into `cli-<name>.ndjson`, no pid / start time / path / endpoint; Expected amendments names architecture §Occupied Resources for it; Symbols: `Unreachable::ServerUnverified` (`detail()` `server-verify-failed`, `during()` always `connect`); the line lists only `instance-dead` for exit 21."
    basis: src/cmd/client.rs:75
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 93
    section: "## Occupied Resources"
    change: "After «`hook-<name>.ndjson`: `hook-invoked`, `hook-decision`, `parse-rejected`» add: «; on Linux and Windows a hook whose serving process fails the server check sends no frame and writes `hook-decision{detail:\"server-verify-failed\"}` at WARN (the dialog path: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:\"server-verify-failed\"}`), a SessionEnd taking the direct append; `viola mcp` writes no line for that mismatch (chunk 2026-10-26-server-verification-before-any-frame)»."
    sidecar: "occupied-resources.md: hook-<name>.ndjson's hook-decision writes detail server-verify-failed on a server-check mismatch; mcp.ndjson writes no line for it."
    rationale: "Report Schema / config: `hook-decision{detail:\"server-verify-failed\"}` at WARN into `hook-<name>.ndjson` (dialog path `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:\"server-verify-failed\"}`), MCP writes no line for the mismatch; Symbols (`src/cmd/hook.rs` `NotSent`, a SessionEnd whose check fails takes `append_session_end`); Expected amendments names architecture §Occupied Resources for the hook-decision value."
    basis: src/cmd/hook.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: "## Occupied Resources"
    change: "After «…may run with a different environment from the wrapper.» add: «Before its first frame each of them but `run`'s collision check (`hook`'s event and dialog paths, the CLI frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release`, and `mcp`'s four frame tools) checks the connected stream's peer pid (`viola_channel::Client::server_pid` / `TokioClient::server_pid`, `peer_creds().pid()`) and that process's OS start time against the snapshot's `pid` and `started_at` (`viola_state::liveness::verify_server`, the closed `ServerVerdict`); a mismatch or an unmakeable check sends no frame (CLI exit 21 `instance-unreachable`, MCP `instance-unreachable`, the hook failing open). macOS reports no peer pid (`SERVER_PID_REPORTED` false) and stays on the recorded gap (chunk 2026-10-26-server-verification-before-any-frame).»"
    sidecar: "occupied-resources.md: the IPC endpoint row records that hook, the CLI frame verbs and mcp verify the serving process (peer pid + start time against the snapshot) before the first frame; macOS on the recorded gap."
    rationale: "Report Symbols / APIs (`Client::server_pid`, `TokioClient::server_pid`, `SERVER_PID_REPORTED`, `ServerVerdict`, `verify_server`, `cmd::client::connect`, hook `connect`, mcp `server_verified`) and Retired (the server-identity half of the dated gaps closed on Linux and Windows, macOS on the recorded gap); the row names who connects to the recorded endpoint but not the check now made before the first frame; the collision check verifies no server (report Spec claims disproved, the `gaps` row)."
    basis: crates/viola-state/src/liveness.rs:113
note: "occupied-resources.md:8 says `run`'s collision check «connects to the recorded `endpoint`», while the report quotes the plan's graph impact for `src/cmd/run.rs:379` as «no change: no frame, no connection» — whether the collision check connects at all is not settled by the report; its disposition rides with security-plan authentication-authorization.md:27 (owed at Validate). No D-arch-decisions drift: no dependency added, `spawn_blocking` stays inside viola-mcp."
