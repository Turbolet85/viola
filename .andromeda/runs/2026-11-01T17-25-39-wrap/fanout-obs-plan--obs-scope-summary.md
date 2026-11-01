proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 37
    section: '## 1. Obs Scope Summary'
    change: 'In the `viola_core::UnreachableCause` row, read «the closed set of exit 21''s six cause words (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`), with no serde path»; the rest of the row stands.'
    sidecar: 'Obs Scope Summary: `UnreachableCause` row, five cause words to six, the sixth `socket-dir-refused`.'
    rationale: 'Report **Symbols / APIs** names `UnreachableCause::SocketDir` as the sixth cause, and **Counts / qualifiers moved** moves exit 21 five to six (pattern `causes`).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 124
    section: '## 1. Obs Scope Summary'
    change: 'In the OS APIs row, add to the call sites the peer-euid read (`peer_euid_of`: `peer_creds` on Unix, none on Windows), a refused euid reading as the same failed server check on each client (the CLI and the hook log `server-verify-failed`, MCP no line), and the wrapper''s refusal of a peer euid writing no line; and add the socket-directory check (`check_socket_dir`, `check_endpoint_dir`: one `symlink_metadata`, never a chmod) as such a call site, which of its refusals write a line being the Founder Direction 4 trigger''s (below).'
    sidecar: 'Obs Scope Summary: OS APIs row gains the peer-euid read and the socket-directory check as call sites.'
    rationale: 'Report **Symbols / APIs** adds `check_socket_dir`, `check_endpoint_dir`, `peer_euid_of` and the admission decisions on both sides; **Coverage of new surfaces** states which write a line.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:74
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 166
    section: '## 1. Obs Scope Summary'
    change: 'Replace the row''s last sentence with «The socket-directory cause carries its code, `socket-dir-refused`; macOS compares no peer pid, the one named carve-out from this rule, owned by no route entry (the Founder Direction 4 trigger below states both).»'
    sidecar: 'Obs Scope Summary: Founder Direction 4 note, the route-entry owner retired; the socket-directory cause has its code, the macOS pid gap stays named.'
    rationale: 'Report **Retired** names the socket-directory check and code owed to "Unix endpoint and home hardening" as built; **Counts / qualifiers moved** discharges each owed clause, macOS''s pid gap staying named but owned by no route entry.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 191
    section: '## 1. Obs Scope Summary'
    change: 'After «(as measured at `.andromeda/runs/2026-10-26T19-02-07-implement/gate-2026-10-26-home-and-code-bearing-file-integrity.json`)», add «; the socket-directory check and the peer-euid check on the hook `connect` sit in that timed path too, and no value moved (chunk 2026-11-01-unix-endpoint-and-home-hardening: mean 2.0, 2.0, 2.0, 2.0, 2.3 ms, highest max 2.8 ms)»; the recorded band stays as it reads.'
    sidecar: 'Obs Scope Summary: hook timed path now holds the socket-directory and peer-euid checks, no perf value moved.'
    rationale: 'Report **Counts / qualifiers moved**, pattern `perf-rows`: both checks are in the timed path and no value moves; this row names what the timed path holds.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 205
    section: '## 1. Obs Scope Summary'
    change: 'New note after this one: «  - A stream whose peer euid is refused is dropped on the accept thread (`accept_loop`, `peer_admitted`), before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no line, there being no frame to key a line on and `channel-response`''s `detail` being closed, and nothing on the terminal.»'
    sidecar: 'Obs Scope Summary: ipc-internal note, the wrapper''s peer-euid drop writes no line.'
    rationale: 'Report **Symbols / APIs** (`accept_loop`) and **Coverage of new surfaces** (the wrapper''s drop, instrumentation n/a by that reason).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:75
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 216
    section: '## 1. Obs Scope Summary'
    change: 'In the `instance-unreachable` closed-cause list, add `socket-dir-refused` after `server-verify-failed`, so the list holds six.'
    sidecar: 'Obs Scope Summary: MCP `instance-unreachable` closed causes gain `socket-dir-refused`.'
    rationale: 'Report **Counts / qualifiers moved**, pattern `causes`: a list of the five extends; **Symbols / APIs** `frame` returns `UnreachableCause::SocketDir`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 303
    section: '## 1. Obs Scope Summary'
    change: 'In the exit-1 `detail` list, add «and `socket-dir-refused`, for `run` and a revived start alike: the socket-directory check before any snapshot, child or `Server::bind`, a step with no span and one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` line»; a `Server::bind` failure stays what it was.'
    sidecar: 'Obs Scope Summary: `viola run` start path gains `socket-dir-refused`, a step with no span and one line.'
    rationale: 'Report **Symbols / APIs** (`src/cmd/run.rs`), **Coverage of new surfaces** (first row), and **Expected amendments** «a step with no span and one line».'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 330
    section: '## 1. Obs Scope Summary'
    change: 'Read «`server-verify-failed` on Linux, Windows and macOS, where on macOS it means an euid mismatch alone: exit 21 on CLI `answer` with `during:"connect"`, fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint; `socket-dir-refused` the same shape on both, no path, pid or uid».'
    sidecar: 'Obs Scope Summary: dialog path, `server-verify-failed` reach widened to macOS by euid; `socket-dir-refused` added.'
    rationale: 'Report **Retired** names the client euid check as built; pattern `verify-word`: «off macOS» to Linux, Windows and macOS, each row changes. **Coverage of new surfaces** gives the frame-verb and hook shapes.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 374
    section: '## 1. Obs Scope Summary'
    change: 'Read «A server-verify mismatch (Linux, Windows and macOS, where on macOS it means an euid mismatch alone)» in place of «(Linux and Windows)»; the closing «macOS compares no peer pid (the recorded gap)» stays.'
    sidecar: 'Obs Scope Summary: hook fail-open server-verify row reaches macOS by euid; the pid gap stays named.'
    rationale: 'Report pattern `verify-word`, each row changes; pattern `mac-gap`: a row stating macOS compares no peer pid is a true claim and stays.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 374
    section: '## 1. Obs Scope Summary'
    change: 'New item after this one: «  - A refused socket directory is a further fail-open path: one `hook-decision{detail:"socket-dir-refused"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` first and `decision_emitted:false` — and no path, pid or uid in the line.»'
    sidecar: 'Obs Scope Summary: hook fail-open trigger gains the `socket-dir-refused` line.'
    rationale: 'Report **Symbols / APIs** (`src/cmd/hook.rs`) and **Coverage of new surfaces** (the check at `viola hook`).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 382
    section: '## 1. Obs Scope Summary'
    change: 'Before the closing «each likewise with no frame», add that a frame tool whose socket-directory check (`check_endpoint_dir`, after `live_endpoint`) refuses returns `socket-dir-refused`, and that a refused peer euid is the server check failing, so `server-verify-failed`; no line, `isError` the record.'
    sidecar: 'Obs Scope Summary: MCP rejected-call trigger gains `socket-dir-refused` and the peer-euid reading.'
    rationale: 'Report **Symbols / APIs** (`crates/viola-mcp/src/tools.rs` `frame`) and **Coverage of new surfaces** (the check at MCP''s `frame`).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:73
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 401
    section: '## 1. Obs Scope Summary'
    change: 'New item after this one: «  - The socket-directory check is its own cause, not a strict-modes failure of the rows above: its refusal writes the Founder Direction 4 trigger''s lines (below), on MCP none, and never chmods, a refused directory staying as found.»'
    sidecar: 'Obs Scope Summary: filesystem trigger places the socket-directory check beside the strict-modes rows.'
    rationale: 'Report **Symbols / APIs** (`check_socket_dir` «never chmods») and **Coverage of new surfaces**: this trigger counts where a strict-modes failure has a line, and the new check has lines under its own word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 460
    section: '## 1. Obs Scope Summary'
    change: 'Read «the server check''s peer-pid read: Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the pid verdict is `Matches` with no comparison: macOS compares no peer pid, the named carve-out, owned by no route entry; `server-verify-failed` is still written there, for an euid mismatch alone).»'
    sidecar: 'Obs Scope Summary: multi-OS row, the macOS pid gap no longer owned by a route entry, `server-verify-failed` written there on an euid mismatch.'
    rationale: 'Report pattern `owner-title`: all change, macOS''s pid gap staying named but owned by no route entry; pattern `verify-word` retires «no `server-verify-failed` line is written there».'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 460
    section: '## 1. Obs Scope Summary'
    change: 'New item after this one: «  - the peer-euid read: Unix `peer_creds` (`PEER_EUID_REPORTED` true, a peer it cannot read refused) vs Windows none (`PEER_EUID_REPORTED` false, the DACL and the pid comparison staying the controls); the socket-directory check: Unix `symlink_metadata` vs off Unix `Ok(())` with no read, so it refuses nothing there.»'
    sidecar: 'Obs Scope Summary: multi-OS trigger gains the peer-euid read and the socket-directory check per OS.'
    rationale: 'Report **Symbols / APIs**: `PEER_EUID_REPORTED` = `cfg!(unix)`, `peer_euid_admitted`, and `check_socket_dir` off Unix returning `Ok(())`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 491
    section: '## 1. Obs Scope Summary'
    change: 'Add `socket-dir-refused` to both lists: after `server-verify-failed` among the causes behind exit 21, and after `batch-script-child` among the exit 1 causes that `run` holds with no `viola revive` qualifier, a revived start sharing it.'
    sidecar: 'Obs Scope Summary: Founder Direction 4 distinct-code lists gain `socket-dir-refused` on exit 21 and exit 1.'
    rationale: 'Report **Counts / qualifiers moved**, pattern `causes` and `revive-codes`: each list extends; **Coverage of new surfaces** gives exit 1 at the start and exit 21 at the frame verbs.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
    dependent-of: D-new-detail
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 492
    section: '## 1. Obs Scope Summary'
    change: 'Read «written on Linux, Windows and macOS as `process-exit{...}`» in place of «written on Linux and Windows as», and close the row with «on macOS it means an euid mismatch alone, since macOS compares no peer pid» in place of «macOS writes none (no peer pid, the recorded gap)».'
    sidecar: 'Obs Scope Summary: Founder Direction 4 server-verify row reaches macOS by euid; «macOS writes none» retired.'
    rationale: 'Report **Retired** names the client euid check as built; pattern `verify-word`: on macOS the word means an euid mismatch alone; **Symbols / APIs** `src/cmd/client.rs` `connect`: a refused euid is `CallError::ServerUnverified`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 493
    section: '## 1. Obs Scope Summary'
    change: 'Replace the row with «  - The socket-directory cause carries its code `socket-dir-refused`, a failed create and a refused check reading the one code: `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` from `run` and a revived start; `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN with no `channel-request` from the eight frame verbs; one `hook-decision{detail:"socket-dir-refused"}` at WARN from the hook; no line on MCP, `isError` the record; no path, pid or uid in any. macOS compares no peer pid: the named carve-out, owned by no route entry.», the OPEN clause gone.'
    sidecar: 'Obs Scope Summary: OPEN on the socket-directory code closed, the owner retired; its lines recorded; the macOS pid gap stays named.'
    rationale: 'Report **Retired** (the socket-directory check and code, built), **Decisions & corrections** (a failed create and a refused check read one code) and **Coverage of new surfaces** (each line).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
    dependent-of: D-retired
note: 'D-obs-instrumentation, D-obs-stack and D-obs-pii read no violation in this file (no dependency moved; the report gives a no-line reason for the wrapper drop and MCP; no path, pid or uid). The body lines .andromeda/obs-plan.md:311, :361 and :537 carry moved claims outside this category file.'
