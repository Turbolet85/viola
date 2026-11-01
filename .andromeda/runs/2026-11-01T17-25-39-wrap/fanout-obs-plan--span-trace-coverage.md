proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 28
    section: '## 4. Span / Trace Coverage'
    change: 'In the server-check bullet: before the peer pid clause add the euid read, `peer_admitted(client.server_euid())` straight after `Client::connect` and before `verify_server`, writing and logging nothing; make the refusal sentence read «A refused euid, a mismatch or a check that cannot be made»; replace «Linux and Windows; macOS reports no peer pid and stays on the recorded gap.» with «The peer euid on Linux and macOS (Windows reports none, where the DACL and the pid comparison stay the controls), the peer pid and start time on Linux and Windows; macOS reports no peer pid and stays on the recorded gap, so there `server-verify-failed` means an euid mismatch alone.»'
    sidecar: '§4 cli server check: the client euid read added ahead of the pid comparison; server-verify-failed reaches Linux, Windows and macOS, on macOS an euid mismatch alone.'
    rationale: 'report.md:15 puts `peer_admitted(client.server_euid())` between `Client::connect` and `verify_server` with a refused euid reading `server-verify-failed`; report.md:31 moves the word from off macOS to Linux, Windows and macOS; report.md:33 keeps the statement that macOS compares no peer pid.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 28
    section: '## 4. Span / Trace Coverage'
    change: 'New bullet after this one: «- **cli (short-lived verbs), the socket-directory check:** each of the eight frame verbs runs `check_endpoint_dir` on the recorded endpoint inside `cmd::client::connect`, in its dispatch span, ahead of `Client::connect`. A symlink, a non-directory, a foreign owner or a foreign bit is refused, an absent path passes, and the directory stays as found: no frame, no `channel-request` line, and `process-exit{subject:"self", exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, with no path, pid or uid.»'
    sidecar: '§4 cli: the socket-directory check at the eight frame verbs and its exit-21 line.'
    rationale: 'report.md:15 and report.md:71 give the check, its order and its line; report.md:11 gives the verdict and that the check never chmods; report.md:70 records no path, pid or uid.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 29
    section: '## 4. Span / Trace Coverage'
    change: 'Append to this bullet: «The socket-directory refusal has two captured-span cases of its own: `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span`, which opens `wait.block` by hand around `connect`, the check sitting in `connect` for its eight callers, and `hook_refuses_a_loosened_socket_directory_inside_hook_handle`. The span recorder `Seen` lives in the test module `refused_seen`, outside the macOS-gated `refused_inside`.»'
    sidecar: '§4 captured-span witnesses: the two socket-directory cases and the recorder module.'
    rationale: 'report.md:82 and report.md:83 record the recorder move and the hand-opened span; the report section New text, by line, lists both new cases.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:83
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 31
    section: '## 4. Span / Trace Coverage'
    change: 'In the hook server-check bullet: make the parenthesis read «`read_snapshot_or_replay`, its span `state.snapshot_recover` under `hook.handle`, then `check_endpoint_dir`, then `connect_by`, then `peer_admitted(client.server_euid())`, then `verify_server`»; add that a refused euid reads as a server that is not the recorded wrapper; replace «Linux and Windows; macOS on the recorded gap.» with «The peer euid on Linux and macOS, the peer pid and start time on Linux and Windows; macOS on the recorded gap, so there the word means an euid mismatch alone.»'
    sidecar: '§4 hook server check: the three checks in order; server-verify-failed now reaches macOS as an euid mismatch alone.'
    rationale: 'report.md:16 gives the same three checks in the same order and a refused euid as `NotSent::ServerUnverified`; report.md:31 moves the reach of the word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
    dependent-of: D-obs-instrumentation
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 31
    section: '## 4. Span / Trace Coverage'
    change: 'New bullet after this one: «- **cli (`viola hook`), the socket-directory check:** a refused socket directory gets no frame: no `channel.request` opens and the hook fails open (exit 0, empty streams) with one `hook-decision{detail:"socket-dir-refused"}` at WARN, inside `hook.handle` and in the timed path of `run --perf`.»'
    sidecar: '§4 hook: the socket-directory check fails open with one hook-decision at WARN.'
    rationale: 'report.md:16 and report.md:72 give the record and the timed path; report.md:95 records the lean that the hook record is one `hook-decision` line.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 32
    section: '## 4. Span / Trace Coverage'
    change: 'New bullet after this one: «- **ipc-internal (viola-channel), the wrapper peer drop:** `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))` (`Server::serve` passing `peer_admitted`), so a refused stream is dropped on the accept thread, before its worker thread and before any frame: it reads end-of-file and gets no `channel.dispatch` work. No line by design: there is no frame to key a line on and `channel-response`''s `detail` is closed. Nothing reaches the outer terminal and the wheel does not move.»'
    sidecar: '§4 viola-channel: the wrapper drops a refused peer on the accept thread, writing no line.'
    rationale: 'report.md:13 gives the filter and the drop; report.md:75 records instrumentation n/a with its reason and that the drop writes nothing to the outer terminal and moves no wheel; report.md:95 records the lean.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:75
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 33
    section: '## 4. Span / Trace Coverage'
    change: 'In the MCP server-check bullet: make the refusal sentence read «A refused euid (`!peer_admitted(client.server_euid())`), a mismatch or a check that cannot be made returns…»; replace «Linux and Windows; macOS on the recorded gap.» with «The peer euid on Linux and macOS, the peer pid and start time on Linux and Windows; macOS on the recorded gap, so there the word means an euid mismatch alone.»'
    sidecar: '§4 MCP server check: the euid read joins the verdict; the reach of the word now includes macOS.'
    rationale: 'report.md:17 joins `!peer_admitted(client.server_euid())` to `!server_verified(…)` under `UnreachableCause::ServerVerify`; report.md:31 moves the reach of the word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
    dependent-of: D-obs-instrumentation
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 33
    section: '## 4. Span / Trace Coverage'
    change: 'New bullet after this one: «- **ipc-internal (viola-mcp), the socket-directory check:** the frame tools run `check_endpoint_dir` after `live_endpoint`, inside `frame`; a refused directory returns the codes-only `isError` `instance-unreachable` with `detail` `socket-dir-refused`, no frame and no log line, the `isError` its record.»'
    sidecar: '§4 MCP: the socket-directory check, an isError with no line.'
    rationale: 'report.md:17 and report.md:73 give the place and the no-line rule; report.md:111 gives the word on MCP less `v`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:73
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 42
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'New sub-bullet after this one: «  - The socket-directory step carries no span and, on a refusal, one line, as landed (chunk 2026-11-01-unix-endpoint-and-home-hardening): `socket_dir_ready(endpoint)` runs `check_endpoint_dir`, creates the parent only when absent, then checks again, before any snapshot, child or `Server::bind`. A refusal is `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, with no path, pid or uid; `viola revive` starts through the same step, and a `Server::bind` failure stays what it was.»'
    sidecar: '§4 run start: the socket-directory step, no span and one line on a refusal.'
    rationale: 'report.md:14 gives the step and its place; report.md:65 names the place on the start chain as a step with no span and one line; report.md:70 gives the line.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 57
    section: '## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn'
    change: 'In the failure `detail` set, make the list read `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child|socket-dir-refused`.'
    sidecar: '§4 run start: socket-dir-refused joins the exit-1 detail set.'
    rationale: 'report.md:14 and report.md:70 add `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` on `viola run` and the start of revive; the listed set omits it.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 78
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Make the server-check sentences read: the socket-directory check runs first, then the peer euid, then the peer pid and its start time against the one snapshot read; a refused directory, a refused euid, a server that is not the recorded wrapper or a check that cannot be made opens no `channel.request`, and the path ends there. Replace «(Linux and Windows; macOS on the recorded gap)» with «(the peer euid on Linux and macOS, the peer pid and start time on Linux and Windows; macOS on the recorded gap, so there `server-verify-failed` means an euid mismatch alone)».'
    sidecar: '§4 send: the socket-directory and euid checks precede the pid comparison; the reach of the word now includes macOS.'
    rationale: 'report.md:15 and report.md:17 give the order at the CLI and MCP seams; report.md:31 moves the reach of the word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 93
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Replace «before any frame (Linux and Windows):» with «before any frame (on Linux, Windows and macOS, on macOS an euid mismatch alone):».'
    sidecar: '§4 send log fields: the server-verification refusal now reaches macOS.'
    rationale: 'report.md:31 moves `server-verify-failed` from off macOS to Linux, Windows and macOS, each row changing.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-obs-instrumentation
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 93
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'New sub-bullet after this one: «  - client-side socket-directory refusal, made before any frame: no `channel-request` / `channel-response` line on either side; CLI `send` ends `process-exit{subject:"self", exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, and MCP `send` writes no line;»'
    sidecar: '§4 send log fields: the socket-directory refusal.'
    rationale: 'report.md:71 and report.md:73 give the CLI line and the MCP no-line rule.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 96
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'At the end of this line, replace «Linux and Windows, macOS on the recorded gap» with «on macOS an euid mismatch alone».'
    sidecar: '§4 send cleanup: server-verify-failed now reaches macOS as an euid mismatch alone.'
    rationale: 'report.md:31 moves the reach of the word, each row changing; report.md:33 keeps the macOS pid gap elsewhere as a true claim, and this shorter wording brings the line under the 2 000 char limit of the entity shape.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-obs-instrumentation
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 96
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'New sub-bullet after this line: «  - `send.client` also closes with exit 21 `detail:"socket-dir-refused"` with `during` `connect` (the socket directory of the recorded endpoint is refused; no frame sent).»'
    sidecar: '§4 send cleanup: socket-dir-refused joins the exit-21 closes.'
    rationale: 'report.md:28 adds the sixth cause and has a list of the causes extend; report.md:15 gives `during` `connect`. A sub-bullet because the line itself is at the 2 000 char limit.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 98
    section: '## 4. Span / Trace Coverage › #### Scenario: Confirmed `send` (CL-1) from driver to readback'
    change: 'Make the list read «`instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during` `connect`.»'
    sidecar: '§4 send cleanup: socket-dir-refused is during connect.'
    rationale: 'report.md:28 has a list of the causes extend; report.md:15 and report.md:71 give `during` `connect`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 107
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Make the sentence read: the socket-directory check and the server check run inside `wait.block` / `last.client`, after the client''s own refusals and before `channel.request`; a refused directory, a refused euid, a server that is not the recorded wrapper or a check that cannot be made opens no `channel.request`. Replace «(Linux and Windows; macOS on the recorded gap)» with «(the peer euid on Linux and macOS, the peer pid and start time on Linux and Windows; macOS on the recorded gap, so there `server-verify-failed` means an euid mismatch alone)».'
    sidecar: '§4 wait / last: the socket-directory and euid checks; the reach of the word now includes macOS.'
    rationale: 'report.md:15 lists `wait` and `last` among the eight callers of `connect` and gives the order; report.md:31 moves the reach of the word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 114
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'After the `endpoint-lost` clause add «`detail:"socket-dir-refused"` with `during:"connect"` when the socket directory is refused,»; replace «(Linux and Windows; macOS on the recorded gap)» with «(on Linux, Windows and macOS, on macOS an euid mismatch alone)»; make the order read «then the liveness-only pre-check, then the socket-directory check and then the server check».'
    sidecar: '§4 wait / last log fields: socket-dir-refused joins the exit-21 details; server-verify-failed now reaches macOS.'
    rationale: 'report.md:15 gives the order and the word on `connect`; report.md:28 has a list of the causes extend; report.md:31 moves the reach of the word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 116
    section: '## 4. Span / Trace Coverage › #### Scenario: `wait` / `last` event-driven readback'
    change: 'Make the closing clause read «or on a socket-directory or server-verification refusal made before any frame (no `channel.request` opened; the client''s `process-exit` its outcome line).»'
    sidecar: '§4 wait cleanup: a socket-directory refusal also closes wait.block.'
    rationale: 'report.md:83 records the case that reads the refusal inside `wait.block`; report.md:71 gives the outcome line and no `channel-request`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:83
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 125
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'Make the closing sentences read: the socket-directory check and the server check come before each frame of this path, the dialog hook''s inside `hook.handle`, before `channel.request(hook.dialog)`, and the driver''s inside `answer.client`, before `channel.request(answer)`; a refused directory, a refused euid, a server that is not the recorded wrapper or a check that cannot be made opens neither. Replace «(Linux and Windows; macOS on the recorded gap)» with «(the peer euid on Linux and macOS, the peer pid and start time on Linux and Windows; macOS on the recorded gap, so there `server-verify-failed` means an euid mismatch alone)».'
    sidecar: '§4 dialog: the socket-directory and euid checks on both sides; the reach of the word now includes macOS.'
    rationale: 'report.md:15 and report.md:16 give the checks at `answer` and at the hook; report.md:31 moves the reach of the word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
    dependent-of: D-obs-instrumentation
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 134
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'New sub-bullet after this one: «  - A refused socket directory is another: `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"socket-dir-refused"}` at WARN, with no `hook.dialog` frame and so no `dialog-raised`.»'
    sidecar: '§4 dialog hook: the socket-directory fail-open pair of lines.'
    rationale: 'report.md:72 gives one `hook-decision{detail:"socket-dir-refused"}` at WARN, on the dialog path `hook-invoked{corr:null}` first and `decision_emitted:false`; report.md:16 gives no frame.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 137
    section: '## 4. Span / Trace Coverage › #### Scenario: Dialog → `answer` (question / permission / plan), including the unverified-CLI branch'
    change: 'New sub-bullet after this one: «  - a socket-directory refusal on CLI `answer`, made before any frame: no `channel-request` / `channel-response` line, `process-exit{subject:"self", exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN; MCP `answer` writes no line.»'
    sidecar: '§4 answer log fields: the socket-directory refusal.'
    rationale: 'report.md:15 lists `answer` among the callers of `connect`; report.md:71 and report.md:73 give the CLI line and the MCP no-line rule.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 149
    section: '## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it'
    change: 'Make the sentence read: `pause.client` / `release.client` run the socket-directory check and the server check before `channel.request(pause|release)`; a refused directory, a refused euid, a server that is not the recorded wrapper or a check that cannot be made opens no `channel.request` and sends no frame. Replace «(Linux and Windows; macOS on the recorded gap)» with «(the peer euid on Linux and macOS, the peer pid and start time on Linux and Windows; macOS on the recorded gap, so there `server-verify-failed` means an euid mismatch alone)».'
    sidecar: '§4 pause / release: the socket-directory and euid checks; the reach of the word now includes macOS.'
    rationale: 'report.md:15 lists `pause` and `release` among the callers of `connect` and gives the order; report.md:31 moves the reach of the word.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-obs-instrumentation
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 154
    section: '## 4. Span / Trace Coverage › #### Scenario: Human takes the wheel, automation is refused, `release` returns it'
    change: 'New sub-bullet after this one: «  - a socket-directory refusal on `pause` / `release`, made before any frame: no `channel-request` / `channel-response` line, the client''s `process-exit{subject:"self", exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN;»'
    sidecar: '§4 pause / release log fields: the socket-directory refusal.'
    rationale: 'report.md:15 lists `pause` and `release` among the callers of `connect`; report.md:71 gives the line and no `channel-request`.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
note: 'No D-retired: this file names neither the route owner nor an owed check, and D-obs-stack and D-obs-pii find no drift (report.md:23, report.md:70). The body .andromeda/obs-plan.md still carries the old reach of server-verify-failed and the socket-dir omission in its §4 wait / last scenario and §7 MCP rejects, and the census at obs-plan.md:311, all outside this file.'
