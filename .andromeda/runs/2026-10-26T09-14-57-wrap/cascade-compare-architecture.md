# Cascade compare — architecture · wrap · 2026-10-26-server-verification-before-any-frame
E items 11 · held 8 · lost at classify 1 · lost at strong 0 · beside a placed fact 1 · cascade only 5 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### occupied-resources#1.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: "## Occupied Resources"
    change: "After «…may run with a different environment from the wrapper.» add: «Before its first frame each of them but `run`'s collision check (`hook`'s event and dialog paths, the CLI frame verbs `send` / `wait` / `last` / `answer` / `pause` / `release`, and `mcp`'s four frame tools) checks the connected stream's peer pid (`viola_channel::Client::server_pid` / `TokioClient::server_pid`, `peer_creds().pid()`) and that process's OS start time against the snapshot's `pid` and `started_at` (`viola_state::liveness::verify_server`, the closed `ServerVerdict`); a mismatch or an unmakeable check sends no frame (CLI exit 21 `instance-unreachable`, MCP `instance-unreachable`, the hook failing open). macOS reports no peer pid (`SERVER_PID_REPORTED` false) and stays on the recorded gap (chunk 2026-10-26-server-verification-before-any-frame).»"
    sidecar: "occupied-resources.md: the IPC endpoint row records that hook, the CLI frame verbs and mcp verify the serving process (peer pid + start time against the snapshot) before the first frame; macOS on the recorded gap."
    rationale: "Report Symbols / APIs (`Client::server_pid`, `TokioClient::server_pid`, `SERVER_PID_REPORTED`, `ServerVerdict`, `verify_server`, `cmd::client::connect`, hook `connect`, mcp `server_verified`) and Retired (the server-identity half of the dated gaps closed on Linux and Windows, macOS on the recorded gap); the row names who connects to the recorded endpoint but not the check now made before the first frame; the collision check verifies no server (report Spec claims disproved, the `gaps` row)."
    basis: crates/viola-state/src/liveness.rs:113

### standard-contracts#4.1 · lost at classify · beside a placed fact F1 (near 47, standard-contracts#5.1) · .andromeda/architecture-detail/standard-contracts.md:41
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 41
    section: "## Standard Contracts › contract-instance-snapshot"
    change: "Add after line 41 one item: «- `pid` and `started_at` also identify the server. As landed (chunk 2026-10-26-server-verification-before-any-frame) every frame-writing client — CLI `send`, `wait`, `last`, `answer`, `pause` and `release` through `cmd::client::connect`, `viola hook`'s `hook.event` and `hook.dialog` through its one shared `connect`, and `viola mcp`'s four frame tools — takes both from its one snapshot read (`viola_state::liveness::LiveEndpoint {endpoint, pid, started_at}`), reads the connected stream's peer pid (`Client::server_pid` / `TokioClient::server_pid`, interprocess `peer_creds().pid()`) and that process's OS start time, and judges them through `viola_state::liveness::server_verdict` (`Matches` · `Mismatch` · `Unverifiable`) before its first frame. A mismatch or an unmakeable check writes no frame: CLI exit 21 with the unchanged `instance-unreachable` document, `viola hook` fails open (`server-verify-failed`; a SessionEnd takes its direct append), MCP `instance-unreachable`. macOS reports no peer pid (`SERVER_PID_REPORTED` false): its verdict is `Matches` with no comparison, the recorded gap owned by \"Unix endpoint and home hardening\". No method, field or `v` changes.»"
    sidecar: "standard-contracts: instance snapshot's pid + started_at now also verify the serving process before any frame (CLI six verbs, hook event/dialog, MCP frame tools); macOS recorded gap."
    rationale: "Report Changes › Symbols / APIs (liveness LiveEndpoint, ServerVerdict, server_verdict, verify_server; Client::server_pid; cmd::client::connect; hook.rs shared connect; tools.rs server_verified) and Retired (the server-identity half of the hook.event and fourth-to-eighth gaps closed on Linux and Windows, macOS on the recorded gap); IPC methods and fields unchanged."
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:12

## Held

- conventions#2.1
- conventions#3.1
- established-decisions#7.1
- established-decisions#9.1
- occupied-resources#13.1
- stack-and-technologies#1.17
- stack-and-technologies#1.6
- standard-contracts#2.0

## Cascade only

- established-decisions#10.1
- established-decisions#14.1
- established-decisions#17.1
- standard-contracts#5.1
- standard-contracts#7.1
