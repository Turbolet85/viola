proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 30
    section: "## 2. Telemetry Strategy"
    change: In the Vector 4 row the server-verify clause reads, The "server-verify mismatch" item writes one `hook-decision{detail:"server-verify-failed"}` at WARN on Linux, Windows and macOS, where on macOS it means an euid mismatch alone (macOS compares no peer pid, the recorded gap)
    sidecar: Telemetry Strategy, Vector 4 row - the hook's `server-verify-failed` line reaches macOS as well, there an euid mismatch alone; the no-peer-pid gap stays named
    rationale: The report moves the word's reach from off macOS to Linux, Windows and macOS, each row changing (report.md:31), and a refused euid at the hook is `NotSent::ServerUnverified` (report.md:16); a row stating macOS compares no peer pid stays true (report.md:33), so only the OS list and the macOS meaning move
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 30
    section: "## 2. Telemetry Strategy"
    change: After the server-verify clause the Vector 4 row adds, A refused socket directory writes one `hook-decision{detail:"socket-dir-refused"}` at WARN, on the dialog path after `hook-invoked{corr:null}` and with `decision_emitted:false`; exit 0, empty streams, no frame, no path, pid or uid in the line
    sidecar: Telemetry Strategy, Vector 4 row - the hook's refused socket directory recorded, one `hook-decision{detail:"socket-dir-refused"}` at WARN, fail-open
    rationale: The chunk adds the socket-directory check at `viola hook` with this record (report.md:16, report.md:72, the lean at report.md:95), and the row lists the hook's other refusal records, so a new one belongs beside them
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 33
    section: "## 2. Telemetry Strategy"
    change: After the row's §10 exemptions the Vector 7 row adds, The socket-directory check is not one of those no-line checks, its refusal reads `socket-dir-refused` with no path, pid or uid - `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` at `run` and `revive`'s start, `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN with no `channel-request` on the eight frame verbs, one `hook-decision` at WARN on `viola hook`; `viola mcp`'s `frame` writes no line, `isError` the record
    sidecar: Telemetry Strategy, Vector 7 row - the socket-directory refusal's lines per role recorded beside the no-line strict-modes exemptions
    rationale: The report's coverage rows give this check a line on three roles and none on MCP (report.md:70, report.md:71, report.md:72, report.md:73), while the row says a strict-modes check before a read or open on those same verbs writes no line; without the carve-out a reader takes the new check for one of them
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 43
    section: "## 2. Telemetry Strategy"
    change: After the strict-modes exception the Founder Direction 4 row adds, A refused socket directory is its own cause outside that exception, its logged `process-exit` `detail` `socket-dir-refused` on exit 21 and on exit 1, and the same word among `revive --json`'s exit-1 codes
    sidecar: Telemetry Strategy, Founder Direction 4 row - `socket-dir-refused` recorded as a distinct logged code on exits 21 and 1, outside the no-line exception
    rationale: Direction 4 asks a distinct `detail` code per cause behind exits 21 and 1; the founder's Q1 makes this refusal a new closed word (report.md:93), logged on both exits (report.md:70, report.md:71), with `revive --json`'s codes moving to eleven (report.md:29)
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:93
note: D-obs-stack and D-obs-pii read clean for this category (no dependency added, report.md:23; no path, pid or uid, report.md:70-72); D-obs-instrumentation found no uninstrumented hot path here, the wrapper's drop writing no line by the report's own ruling (report.md:75), which trigger row 28 of this file, naming sections only, does not contradict
