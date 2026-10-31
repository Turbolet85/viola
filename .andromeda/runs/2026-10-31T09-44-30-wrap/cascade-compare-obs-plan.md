# Cascade compare — obs-plan · wrap · 2026-10-31-sanitised-error-surfaces
E items 18 · held 10 · lost at classify 4 · lost at strong 2 · beside a placed fact 0 · cascade only 6 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### obs-scope-summary#49.1 · lost at classify · .andromeda/obs-plan-detail/obs-scope-summary.md:499
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 499
    section: "## 1. Obs Scope Summary"
    change: Higher-`v` rejections are `channel-response{error_code:-32602, detail:"unsupported-version"}` alone; the code also carries `detail:"invalid-params"` (`send`'s invalid params, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`) and `detail:"release-from-driver"` (`release` carrying `from`), so the count reads `detail` and not the code.
    sidecar: The higher-`v` rejection signal reads `channel-response`'s `detail:"unsupported-version"`; the `-32602` over-count is retired now the line tells the causes apart (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Expected amendments carries obs-plan §5 as `viola.compat.v_rejected` from `channel-response{error_code:-32602, detail:"unsupported-version"}`, the over-count retired, and its Symbols / APIs names the three causes; this line still closes on the code not being a higher-`v` count alone with no way to separate them.

### obs-scope-summary#22.5 · lost at classify · .andromeda/obs-plan-detail/obs-scope-summary.md:215
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 215
    section: "## 1. Obs Scope Summary"
    change: Keep the codes-only sentence and add that a peer's reply is screened before it reaches a result, a `refusal` that is no code word (`viola_core::is_code_word`) reading `unknown`, a string `detail` that is no code word reading `null`, and `wrapper-fault`'s `detail` being the closed object `viola_channel::sanitised_fault` reads (the fixed `message` of the five codes, `data` null or closed values; a code outside the five reads `message` `wrapper fault`, `data` null); a code word outside the closed list stays visible, its exit `unknown`'s 14.
    sidecar: The `mcp` surface's codes-only note states the screen on a peer's reply, `is_code_word` and `sanitised_fault` (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs gives `of_reply`'s screen and `sanitised_fault`, replacing both `fault_detail` copies, and its Coverage of new surfaces names the fixed `wrapper fault` word as new on `--json` and MCP; the line states the rule without the mechanism that now holds it against a peer.

### obs-scope-summary#22.3 · lost at strong · .andromeda/obs-plan-detail/obs-scope-summary.md:212
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 212
    section: "## 1. Obs Scope Summary"
    change: To the `CappedLines` sentence add that such a line before `initialize` exits the process 1 with both streams empty and the same one record; 11 is the unit child's own code, not `viola mcp`'s exit.
    sidecar: The `viola mcp` stdin cut's exit before `initialize` is stated as measured, exit 1 with both streams empty (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Spec claims disproved measures this through `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`, and its Expected amendments carries the stdin clause's exit for obs-plan §7; the surface entry states the stderr and the record and leaves the exit unstated.

### pii-scrubbing-compliance#1.11 · lost at strong · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 20
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: Add this row after line 20 | A channel peer's reply (`refusal`, `detail`, an error's `message` and `data`) | Untrusted; may carry a path or upstream text, so the Medium rule applies | no process-log line carries it; reaches a CLI `--json` document, an MCP error or human text only screened | `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first) and `viola_channel::sanitised_fault`, read by `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`) — a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, an error keeps its integer `code`, its `message` is that code's fixed text (`wrapper fault` for a code outside the five) and its `data` holds only closed values or null; a code word outside the closed list stays visible by design; asserted by `reply_of_reads_the_three_shapes`, `of_reply_maps_each_reply_shape`, `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail` and `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` |
    sidecar: §8 PII table gains a peer's reply as a data type, screened by `is_code_word` and `sanitised_fault` before any document or text (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs and Coverage of new surfaces add the screen on `reply_of` and `of_reply` (PII redacted, no line added), and its Outcome states a peer's fields reach no document or text; the table names no such data type. The report states no classification, so "Untrusted" and the Medium rule are this proposal's reading, taken from the `claude agents --json` row and the path-valued clause of the table.

### pii-scrubbing-compliance#1.13 · lost at classify · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 22
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: Line 22's Scrubbing mechanism cell reads Closed kebab-case enums; `channel-response`'s `detail` is one of `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults, on both the server's and the client's line, absent on every other `channel-response` line, closed by the enum in `schemas/diag-line.v1.json` and naming no param or value; asserted by `answer_invalid_params_names_its_cause_and_no_param`, `reply_from_logs_the_cause_of_each_invalid_params` and `link_verbs_log_the_channel_pair_and_no_names`
    sidecar: §8 PII table's protocol-codes row names `channel-response`'s three closed `detail` words and their no-param rule (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Schema / config closes the field to the enum, its Symbols / APIs has `ResponseLine`'s `Drop` write `ProtocolError::cause`, and its Coverage reads PII no param value or name; this is the one log field the chunk adds near user data, and the row is where its class stands. D-obs-pii is not violated, since the words are fixed.

### span-trace-coverage#3.5 · lost at classify · .andromeda/obs-plan-detail/span-trace-coverage.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 32
    section: '## 4. Span / Trace Coverage'
    change: '- **The clients, a peer reply:** CLI `reply_of` and MCP `of_reply` screen a reply through `viola_core::is_code_word` and `viola_channel::sanitised_fault`, adding no span and no line, as landed (chunk 2026-10-31-sanitised-error-surfaces).'
    sidecar: 'span-trace-coverage: the reply screen of `reply_of` and `of_reply` recorded as adding no span and no line (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'Report Changes, Coverage of new surfaces: the `reply_of` / `of_reply` screen reads instrumentation n/a (no line added), PII redacted; the file records no-span operations as landed elsewhere in §4.'

## Held

- error-capture-reporting#2.1
- error-capture-reporting#5.2
- log-coverage#1.4
- metric-coverage#2.9
- obs-scope-summary#33.1
- pii-scrubbing-compliance#2.1
- span-trace-coverage#2.4
- span-trace-coverage#6.3
- span-trace-coverage#8.3
- span-trace-coverage#9.3

## Cascade only

- ci-integration#4.1
- obs-scope-summary#28.3
- obs-scope-summary#31.2
- span-trace-coverage#13.1
- telemetry-strategy#1.2
- telemetry-strategy#1.3
