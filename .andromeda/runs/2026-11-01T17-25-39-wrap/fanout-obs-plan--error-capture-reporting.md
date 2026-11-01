proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 57
    section: '## 7. Error Capture & Reporting'
    change: 'In the exit-21 row, `detail` now reads: `detail` is one of the six closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`. The rest of the row stands.'
    sidecar: 'Exit 21 closed causes five to six in the error-capture row, the sixth `socket-dir-refused` (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    rationale: 'The report moves exit 21 causes five to six with the list words and adds `UnreachableCause::SocketDir` with no serde path (report.md:10, report.md:28); this row reads «five closed words» as standing.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 45
    section: '## 7. Error Capture & Reporting'
    change: 'The `frame` clause now reads: in `frame` a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`''s reading, a refused `check_endpoint_dir` after `live_endpoint` is `SocketDir`, and a refused `peer_admitted` or a failed `server_verified` is `ServerVerify`. The `of_board` and `of_reply` clauses stand.'
    sidecar: 'MCP `frame` refusal mapping gains `SocketDir` and the peer euid decision under `ServerVerify`.'
    rationale: 'The report adds both reads to `frame`: `check_endpoint_dir` to `result::unreachable(UnreachableCause::SocketDir)`, and `!peer_admitted(client.server_euid()) || !server_verified(…)` to `ServerVerify` (report.md:17); the row claims to map each refusal and names neither.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 22
    section: '## 7. Error Capture & Reporting'
    change: 'The word list now reads: for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`; add after it that the `socket-dir-refused` refusal comes before any snapshot, child or `Server::bind`, and that `revive`''s start shares it. The rest of the row stands.'
    sidecar: '`refused_with` start refusals under `--json` gain `socket-dir-refused`, shared by `run` and `revive`''s start.'
    rationale: 'The report has `start` call `refused_with(launch.json, "socket-dir-refused", …)` before any snapshot, child or `Server::bind`, with `revive`''s start sharing it (report.md:14) and `revive --json` exit-1 codes ten to eleven (report.md:29); this row enumerates what `refused_with` prints the exit-1 document for, so it is an edit of what that function does, beyond the count pattern''s no-change reading of a bare `batch-script-child` naming.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 55
    section: '## 7. Error Capture & Reporting'
    change: 'Append to the row: `Refused` gains the fixed-message variant `NotADirectory`, «a trusted state path is not a directory» (chunk 2026-11-01-unix-endpoint-and-home-hardening): a variant, not a derive, so the derives stay 9.'
    sidecar: '`Refused::NotADirectory` recorded as a new fixed-message variant; thiserror derives unmoved at 9.'
    rationale: 'The report names the new fixed-message variant and states the derives did not move (report.md:11, report.md:36); this row holds the derive census and the fixed messages of variants.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:36
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 57
    section: '## 7. Error Capture & Reporting'
    change: '- New row following the exit-21 row: A refused socket directory has one record per seam, each with no path, pid or uid, and the check never chmods, so the directory stays as found (chunk 2026-11-01-unix-endpoint-and-home-hardening). `viola run` and `revive`''s start: `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, before any snapshot, child or `Server::bind`; a failed create and a refused check read this one code, and a `Server::bind` failure stays what it was. The eight CLI frame verbs: `CallError::SocketDirRefused` to `Unreachable::SocketDir`, one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN and no `channel-request`. `viola hook`: `NotSent::SocketDir` and `Asked::SocketDir`, one `hook-decision{detail:"socket-dir-refused"}` at WARN, exit 0, empty streams, no frame. MCP `frame`: `isError` is the record, no frame and no line.'
    sidecar: 'New row: the `socket-dir-refused` record at `run`, the CLI frame verbs, `hook` and MCP.'
    rationale: 'The report brings a new closed-word error path with a distinct record on each role (report.md:14, report.md:15, report.md:16, report.md:17, report.md:70-73, report.md:95); the body names module-boundary errors ending in a logged `detail` and MCP rejects as `isError` only (.andromeda/obs-plan.md:533, .andromeda/obs-plan.md:537), and this file holds their detail.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 57
    section: '## 7. Error Capture & Reporting'
    change: '- New row following the socket-directory row: A peer of another euid is a failed server check on the client side and a silent drop on the wrapper side (chunk 2026-11-01-unix-endpoint-and-home-hardening). `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer it cannot read is refused where `PEER_EUID_REPORTED` holds (`cfg!(unix)`), and on Windows the DACL and the pid comparison stay the controls. Clients: `CallError::ServerUnverified`, `NotSent::ServerUnverified` and MCP `ServerVerify`, each reading `server-verify-failed`, on macOS meaning an euid mismatch alone. Wrapper: `accept_loop` drops a refused stream on the accept thread, before its worker thread and before any frame; the peer reads end-of-file, and nothing is written as a line or on the terminal, since there is no frame to key a line on and `channel-response`''s `detail` is closed.'
    sidecar: 'New row: how a refused peer euid is captured, `server-verify-failed` on the clients and no line at the wrapper drop.'
    rationale: 'The report adds the decision, its three client readings and the wrapper drop with no line (report.md:12, report.md:13, report.md:15, report.md:16, report.md:17, report.md:31, report.md:74, report.md:75); an error path deliberately without a line is a carve-out to the body module-boundary rule (.andromeda/obs-plan.md:533) and this file is where it is recorded.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:75
note: 'D-obs-instrumentation, D-obs-stack and D-obs-pii found no violation (report.md:23, report.md:70-76, no dependency moved, no path, pid or uid in a line); the body section of this category is outside my proposals and still reads stale at .andromeda/obs-plan.md:537, a server-verification mismatch «on Linux and Windows» with no socket-directory refusal named (report.md:17, report.md:31).'
