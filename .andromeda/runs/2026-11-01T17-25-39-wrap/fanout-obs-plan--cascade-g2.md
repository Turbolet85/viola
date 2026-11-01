proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 11
    section: '## 2. Telemetry Strategy'
    change: 'In the Structured Logs row, widen the parenthesis to read: (the `mcp` role writes no self `process-exit`, Section 7; a peer the wrapper drops on the accept thread has no line, no frame existing to key one on, and MCP `frame` refusals of a socket directory or a server euid have none, `isError` their record)'
    sidecar: 'telemetry-strategy Structured Logs row: every refusal as a line now carves out the wrapper drop of a refused peer and MCP frame refusals of a socket directory or a server euid.'
    rationale: 'The chunk adds refusals that write no line by rule: **Symbols / APIs**, the accept_loop drop before any frame, and MCP frame with no line and `isError` the record; **Coverage of new surfaces** reads instrumentation n/a on both (report.md:13, report.md:17, report.md:73, report.md:75).'
    basis: crates/viola-mcp/src/tools.rs:345-347
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 30
    section: '## 2. Telemetry Strategy'
    change: 'In the Vector 4 row, the last sentence reads: The "server-verify mismatch" item writes one `hook-decision{detail:"server-verify-failed"}` at WARN on Linux, Windows and macOS (macOS compares no peer pid, the recorded gap, so there the word means an euid mismatch alone)'
    sidecar: 'telemetry-strategy Vector 4 row: the hook server-verify line now reaches macOS, on an euid mismatch alone; the no-peer-pid gap stays named.'
    rationale: '**Counts / qualifiers moved**: the reach of `server-verify-failed` moves to Linux, Windows and macOS, each row changing, and a row stating macOS compares no peer pid keeps that true claim; the hook reads a refused euid as `NotSent::ServerUnverified` (report.md:16, report.md:31, report.md:33).'
    basis: src/cmd/hook.rs:236-238
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 30
    section: '## 2. Telemetry Strategy'
    change: 'In the Vector 4 row, after the sentence on the event and dialog paths refused or absent instance, add: A refused socket directory is no such silent path: the hook `connect` writes one `hook-decision{detail:"socket-dir-refused"}` at WARN, with exit 0, empty streams and no frame'
    sidecar: 'telemetry-strategy Vector 4 row: a refused socket directory is named as a hook fail-open that writes one hook-decision, apart from the silent refused-instance paths.'
    rationale: '**Symbols / APIs** and **Coverage of new surfaces**: the hook meets the directory check in `connect` and writes one `hook-decision` at WARN, exit 0, empty streams, no frame; the row reads a refused instance as lineless, so this refusal needs naming beside it (report.md:16, report.md:59, report.md:72).'
    basis: src/cmd/hook.rs:231-233
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 33
    section: '## 2. Telemetry Strategy'
    change: 'In the Vector 7 row, after the §10 exemptions clause, add: ; the socket-directory check shares the read verdict yet is no such exemption, §6 `socket-dir-refused`: one `process-exit` at a `run` or `revive` start and at the eight CLI frame verbs, one `hook-decision` at `viola hook`, and no line only at MCP `frame`'
    sidecar: 'telemetry-strategy Vector 7 row: the socket-directory refusal is distinguished from the no-line strict-modes exemptions, with its own word and its line sites.'
    rationale: '`socket_dir_verdict` ends in `unix_read_verdict`, yet **Coverage of new surfaces** records a line at the start, the frame verbs and the hook, and none at MCP `frame`; the row reads such checks as lineless, and the strict-modes sites themselves do not move (report.md:11, report.md:70, report.md:71, report.md:72, report.md:73).'
    basis: crates/viola-state/src/strict.rs:183-198
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 43
    section: '## 2. Telemetry Strategy'
    change: 'In the Founder Direction 4 row, after the strict-modes refusal clause, add: ; a refused socket directory is outside that exception: its cause carries the logged `detail` `socket-dir-refused`, no path or pid, exit 21 at the CLI frame verbs and exit 1 at `run` and the `revive` start'
    sidecar: 'telemetry-strategy Founder Direction 4 row: socket-dir-refused is a further distinct logged detail code on exit 21 and exit 1, outside the before-any-log exception.'
    rationale: '**Coverage of new surfaces** records `process-exit` with `detail` `socket-dir-refused` at exit 1 on a start and exit 21 on a frame verb; the sixth exit 21 cause is in **Counts / qualifiers moved**, and the (obs) acceptance reads no path, pid or uid (report.md:28, report.md:70, report.md:71, report.md:116).'
    basis: schemas/diag-line.v1.json:78
note: 'D-obs-instrumentation, D-obs-stack and D-obs-pii read no drift: each new surface is logged or carries a reasoned no-line, no dependency moved, and the new lines hold no path, pid or uid; no New facts item is in telemetry-strategy, F4 and F6 stay with span-trace-coverage and F1 with log-coverage for the new-detail pass.'
