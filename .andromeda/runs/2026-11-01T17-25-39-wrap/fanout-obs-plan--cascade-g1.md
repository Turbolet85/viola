proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 37
    section: '## 1. Obs Scope Summary'
    change: 'Read `viola_core::UnreachableCause` as the closed set of exit 21''s six cause words (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`), with no serde path; the rest of the item stands.'
    sidecar: 'obs-scope-summary, viola-core entity: `UnreachableCause` five cause words to six, `socket-dir-refused` added.'
    rationale: 'Exit 21''s closed causes move five to six, a row counting five changes and its list extends (report.md:10, report.md:28).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 124
    section: '## 1. Obs Scope Summary'
    change: 'Read the call sites as: bind, connect, lock, rename, kill-fallback use, and the server check''s peer-euid (Unix), peer-pid and start-time reads, whose failed check the CLI and the hook log as `server-verify-failed` and MCP logs by no line; after the strict-modes sentence add: the socket directory''s check (`check_socket_dir`: viola''s stat, off Unix no read) is such a call site, its refusal written under `socket-dir-refused` (MCP by no line), and the wrapper''s read of a client''s euid in `accept_loop` writes no line.'
    sidecar: 'obs-scope-summary, OS APIs entity: the peer-euid read and the socket directory''s stat named among viola''s call sites.'
    rationale: 'The chunk adds `peer_euid_of` on both sides and `check_socket_dir` at the start, client seams and hook, all OS call sites this row enumerates (report.md:11, report.md:12, report.md:13, report.md:70, report.md:74, report.md:75).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 166
    section: '## 1. Obs Scope Summary'
    change: 'Replace the note''s last sentence with: A refused socket directory has its own code, `socket-dir-refused`, and writes its line; macOS compares no peer pid, the named carve-out from this rule, owned by no route entry, and there `server-verify-failed` means an euid mismatch alone (the Founder Direction 4 trigger below states them).'
    sidecar: 'obs-scope-summary, cli surface notes: the socket-directory carve-out and the route-entry owner retired; macOS''s pid gap stays named, unowned.'
    rationale: 'The socket-directory check and code owed to the route entry are built, and macOS''s pid gap alone stays named with no route entry owning it (report.md:26, report.md:31, report.md:33, report.md:68).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 216
    section: '## 1. Obs Scope Summary'
    change: 'Extend `instance-unreachable`''s closed causes in the note to `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`; the rest of the note stands.'
    sidecar: 'obs-scope-summary, mcp surface notes: `socket-dir-refused` added to `instance-unreachable`''s closed causes.'
    rationale: 'A row listing the five extends, and MCP''s `frame` returns the new cause through `result::unreachable(UnreachableCause::SocketDir)` (report.md:17, report.md:28).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 303
    section: '## 1. Obs Scope Summary'
    change: 'After the exit-1 causes'' `.cmd`/`.bat` child add: a refused socket directory, `socket-dir-refused`, on `run` and on a start made by `viola revive`, written as one `process-exit{subject:"self", exit_code:1}` before any snapshot, child or bind; the rest of the row stands.'
    sidecar: 'obs-scope-summary, `viola run` start path: `socket-dir-refused` added to the exit-1 `detail` codes.'
    rationale: '`start` calls `refused_with(launch.json, "socket-dir-refused", …)` before any snapshot, child or `Server::bind`, `revive`''s start shares it, and it carries no path, pid or uid (report.md:14, report.md:70).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 321
    section: '## 1. Obs Scope Summary'
    change: 'Extend the `exit_code` field to read: 21 when the wrapper is killed mid-wait (`detail` `endpoint-lost`, `during:"call"`) or when a CLI `wait` meets a refused socket directory before any frame (`detail` `socket-dir-refused`, `during:"connect"`, no `channel-request`).'
    sidecar: 'obs-scope-summary, `wait` / `last` path: the socket-directory exit 21 added to the `exit_code` field.'
    rationale: '`wait` is one of `connect`''s 8 callers, and the same check at the eight CLI frame verbs writes that `process-exit` at WARN with no `channel-request` (report.md:15, report.md:71).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 330
    section: '## 1. Obs Scope Summary'
    change: 'Read the `detail` list as: `server-verify-failed` on Linux, Windows and macOS (on macOS an euid mismatch alone) and `socket-dir-refused`, each exit 21 on CLI `answer` with `during:"connect"` and fail-open on the dialog hook with `corr` null and `decision_emitted:false`, with no pid, start time, path or endpoint; `unknown-dialog`, `unverified-cli` and the refused instance check''s sentence stand.'
    sidecar: 'obs-scope-summary, dialog path: `server-verify-failed` reach widened to macOS, `socket-dir-refused` added on `answer` and the dialog hook.'
    rationale: '`server-verify-failed`''s reach moves to macOS, where it means an euid mismatch alone, and the new word takes both forms at `answer` and the dialog hook (report.md:15, report.md:16, report.md:31, report.md:71, report.md:72).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 373
    section: '## 1. Obs Scope Summary'
    change: 'Add a refused socket directory to the fail-open paths logged to `hook-<name>.ndjson`, and after the strict-modes sentence add: a refused socket directory, though read by the same verdict, is no such silent path: it writes its line under its own word.'
    sidecar: 'obs-scope-summary, Vector 4 trigger: the refused socket directory added to the logged fail-open paths, set apart from the lineless strict-modes failure.'
    rationale: 'A hook meeting a refused socket directory exits 0 with empty streams and no frame, and writes one `hook-decision` at WARN (report.md:11, report.md:16, report.md:72).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 374
    section: '## 1. Obs Scope Summary'
    change: 'Read: A server-verify mismatch (Linux, Windows and macOS, on macOS an euid mismatch alone) is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` — and no pid, start time, path or endpoint in the line; a refused socket directory takes the same form with `detail` `socket-dir-refused`; macOS compares no peer pid (the named carve-out).'
    sidecar: 'obs-scope-summary, Vector 4 trigger: the hook''s server-verify line reaches macOS by euid; `socket-dir-refused` takes the same form.'
    rationale: 'A refused euid is `NotSent::ServerUnverified` at the hook, the word now reaches macOS, a row stating macOS compares no peer pid stays true, and the new word writes the same line form (report.md:16, report.md:31, report.md:33, report.md:72).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 382
    section: '## 1. Obs Scope Summary'
    change: 'End the no-line list with: one whose socket directory is refused (`check_endpoint_dir`, after `live_endpoint`) returns `socket-dir-refused`, and one whose server check fails or whose server''s euid is refused returns `server-verify-failed`, each likewise with no frame.'
    sidecar: 'obs-scope-summary, Vector 5 trigger: the socket-directory and euid refusals added to the rejected frame tools that write no line.'
    rationale: 'MCP''s `frame` checks the directory after `live_endpoint` and refuses an euid as `UnreachableCause::ServerVerify`, no frame and no line, `isError` the record (report.md:17, report.md:73).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 400
    section: '## 1. Obs Scope Summary'
    change: 'Add a bullet after this one:   - A refused socket directory (`check_socket_dir`, which shares the read verdict) is outside that count, writing under its own word `socket-dir-refused`: one `process-exit` on a start (exit 1) and on a CLI frame verb (exit 21, `during:"connect"`), one `hook-decision` on `viola hook` (exit 0), and no line on MCP''s `frame`. `strict-modes-failed`''s own line sites do not move.'
    sidecar: 'obs-scope-summary, Vector 7 trigger: the socket-directory refusal set apart from the lineless strict-modes refusals.'
    rationale: '`socket_dir_verdict` ends in `unix_read_verdict`, yet the refusal writes lines under its own word at each surface but MCP (report.md:11, report.md:70, report.md:71, report.md:72, report.md:73).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 215
    section: '## 1. Obs Scope Summary'
    change: 'End the signal with: a refused socket directory, read by the same verdict, is no strict-modes failure: it writes under its own word `socket-dir-refused`.'
    sidecar: 'obs-plan body §1, Vector 7 trigger: the socket-directory refusal distinguished from the strict-modes line sites.'
    rationale: 'The broad line #38.1 details limits where a strict-modes failure has a line, and the refusal sharing that verdict writes under another word, so the line needs the same distinction (report.md:11, report.md:70, report.md:71, report.md:72).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 460
    section: '## 1. Obs Scope Summary'
    change: 'Read: the server check''s reads: the peer pid, Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so that verdict is `Matches` with no comparison: the named carve-out, owned by no route entry); the peer euid, Unix `peer_creds` (a mismatch reads `server-verify-failed`, on macOS its one meaning) vs Windows none (`PEER_EUID_REPORTED` false, the DACL and the pid comparison the controls).'
    sidecar: 'obs-scope-summary, multi-OS trigger: the peer-euid read''s platform split added; macOS writes `server-verify-failed` on an euid mismatch; the discharged owner removed from the pid gap.'
    rationale: 'A client meeting another euid reads `server-verify-failed` on macOS as well, the pid gap stays named with no owner, and Windows reads no euid (report.md:12, report.md:31, report.md:33, report.md:93).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 491
    section: '## 1. Obs Scope Summary'
    change: 'Add `socket-dir-refused` to both lists: exit 21 (`instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`) and exit 1 (`already-live`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `socket-dir-refused`, and `viola revive`''s `no-session`, `session-live` and `cwd-missing`); the sentences on `viola revive` and `viola link` / `viola unlink` stand.'
    sidecar: 'obs-scope-summary, Founder Direction 4 trigger: `socket-dir-refused` added to exit 21''s and exit 1''s distinct codes.'
    rationale: 'The new word is written at exit 1 on a start and at exit 21 on a frame verb, and the instance-check exception does not cover it (report.md:28, report.md:70, report.md:71).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 492
    section: '## 1. Obs Scope Summary'
    change: 'Read the reach as: written on Linux, Windows and macOS (on macOS an euid mismatch alone) as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}`, verbs and files as they stand, and end: with no pid, start time, path or endpoint; macOS compares no peer pid (the named carve-out).'
    sidecar: 'obs-scope-summary, Founder Direction 4 trigger: the CLI server-verify line reaches macOS by euid.'
    rationale: 'A refused euid is `CallError::ServerUnverified` at `connect`, so `server-verify-failed` is no longer written off macOS alone (report.md:15, report.md:31, report.md:33, report.md:74).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 493
    section: '## 1. Obs Scope Summary'
    change: 'Replace the bullet and its OPEN with: The socket-directory cause has its code, `socket-dir-refused`, whatever failed: a refused check, a failed create or a lost race reads that one word. One carve-out stands outside the distinct-code rule: macOS compares no peer pid, named and owned by no route entry.'
    sidecar: 'obs-scope-summary, Founder Direction 4 trigger: the OPEN on the socket-directory code closed, built; the route-entry owner removed; macOS''s pid gap stays named, unowned.'
    rationale: 'The Retired bullet names the socket-directory check and code owed to the route entry, and only macOS''s pid gap stays, owned by no route entry (report.md:26, report.md:33, report.md:68, report.md:95).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
note: 'D-obs-instrumentation, D-obs-stack and D-obs-pii read no drift: the report records each new surface''s line or its no-line rule, adds no dependency and holds no path, pid or uid in the new lines (report.md:23, report.md:70-76); F1-F10 belong to other categories, none placed here; this detector read no draw''s trail, so the pick trail stays the orchestrator''s to report.'
