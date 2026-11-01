proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 36
    section: '## 5. Metric Coverage'
    change: 'Replace the second sentence with: «The CLI''s server-verification refusal (Linux, Windows and macOS, where it means an euid mismatch alone) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, and its socket-directory refusal one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN there, each counted by the Counter row''s `detail` grouping; MCP''s `wait` tool writes no line for either.»'
    sidecar: 'metric-coverage: a `wait` refused before any frame now reads server-verify-failed on macOS too, and socket-dir-refused beside it.'
    rationale: 'A refused euid reads server-verify-failed on Linux, Windows and macOS (report.md:31); the frame verbs'' directory refusal is one process-exit line at WARN with no channel-request (report.md:71), and MCP writes no line (report.md:73).'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 26
    section: '## 6. Log Coverage'
    change: 'In the exit-1 word list, after `internal-error`, add: «`socket-dir-refused` (the socket directory of `viola run` or `viola revive`''s start refused, before any snapshot, child or bind),».'
    sidecar: 'log-coverage: exit 1''s word list gains socket-dir-refused.'
    rationale: '`start` calls refused_with with the word, `revive`''s start sharing it (report.md:14), as one process-exit with subject self and exit_code 1 (report.md:70); the row''s list is what the enum count at line 29 counts.'
    basis: 'src/cmd/run.rs:822'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 29
    section: '## 6. Log Coverage'
    change: 'Read: «`schemas/diag-line.v1.json` closes the `process-exit` `detail` at 20 values (the enum''s members: the words of the exit-1, exit-21 and exit-20 lists here, as counted at chunk 2026-11-01-unix-endpoint-and-home-hardening).»'
    sidecar: 'log-coverage: process-exit''s detail enum 19 to 20.'
    rationale: 'The enum moves 19 to 20 members, counted from its members in the diff (report.md:24, report.md:30).'
    basis: 'schemas/diag-line.v1.json:78'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 31
    section: '## 6. Log Coverage'
    change: 'Read: «- `process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`.»'
    sidecar: 'log-coverage: exit 21''s list gains socket-dir-refused.'
    rationale: 'Exit 21''s closed causes move five to six; a row listing the five extends (report.md:28).'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 32
    section: '## 6. Log Coverage'
    change: 'Replace «on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows):» with «on a server of another euid, on a server that is not the recorded wrapper, or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone):».'
    sidecar: 'log-coverage: the CLI''s server-verify-failed now reaches macOS, as an euid mismatch alone.'
    rationale: 'A refused euid is CallError::ServerUnverified, reading server-verify-failed (report.md:15); the word''s reach moves, each row changing (report.md:31).'
    basis: 'src/cmd/client.rs:177'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 34
    section: '## 6. Log Coverage'
    change: 'Add `socket-dir-refused` to the fail-open codes after `server-verify-failed`, read «`viola hook statusline` uses three of the eight» in place of «three of the seven», and read the history clause as «four of the seven then».'
    sidecar: 'log-coverage: hook-decision''s fail-open codes 7 to 8, the statusline arm''s three unmoved.'
    rationale: 'hook-decision''s enum moves 7 to 8 (report.md:24, report.md:30), written by the hook''s `connect` (report.md:16); the report names no new statusline path, so the arm''s three stand.'
    basis: 'schemas/diag-line.v1.json:53'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 35
    section: '## 6. Log Coverage'
    change: 'Replace «on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows):» with «on a server of another euid, on a server that is not the recorded wrapper, or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone):».'
    sidecar: 'log-coverage: the hook''s server-verify-failed now reaches macOS, as an euid mismatch alone.'
    rationale: 'A refused euid is NotSent::ServerUnverified at the hook (report.md:16); the word''s reach moves, each row changing (report.md:31).'
    basis: 'src/cmd/hook.rs:236'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 41
    section: '## 6. Log Coverage'
    change: 'Add after this line, in the entity''s continuation form: « `socket-dir-refused` is a closed `detail` code on `process-exit` and on `hook-decision`, meaning the socket directory of a start or a recorded endpoint is refused: a symlink, not a directory, another owner, or a foreign mode bit. It shares the read verdict of the strict-modes checks above yet writes lines under its own word: `viola run` and `viola revive`''s start one `process-exit{subject:"self", exit_code:1}`, the eight CLI frame verbs one `process-exit{exit_code:21, during:"connect"}` at WARN, `viola hook` one `hook-decision` at WARN; only `viola mcp`''s `frame` writes no line, `isError` its record.»'
    sidecar: 'log-coverage: new closed detail code socket-dir-refused, on process-exit and hook-decision, distinct from the lineless strict-modes refusals.'
    rationale: 'The word and its forms stand at report.md:11 and report.md:70-73; the entity reads refused checks as lineless, so the new word needs the distinction.'
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 22
    section: '## 7. Error Capture & Reporting'
    change: 'Read «prints the exit-1 document in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`», the rest of the line as it stands.'
    sidecar: 'error-capture-reporting: refused_with''s start refusals under --json gain socket-dir-refused.'
    rationale: '`start` calls refused_with with socket-dir-refused before any snapshot, child or Server::bind, `revive`''s start sharing it (report.md:14).'
    basis: 'src/cmd/run.rs:822'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 44
    section: '## 7. Error Capture & Reporting'
    change: 'Add after this line: «- MCP rejects on a refused socket directory: `frame` (`crates/viola-mcp/src/tools.rs`) checks the recorded endpoint''s directory after `live_endpoint` and returns `instance-unreachable` through `UnreachableCause::SocketDir`, with no frame and no line, the `isError` result its only record; `UnreachableCause::SocketDir` has no serde path and no `RefusalReason`.»'
    sidecar: 'error-capture-reporting: MCP''s frame refuses a refused socket directory as instance-unreachable, with no frame and no line.'
    rationale: 'report.md:10 and report.md:17 state the check, its place after live_endpoint, and the absent serde path and RefusalReason; report.md:73 states no line by rule.'
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 45
    section: '## 7. Error Capture & Reporting'
    change: 'Read «in `frame` a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`''s reading, a refused `check_endpoint_dir` is `SocketDir`, and a refused `peer_admitted` or a failed `server_verified` is `ServerVerify`;», the rest of the line as it stands.'
    sidecar: 'error-capture-reporting: frame''s cause mapping gains SocketDir, and a refused euid reads ServerVerify.'
    rationale: '`frame` maps check_endpoint_dir to SocketDir and `!peer_admitted(...) || !server_verified(...)` to ServerVerify (report.md:17).'
    basis: 'crates/viola-mcp/src/tools.rs:352'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 55
    section: '## 7. Error Capture & Reporting'
    change: 'Add after this line: «- `Refused::NotADirectory` («a trusted state path is not a directory») is a new fixed-message variant of `Refused`, naming no path; a variant, not a derive, so the count of 9 stands.»'
    sidecar: 'error-capture-reporting: Refused gains the fixed-message variant NotADirectory; derives stay 9.'
    rationale: 'report.md:11 states the variant and its message; report.md:36 states the derives stay 9.'
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 57
    section: '## 7. Error Capture & Reporting'
    change: 'Read «`detail` is one of the six closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`.», the rest of the line as it stands.'
    sidecar: 'error-capture-reporting: exit 21''s closed words five to six.'
    rationale: 'UnreachableCause::SocketDir is the sixth cause, with no serde path (report.md:10), and the row counting five changes (report.md:28).'
    basis: 'crates/viola-core/src/lib.rs:190'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 537
    section: '## 7. Error Capture & Reporting'
    change: 'Read «and `instance-unreachable` — a frame tool''s refused instance check (made before the liveness pre-check), the liveness pre-check''s, a frame tool''s refused socket directory (made after it), or a frame tool''s refused euid or server-verification mismatch (Linux, Windows and macOS, where it means an euid mismatch alone) — is logged by no line», the rest of the line as it stands.'
    sidecar: 'obs-plan section 7: MCP''s no-line instance-unreachable list gains the socket directory and a refused euid; the mismatch now reaches macOS.'
    rationale: 'This is the broad line entities 5.1 and 5.2 detail; `frame` adds both refusals with no frame and no line (report.md:17, report.md:73), and the word''s reach moves (report.md:31).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'Add after this line, at its indent: «- A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed `ViolaName`: `tests/security_negatives_socket_dir.rs` reads them through `assert_names_nothing` and `digit_runs`, on `ubuntu-latest` and `macos-latest`; that binary is `cfg(unix)` and has no Windows case.»'
    sidecar: 'pii-scrubbing-compliance: the socket-directory refusal''s no path, pid or uid, and its witnesses on two OSes.'
    rationale: 'report.md:70-72 and report.md:116 state the PII reading and its witnesses, report.md:76 the pair naming only the name, and report.md:128 the OS legs and `#![cfg(unix)]`.'
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 42
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'Read «the four live witnesses» in place of «the three live witnesses».'
    sidecar: 'pii-scrubbing-compliance: the clean-error-body live witnesses three to four.'
    rationale: 'The chunk adds a fourth live case calling assert_error_body_clean (report.md, New text, by line: tests/cli_error_bodies.rs, 274-371 and 360-365).'
    basis: 'tests/cli_error_bodies.rs:360'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 43
    section: '## 8. PII Scrubbing & Compliance › ### PII Scrubbing'
    change: 'Read «`cli_error_documents_hold_no_path_or_upstream_text` and `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`)» in place of the one case, the rest of the line as it stands.'
    sidecar: 'pii-scrubbing-compliance: the loosened-directory case joins the live witnesses of clean error bodies.'
    rationale: 'The case stands at tests/cli_error_bodies.rs:278, in the report''s New text section, and calls assert_error_body_clean at 360-365.'
    basis: 'tests/cli_error_bodies.rs:278'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 29
    section: '## 10. SLO Invariants & Telemetry Budgets'
    change: 'Replace the clause from «the timed path holds the server check» through «exit 0)» with: «the timed path holds the checks a frame-writing hook makes before its first frame: the socket-directory check on the recorded endpoint''s parent, the peer euid decision on Unix, then the server check (the connected endpoint''s peer pid and its OS start time against the one `snapshot.json` read; macOS the recorded gap, no pid comparison), bounded by the same gate, and a failed check fails open (`hook-decision{detail:"socket-dir-refused"}` on the directory, `hook-decision{detail:"server-verify-failed"}` on the euid or the server, exit 0)»; every value and the «max 2.4–3.3 ms» band stay as they stand.'
    sidecar: 'slo-invariants-telemetry-budgets: the hook''s timed path now holds the socket-directory check and the euid decision; no value moved.'
    rationale: 'Both checks sit in the timed path and the band stays a true record, this run at or under it (report.md:38); the checks and their order stand at report.md:16, and macOS''s no-pid wording stays a true claim (report.md:33).'
    basis: 'src/cmd/hook.rs:231'
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-anti-patterns.md
    near: 8
    section: '## 11. Obs Anti-Patterns (NEVER do these) › ### Logs'
    change: 'Read «callers `run` 8 · `verify` 6 · `list` 2 · `revive` 1 · `plugin install` 1 · `client` 1» in place of «`run` 7», the rest of the line as it stands.'
    sidecar: 'obs-anti-patterns: the human::refuse( census moves run from 7 to 8.'
    rationale: '`refuse_socket_dir` adds the site; the census moves 18 to 19, `run` 7 to 8 (report.md:14, report.md:34).'
    basis: 'src/cmd/run.rs:825'
note: 'obs-plan.md:311 holds the same `human::refuse(` census and start-refusal counts, but in section 3, which no group-4 category details, so it is not proposed here; report.md:30 also names .claude/rules/observability.md:28, outside these categories.'
