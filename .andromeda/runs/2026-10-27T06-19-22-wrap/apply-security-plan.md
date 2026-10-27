# Apply — security-plan

## Verdicts
- fanout-security-plan--cascade.md 2 · already stands · .andromeda/security-plan.md:43–43 · «each stdin line is capped at `MAX_FRAME` before rmcp reads it, an over-long line refused whole and ending the session, never dispatched in part»
- fanout-security-plan.md 1 · already stands · .andromeda/security-plan.md:142–142 · «MCP stdin (`viola mcp`'s transport, the driver's JSON-RPC lines) — each line capped at `MAX_FRAME`, its `\n` counted, by the reader viola hands rmcp»
- fanout-security-plan.md 2 · already stands · .andromeda/security-plan.md:178–178 · «`viola mcp`'s stdin lines, sent text, child output, ndjson lines, `config.json` and the stamps read; viola's reads of its own executable are no external input); `replace_private_shared`'s fallback compare reads its target through `take(bytes.len() + 1)`, never the whole file»
- fanout-security-plan--cascade.md 12 · already stands · .andromeda/security-plan.md:178–178 · «`answer` input, `viola mcp`'s stdin lines, sent text»
- fanout-security-plan.md 3 · already stands · .andromeda/security-plan.md:454–454 · «NEVER hand rmcp a `viola mcp` stdin reader that does not cap each line at `MAX_FRAME`, nor truncate or skip an over-long line instead of ending the session»
- fanout-security-plan.md 4 · already stands · .andromeda/security-plan.md:472–472 · «NEVER skip request size limits: `MAX_FRAME` on the channel and on each `viola mcp` stdin line.»
- fanout-security-plan.md 5 · already stands · .andromeda/security-plan.md:257–257 · «`serde_json` denies `unbounded_depth` (every parse of external bytes keeps serde_json's default recursion limit), its firing proven by `scripts/deny-probes.sh`.»
- fanout-security-plan.md 6 · already stands · .andromeda/security-plan.md:455–455 · «NEVER enable serde_json's `unbounded_depth` feature; `deny.toml` bans it (§Dependency Security).»
- fanout-security-plan--cascade.md 1 · already stands · .andromeda/security-plan-detail/threat-model-summary.md:108–108 · «Each stdin line reaches rmcp only through viola's `CappedLines` reader at `MAX_FRAME`, its `\n` counted: an over-long line from the driver is refused whole and ends the MCP session after the earlier lines are answered»
- fanout-security-plan--cascade.md 3 · applied · .andromeda/security-plan-detail/input-validation.md:29#1
- fanout-security-plan--cascade.md 4 · already stands · .andromeda/security-plan-detail/input-validation.md:31–31 · «**On the cut:** after the session ends, `serve` writes one WARN `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` through `obs_event!`, holding no byte of the line»
- fanout-security-plan--cascade.md 5 · already stands · .andromeda/security-plan-detail/input-validation.md:32–32 · «**Why the reader, not the codec:** rmcp 3.4.1's stdio transport reads each line with an unbounded `read_until`»
- fanout-security-plan--cascade.md 7 · already stands · .andromeda/security-plan-detail/input-validation.md:70–70 · «**Witness:** after a caught feed panic, `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the outer terminal»
- fanout-security-plan--cascade.md 8 · already stands · .andromeda/security-plan-detail/input-validation.md:96–96 · «`viola mcp`'s stdin lines (`CappedLines` at `MAX_FRAME`, the newline counted, an over-long line ending the session)»
- fanout-security-plan--cascade.md 9 · already stands · .andromeda/security-plan-detail/input-validation.md:97–97 · «**Bounded by its own length, not `MAX_FRAME`:** `replace_private_shared`'s fallback compare reads its target through `take(bytes.len() + 1)` (`holds_exactly`), never the whole file»
- fanout-security-plan--cascade.md 10 · already stands · .andromeda/security-plan-detail/input-validation.md:98–98 · «**Enumerated and censused:** `tests/contract_bounded_readers.rs` (an E2E `contract_` binary) names each external boundary's bounded read and its witness in `BOUNDARIES`»
- fanout-security-plan--cascade.md 11 · already stands · .andromeda/security-plan-detail/input-validation.md:99–99 · «**Not external input:** viola's reads of its own executable for the pinned copy are no external input; the census's `EXCEPTIONS` rows name them.»
- fanout-security-plan--cascade.md 13 · already stands · .andromeda/security-plan-detail/dependency-security.md:20–20 · «`14/14 banned, control clean`, counted as the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe»
- fanout-security-plan--cascade.md 14 · already stands · .andromeda/security-plan-detail/dependency-security.md:14–14 · «`deny.toml`'s `[[bans.features]]` also bans serde_json's `unbounded_depth` feature, its reason that every parse of external bytes keeps serde_json's default recursion limit»

## Edits
- line: .andromeda/security-plan-detail/input-validation.md:29
  old: «- **Boundary:** `viola mcp` stdin (the driver's JSON-RPC lines)»
  new: «- **Boundary:** MCP stdin (`viola mcp`'s transport, the driver's JSON-RPC lines)»

## Patterns
(none)
