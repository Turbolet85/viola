proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 28
    section: '## 6. Log Coverage'
    change: 'New continuation line of the exit-1 bullet, following this line: And `socket-dir-refused`: a socket directory `viola run` or `viola revive`''s start refuses (`socket_dir_ready`: `check_endpoint_dir`, the parent created only when absent, then checked again; a failed create and a refused check read this one code) ends with one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` in `run-<name>.ndjson`, a step with no span, before any snapshot, child or `Server::bind`, with no path, pid or uid; the check never chmods, so a refused directory stays as found, and a `Server::bind` failure stays what it was (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    sidecar: 'log-coverage §6: `socket-dir-refused` added to `process-exit` exit 1, from `viola run` and `viola revive`''s start (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    rationale: 'report.md:14 and report.md:70 add the word on the start chain, one line and no span; report.md:95 records one code for a failed create and a refused check; the exit-1 words of this bullet do not hold it.'
    basis: src/cmd/run.rs:822
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 29
    section: '## 6. Log Coverage'
    change: 'Replace «closes the `process-exit` `detail` at 19 values (the enum''s members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-10-31-exit-cause-code-catalogue)» with «closes the `process-exit` `detail` at 20 values (the enum''s members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening)».'
    sidecar: 'log-coverage §6: `process-exit` `detail` enum 19 → 20 values, `socket-dir-refused` the new member.'
    rationale: 'report.md:24 and report.md:30 move the enum 19 → 20 and name this line as the standing row of pattern `detail-enums`.'
    basis: schemas/diag-line.v1.json:78
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 31
    section: '## 6. Log Coverage'
    change: '- `process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`.'
    sidecar: 'log-coverage §6: exit 21''s list of closed causes extended with `socket-dir-refused`, the sixth.'
    rationale: 'report.md:28 moves exit 21''s closed causes five → six; a row listing the five extends.'
    basis: crates/viola-core/src/lib.rs:201
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 32
    section: '## 6. Log Coverage'
    change: 'Replace «or a check that cannot be made (Linux and Windows):» with «or a check that cannot be made (Linux and Windows), and on a server of another euid (Linux and macOS; on macOS an euid mismatch alone, macOS comparing no peer pid):»; the rest of the line stands.'
    sidecar: 'log-coverage §6: the frame verbs'' `server-verify-failed` reaches Linux, Windows and macOS, on macOS an euid mismatch alone.'
    rationale: 'report.md:15 reads a refused euid as `CallError::ServerUnverified`, so `server-verify-failed`; report.md:31 widens the word''s reach, pattern `verify-word`, each row changes; report.md:93 holds the founder''s Q2.'
    basis: src/cmd/client.rs:177
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 32
    section: '## 6. Log Coverage'
    change: 'New continuation line of the exit-21 bullet, following this line: `socket-dir-refused` is written by the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) when `connect`''s `check_endpoint_dir` refuses the recorded endpoint''s directory, the check made before `Client::connect`, the euid read and `verify_server`: `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN into `cli-<name>.ndjson` (`link` / `unlink`: the driven instance''s), with no `channel-request` line, no frame and no path, pid or uid.'
    sidecar: 'log-coverage §6: `socket-dir-refused` on the eight CLI frame verbs, its `process-exit` line at WARN, no `channel-request`.'
    rationale: 'report.md:15 places the check and its order in `connect`; report.md:71 states the line and its absences.'
    basis: src/cmd/client.rs:173
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 34
    section: '## 6. Log Coverage'
    change: 'Replace «`server-verify-failed`, `strict-modes-failed`, `unverified-cli`, `deadline`.» with «`server-verify-failed`, `socket-dir-refused`, `strict-modes-failed`, `unverified-cli`, `deadline`.»'
    sidecar: 'log-coverage §6: `hook-decision`''s fail-open codes extended with `socket-dir-refused`, 7 → 8 members.'
    rationale: 'report.md:16 gives the hook''s `detail` `socket-dir-refused`; report.md:24 and report.md:30 move the `hook-decision` enum 7 → 8.'
    basis: schemas/diag-line.v1.json:53
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 34
    section: '## 6. Log Coverage'
    change: 'Replace «`viola hook statusline` uses three of the seven» with «`viola hook statusline` uses three of the eight»; «four of the seven», the record of the arm before chunk 2026-10-10-budget-governor, stays.'
    sidecar: 'log-coverage §6: the statusline arm''s three codes now read of the eight.'
    rationale: 'report.md:30 moves the enum 7 → 8; report.md:16 adds the code at the hook''s `connect`, on the event and dialog paths, so the statusline arm''s three do not move.'
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 35
    section: '## 6. Log Coverage'
    change: 'Replace «or a check that cannot be made (Linux and Windows):» with «or a check that cannot be made (Linux and Windows), and on a server of another euid (Linux and macOS; on macOS an euid mismatch alone, macOS comparing no peer pid):»; the rest of the line stands.'
    sidecar: 'log-coverage §6: the hook''s `server-verify-failed` reaches Linux, Windows and macOS, on macOS an euid mismatch alone.'
    rationale: 'report.md:16 reads a refused euid as `NotSent::ServerUnverified`; report.md:31 widens the word''s reach, pattern `verify-word`, each row changes.'
    basis: src/cmd/hook.rs:236
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 35
    section: '## 6. Log Coverage'
    change: 'New continuation line of the `hook-decision` bullet, following this line: `socket-dir-refused` is written by `viola hook` when `connect`''s `check_endpoint_dir` refuses the recorded endpoint''s directory (`NotSent::SocketDir`, `Asked::SocketDir`): one `hook-decision{detail:"socket-dir-refused"}` at WARN into `hook-<name>.ndjson`, on the dialog path `hook-invoked{corr:null}` first and `decision_emitted:false`; exit 0, empty streams, no frame, no path, pid or uid.'
    sidecar: 'log-coverage §6: `socket-dir-refused` on `viola hook`, one `hook-decision` at WARN, the hook failing open.'
    rationale: 'report.md:16 and report.md:72 state the variants, the line and the dialog path''s order; report.md:59 retires the plan''s «note in `diagnostics/`» for this one `hook-decision`.'
    basis: src/cmd/hook.rs:231
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 41
    section: '## 6. Log Coverage'
    change: 'New bullet, following this line: - `viola mcp`''s frame tools on a refused socket directory (`check_endpoint_dir` after `live_endpoint` in `frame`) return `instance-unreachable` with `detail` `socket-dir-refused` and write no line and no frame, the `isError` result the only record.'
    sidecar: 'log-coverage §6: MCP''s frame tools on a refused socket directory write no line, `isError` the record.'
    rationale: 'report.md:17 and report.md:73 state no frame and no line by rule; report.md:111 holds the word on MCP.'
    basis: crates/viola-mcp/src/tools.rs:345
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 41
    section: '## 6. Log Coverage'
    change: 'New bullet, following the socket-directory bullet: - A peer the wrapper refuses by euid in `accept_loop` writes no line: the stream is dropped on the accept thread, before its worker thread and before any frame, so there is no frame to key a line on and `channel-response`''s closed `detail` gains no word; the peer reads end-of-file, and nothing reaches the terminal.'
    sidecar: 'log-coverage §6: the wrapper''s drop of a refused peer writes no line.'
    rationale: 'report.md:13 and report.md:75 state the drop, its place before any frame and instrumentation n/a; report.md:95 records the lean that the drop writes no line.'
    basis: crates/viola-channel/src/server.rs:173
note: 'log-coverage.md:40 was read against `board-rule` (report.md:32): it states the scan''s counts and no rule on the home, no change; the master body''s rows the chunk moves (obs-plan.md:311 census, §4''s `wait` / `last` scenario and §7''s MCP line on server verification) and `.claude/rules/observability.md:28` «19 values» lie outside this file.'
