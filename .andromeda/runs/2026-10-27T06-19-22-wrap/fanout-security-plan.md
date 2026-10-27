proposals:
  - detector: D-security-input
    severity: warning
    section: Input Validation
    change: 'Add a body row beside the "MCP tool inputs" row: MCP stdin (`viola mcp`''s transport) — each line capped at `MAX_FRAME`, its newline counted, by the reader viola hands rmcp (rmcp 3.4.1 reads stdin lines unbounded and its codec bounds only the writer); a longer line releases no byte, is refused whole (never truncated, never skipped) and ends the session as a stdin close does, recorded as one `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line holding no byte of it.'
    sidecar: '§Input Validation gains the `viola mcp` stdin row: each line capped at MAX_FRAME by `CappedLines`, an over-long line refused whole and ending the session with one parse-rejected{mcp-stdin, oversize} (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    rationale: 'Report Changes, Symbols / APIs: `viola_mcp::stdin::CappedLines` and `viola_mcp::serve` now bound the MCP stdin at MAX_FRAME and write one parse-rejected{mcp-stdin, oversize, count:1}; Cross-project claims: rmcp 3.4.1 `read_until` unbounded, `new_with_max_length` writer-only; Expected amendments: "security-plan §Input Validation — a `viola mcp` stdin row" with `mcp-stdin` at 0 master sites (new text). The invariant itself holds (the surface is validated per §Input Validation, Coverage of new surfaces ✓), so this records the new bound rather than an escalation.'
    basis: 'crates/viola-mcp/src/stdin.rs:17-29'
  - detector: D-security-input
    severity: warning
    section: Input Validation
    change: 'In the **Constants:** line, the `MAX_FRAME` list of capped external reads gains `viola mcp`''s stdin lines; add that `replace_private_shared`''s fallback compare reads its target through `take(bytes.len() + 1)`, never the whole file, and that viola''s reads of its own executable are no external input (the bounded-reader census''s named exceptions).'
    sidecar: '§Input Validation Constants: the MAX_FRAME list gains the MCP stdin; the replace compare read bounded at bytes.len()+1; the own-executable reads named as no external input (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    rationale: 'Report Expected amendments: "the Constants list gaining `viola mcp`''s stdin and `replace_private_shared`''s compare read; the own-executable reads named as no external input", search `constants` names security-plan.md:176 as a change site; Symbols / APIs: `holds_exactly` reads through `take(bytes.len() + 1)`; Counts: `EXCEPTIONS` 2 rows at tests/contract_bounded_readers.rs:392-405.'
    basis: '.andromeda/security-plan.md:176'
  - detector: D-security-input
    severity: warning
    section: Security Anti-Patterns › Input
    change: 'Extend the `read_line` / `read_to_end` without `Read::take(MAX_FRAME)` ban: and NEVER hand rmcp a stdin reader that does not cap each line at `MAX_FRAME`, nor truncate or skip an over-long line instead of ending the session.'
    sidecar: '§Security Anti-Patterns › Input: the bounded-reader ban names the `viola mcp` stdin line cap (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    rationale: 'The body ban restates which external readers must be capped and omits the MCP stdin the chunk bounded (report Symbols / APIs, `serve`; Decisions & corrections: an over-long MCP line is refused whole, never truncated, never skipped).'
    basis: 'crates/viola-mcp/src/lib.rs:93-102'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: warning
    section: Security Anti-Patterns › API
    change: 'NEVER skip request size limits: `MAX_FRAME` on the channel and on each `viola mcp` stdin line.'
    sidecar: '§Security Anti-Patterns › API: the request size limit names the MCP stdin line cap beside the channel (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    rationale: 'The API ban names MAX_FRAME on the channel only; the MCP transport, whose feature set §API Security covers, now has its own request size limit (report Symbols / APIs, `CappedLines::new(stdin, MAX_FRAME)` in `serve`).'
    dependent-of: D-security-input
  - detector: D-security-deps
    severity: warning
    section: Dependency Security
    change: 'Under **`deny.toml` additions**, `[[bans.features]]` gains a row: `serde_json` denies `unbounded_depth` (every parse of external bytes keeps serde_json''s default recursion limit), its firing proven by the `serde_json-unbounded-depth` row of `scripts/deny-probes.sh`.'
    sidecar: '§Dependency Security: deny.toml [[bans.features]] gains serde_json unbounded_depth, proven by a deny-probes row (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    rationale: 'Report Changes, Schema / config: `deny.toml` :58-62 adds the serde_json `unbounded_depth` feature ban; Harness / gate surface: deny-probes gains `serde_json-unbounded-depth` (:32); Expected amendments: "security-plan §Dependencies — the serde_json `unbounded_depth` feature ban". No dependency added (Dependencies: none), so the invariant holds; the body list of feature bans names rmcp only.'
    basis: 'deny.toml:58-62'
  - detector: D-security-deps
    severity: warning
    section: Security Anti-Patterns › Input
    change: 'NEVER enable serde_json''s `unbounded_depth` feature: `deny.toml` bans it in `[[bans.features]]`, and `scripts/deny-probes.sh` proves the ban fires.'
    sidecar: '§Security Anti-Patterns › Input: the serde_json unbounded_depth ban is now enforced by deny.toml with a firing probe (chunk 2026-10-27-bounded-inputs-at-every-boundary).'
    rationale: 'Report Expected amendments, search `depth-ban`: security-plan.md:452 "NEVER enable serde_json''s `unbounded_depth` feature." — change: now a `deny.toml` feature ban with a firing probe.'
    basis: '.andromeda/security-plan.md:452'
    dependent-of: D-security-deps
note: 'D-security-auth: no identity/token/key surface touched; Retired: none; no D-new-detail (every section has a category file). The detail-file moves — input-validation.md:90 (Constants), dependency-security.md:14 (deny-probes 13/13 to 14/14), and an MCP stdin row in input-validation.md — are the cascade''s. security-plan.md:136 and :160 stay true (report: no-change).'
