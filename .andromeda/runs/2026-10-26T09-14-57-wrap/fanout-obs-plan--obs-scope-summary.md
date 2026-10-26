proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 370
    section: '## 1. Obs Scope Summary'
    change: 'Append to the fail-open bullet: As landed (chunk 2026-10-26-server-verification-before-any-frame) a server-verify mismatch is logged on Linux and Windows as one `hook-decision{detail:"server-verify-failed"}` at WARN in `hook-<name>.ndjson` (a dialog hook writes `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}`), exit 0, streams empty, no frame, and no pid, start time, path or endpoint in the line; macOS compares no peer pid (the recorded gap).'
    sidecar: 'obs-scope-summary Vector 4 trigger: the server-verify mismatch fail-open path recorded as landed (hook-decision server-verify-failed at WARN, Linux and Windows), chunk 2026-10-26-server-verification-before-any-frame.'
    rationale: 'Report Changes, Schema / config: this build is the first that writes `hook-decision{detail:"server-verify-failed"}` (dialog path `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, ...}`), no pid, start time, path or endpoint asserted; Coverage of new surfaces, `viola hook` row; Retired: macOS stays on the recorded gap. The file lists the mismatch as a required fail-open signal with an as-landed note only for strict-modes; the landed shape of the mismatch line is new detail.'
    basis: .andromeda/obs-plan-detail/obs-scope-summary.md:370
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 485
    section: '## 1. Obs Scope Summary'
    change: 'Append to the exit-21 / exit-1 bullet: As landed (chunk 2026-10-26-server-verification-before-any-frame) the server-verify fail is written by the six CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson`, inside the verb dispatch span with no `channel-request` line and no pid, start time, path or endpoint; `viola mcp` frame tools write no line for it (`isError` `instance-unreachable`); Linux and Windows only, macOS on the recorded gap.'
    sidecar: 'obs-scope-summary Founder Direction 4 trigger: the exit-21 server-verify-failed detail recorded as landed for the six CLI frame verbs (process-exit during connect at WARN), MCP writing no line, chunk 2026-10-26-server-verification-before-any-frame.'
    rationale: 'Report Changes, Schema / config: `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson` is first written by this build, no pid, start time, path or endpoint (asserted), MCP writes no line for the mismatch; Symbols / APIs: the six verbs go through `client::connect` inside their dispatch spans; Coverage: no `channel.request`. The file names server-verify fail as a distinct exit-21 cause with an as-landed note only for revive strict-modes; the landed shape is new detail.'
    basis: .andromeda/obs-plan-detail/obs-scope-summary.md:485
note: 'D-obs-instrumentation, D-obs-stack and D-obs-pii hold for this file (no dependency change; every new pre-frame refusal logged codes-only or by plan unlogged on MCP); no passage here states the retired dated gaps; the must-trace path rows (299-345) describe success paths and the «cannot occur» wait/last sentence lives in span-trace-coverage.md:90, not this file.'
