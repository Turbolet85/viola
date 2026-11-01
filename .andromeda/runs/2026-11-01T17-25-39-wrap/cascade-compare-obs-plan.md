# Cascade compare — obs-plan · wrap · 2026-11-01-unix-endpoint-and-home-hardening
E items 71 · held 42 · lost at classify 2 · lost at strong 0 · beside a placed fact 1 · cascade only 5 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### obs-scope-summary#20.5 · lost at classify · .andromeda/obs-plan-detail/obs-scope-summary.md:191
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 191
    section: '## 1. Obs Scope Summary'
    change: 'After «(as measured at `.andromeda/runs/2026-10-26T19-02-07-implement/gate-2026-10-26-home-and-code-bearing-file-integrity.json`)», add «; the socket-directory check and the peer-euid check on the hook `connect` sit in that timed path too, and no value moved (chunk 2026-11-01-unix-endpoint-and-home-hardening: mean 2.0, 2.0, 2.0, 2.0, 2.3 ms, highest max 2.8 ms)»; the recorded band stays as it reads.'
    sidecar: 'Obs Scope Summary: hook timed path now holds the socket-directory and peer-euid checks, no perf value moved.'
    rationale: 'Report **Counts / qualifiers moved**, pattern `perf-rows`: both checks are in the timed path and no value moves; this row names what the timed path holds.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:38

### obs-scope-summary#21.5 · lost at classify · .andromeda/obs-plan-detail/obs-scope-summary.md:205
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 205
    section: '## 1. Obs Scope Summary'
    change: 'New note after this one: «  - A stream whose peer euid is refused is dropped on the accept thread (`accept_loop`, `peer_admitted`), before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no line, there being no frame to key a line on and `channel-response`''s `detail` being closed, and nothing on the terminal.»'
    sidecar: 'Obs Scope Summary: ipc-internal note, the wrapper''s peer-euid drop writes no line.'
    rationale: 'Report **Symbols / APIs** (`accept_loop`) and **Coverage of new surfaces** (the wrapper''s drop, instrumentation n/a by that reason).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:75

### span-trace-coverage#3.5 · lost at classify · beside a placed fact F6 (near 21, span-trace-coverage#2.4) · .andromeda/obs-plan-detail/span-trace-coverage.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 32
    section: '## 4. Span / Trace Coverage'
    change: 'New bullet after this one: «- **ipc-internal (viola-channel), the wrapper peer drop:** `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))` (`Server::serve` passing `peer_admitted`), so a refused stream is dropped on the accept thread, before its worker thread and before any frame: it reads end-of-file and gets no `channel.dispatch` work. No line by design: there is no frame to key a line on and `channel-response`''s `detail` is closed. Nothing reaches the outer terminal and the wheel does not move.»'
    sidecar: '§4 viola-channel: the wrapper drops a refused peer on the accept thread, writing no line.'
    rationale: 'report.md:13 gives the filter and the drop; report.md:75 records instrumentation n/a with its reason and that the drop writes nothing to the outer terminal and moves no wheel; report.md:95 records the lean.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:75

## Held

- error-capture-reporting#2.1
- error-capture-reporting#5.2
- error-capture-reporting#6.3
- error-capture-reporting#6.5
- log-coverage#2.1
- log-coverage#2.2
- log-coverage#2.4
- log-coverage#2.8
- metric-coverage#3.1
- obs-anti-patterns#1.1
- obs-scope-summary#14.3
- obs-scope-summary#18.5
- obs-scope-summary#22.5
- obs-scope-summary#27.3
- obs-scope-summary#30.3
- obs-scope-summary#35.1
- obs-scope-summary#36.1
- obs-scope-summary#38.1
- obs-scope-summary#4.3
- obs-scope-summary#44.1
- obs-scope-summary#48.1
- pii-scrubbing-compliance#2.1
- slo-invariants-telemetry-budgets#2.2
- span-trace-coverage#3.1
- span-trace-coverage#3.2
- span-trace-coverage#3.4
- span-trace-coverage#3.6
- span-trace-coverage#4.1
- span-trace-coverage#4.3
- span-trace-coverage#6.1
- span-trace-coverage#6.3
- span-trace-coverage#6.4
- span-trace-coverage#7.1
- span-trace-coverage#7.3
- span-trace-coverage#7.4
- span-trace-coverage#8.1
- span-trace-coverage#8.3
- span-trace-coverage#9.1
- span-trace-coverage#9.3
- telemetry-strategy#3.16
- telemetry-strategy#3.3
- telemetry-strategy#3.6

## Cascade only

- error-capture-reporting#5.1
- obs-scope-summary#29.3
- span-trace-coverage#2.4
- span-trace-coverage#3.3
- telemetry-strategy#1.2
