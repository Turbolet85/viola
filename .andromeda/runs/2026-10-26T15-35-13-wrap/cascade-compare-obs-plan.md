# Cascade compare — obs-plan · wrap · 2026-10-26-explicit-session-links
E items 19 · held 15 · lost at classify 2 · lost at strong 0 · beside a placed fact 0 · cascade only 1 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### obs-scope-summary#33.1 · lost at classify · .andromeda/obs-plan-detail/obs-scope-summary.md:355
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 355
    section: '## 1. Obs Scope Summary'
    change: '  - The served `link` / `unlink` methods add nothing beyond this pair (`method:"link"|"unlink"`): no span and no process-log line of their own, and the pair holds neither the `driver` nor the `driven` param (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'obs-scope-summary Vector 1 trigger: the served `link` / `unlink` methods logged by the channel pair alone, no names (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Report Symbols / APIs: neither method opens a span nor writes a process-log line, the wrapper''s `channel-request` / `channel-response` pair carries `method:"link"|"unlink"`; Outcome (obs): channel pair holds no names, met by `link_verbs_log_the_channel_pair_and_no_names`; Coverage: no driver name in the wrapper''s process log, asserted.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:28

### pii-scrubbing-compliance#1.11 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 20
    section: ## 8. PII Scrubbing & Compliance › ### PII Scrubbing
    change: add one table row after the `claude agents --json` row — "| `link` / `unlink` pair names (`driver`, `driven`, `ViolaName`) | Low | OK in the contract payload: the wrapper's `link` / `unlink` event lines in `events.ndjson` (`data: {driver, driven}`, `source: wrapper`); the driver name reaches no wrapper process-log line | the served `link` / `unlink` methods open no span and write no process-log line of their own; the wrapper's `channel-request` / `channel-response` pair carries `method:"link"\|"unlink"` and no name; the verbs' result lines print the verb's own parsed `ViolaName` arguments, never a name read back from the reply or the log; asserted by `link_verbs_log_the_channel_pair_and_no_names` (`tests/cli_links.rs`) |"
    sidecar: PII table gains the `link` / `unlink` pair-names row — names only in the `events.ndjson` link lines, none in the wrapper's channel pair or any span (chunk 2026-10-26-explicit-session-links)
    rationale: the report's Changes (Symbols / APIs: "Neither opens a span nor writes a process-log line; the wrapper's existing `channel-request` / `channel-response` pair carries `method`"; the product `unlink` line `{driver, driven}`, `source: wrapper`; result lines print "the verb's own parsed `ViolaName` arguments, never a name read back from the reply or the log") and Coverage ("PII n/a (names only; no driver name in the wrapper's process log — asserted)"; Outcome (obs) "channel pair holds no names — met: `link_verbs_log_the_channel_pair_and_no_names`") bring a new user-supplied name class onto the logged surface whose handling this table does not yet classify
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:28

## Held

- error-capture-reporting#3.1
- log-coverage#2.2
- log-coverage#2.7
- metric-coverage#2.9
- obs-anti-patterns#1.1
- obs-scope-summary#18.3
- obs-scope-summary#38.1
- obs-scope-summary#48.1
- obs-scope-summary#49.1
- slo-invariants-telemetry-budgets#1.1
- span-trace-coverage#1.1
- span-trace-coverage#11.1
- span-trace-coverage#2.1
- span-trace-coverage#3.1
- telemetry-strategy#2.1

## Cascade only

- log-coverage#1.4
