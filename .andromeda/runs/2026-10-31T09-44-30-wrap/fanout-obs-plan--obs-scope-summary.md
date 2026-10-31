proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 354
    section: "## 1. Obs Scope Summary"
    change: The scrub rule on `channel-request` / `channel-response` reads method, `id` / `corr`, result class (`ok` / `refusal` / `error` code), `sender` version and, on a `channel-response` answering one of the three `-32602` faults only, the closed `detail` (`unsupported-version` · `invalid-params` · `release-from-driver`), absent on every other line; never `params`, a param's name or result bodies.
    sidecar: Vector 1's scrub rule on the channel pair admits the closed three-word `detail` on a `-32602` `channel-response`; its field list no longer reads as excluding it (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs has `ResponseLine`'s `Drop` write `detail = fault.and_then(ProtocolError::cause)` on both sides, and its Schema / config closes `channel-response`'s `detail` to the three words; the line's "only" list omits the field.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 499
    section: "## 1. Obs Scope Summary"
    change: Higher-`v` rejections are `channel-response{error_code:-32602, detail:"unsupported-version"}` alone; the code also carries `detail:"invalid-params"` (`send`'s invalid params, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`) and `detail:"release-from-driver"` (`release` carrying `from`), so the count reads `detail` and not the code.
    sidecar: The higher-`v` rejection signal reads `channel-response`'s `detail:"unsupported-version"`; the `-32602` over-count is retired now the line tells the causes apart (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Expected amendments carries obs-plan §5 as `viola.compat.v_rejected` from `channel-response{error_code:-32602, detail:"unsupported-version"}`, the over-count retired, and its Symbols / APIs names the three causes; this line still closes on the code not being a higher-`v` count alone with no way to separate them.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 215
    section: "## 1. Obs Scope Summary"
    change: Keep the codes-only sentence and add that a peer's reply is screened before it reaches a result, a `refusal` that is no code word (`viola_core::is_code_word`) reading `unknown`, a string `detail` that is no code word reading `null`, and `wrapper-fault`'s `detail` being the closed object `viola_channel::sanitised_fault` reads (the fixed `message` of the five codes, `data` null or closed values; a code outside the five reads `message` `wrapper fault`, `data` null); a code word outside the closed list stays visible, its exit `unknown`'s 14.
    sidecar: The `mcp` surface's codes-only note states the screen on a peer's reply, `is_code_word` and `sanitised_fault` (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs gives `of_reply`'s screen and `sanitised_fault`, replacing both `fault_detail` copies, and its Coverage of new surfaces names the fixed `wrapper fault` word as new on `--json` and MCP; the line states the rule without the mechanism that now holds it against a peer.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 212
    section: "## 1. Obs Scope Summary"
    change: To the `CappedLines` sentence add that such a line before `initialize` exits the process 1 with both streams empty and the same one record; 11 is the unit child's own code, not `viola mcp`'s exit.
    sidecar: The `viola mcp` stdin cut's exit before `initialize` is stated as measured, exit 1 with both streams empty (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Spec claims disproved measures this through `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`, and its Expected amendments carries the stdin clause's exit for obs-plan §7; the surface entry states the stderr and the record and leaves the exit unstated.
note: Not proposed, outside this file or anchored — .andromeda/obs-plan.md:527 still has `mcp`'s main-thread catch site writing `process-exit`, which the report's §7 amendment and its mcp-exit-line disproof retire though its sweep rows (.andromeda/obs-plan.md:523, .andromeda/obs-plan-detail/error-capture-reporting.md:39) do not list that line; .andromeda/obs-plan-detail/obs-scope-summary.md:190 labels as max the 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms figures the report's Gates calls mean, left because the line anchors them to its dated gate run.
