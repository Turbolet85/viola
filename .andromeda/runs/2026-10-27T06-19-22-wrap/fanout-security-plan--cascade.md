proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 101
    section: '## Threat Model Summary'
    change: 'add under the MCP-over-stdio vector''s **Trust boundary:** list a sub-bullet: `    - Each stdin line reaches rmcp only through viola''s `CappedLines` reader at `MAX_FRAME` (the `\n` counted): an over-long line from the driver is refused whole and ends the MCP session after the earlier lines are answered, never truncated, skipped or dispatched in part, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN holding no byte of it.`'
    sidecar: 'threat-model-summary boundary-mcp: the MCP-over-stdio trust boundary gains the stdin line cap (an over-long driver line ends the session, never dispatched in part)'
    rationale: 'the report''s Symbols / APIs (`serve` hands rmcp `CappedLines::new(stdin, MAX_FRAME)`) and Outcome (a line past `MAX_FRAME` ends the session after the earlier lines are answered, exit 0, one `parse-rejected` line holding no byte)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:85
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 43
    section: '## Threat Model Summary'
    change: 'in the **Vector:** MCP over stdio line, after «tool input schemas and the `ViolaName` newtype check the input;» add «each stdin line passes viola''s `MAX_FRAME` line cap before rmcp reads it, an over-long line refused whole and ending the session, never dispatched in part;»'
    sidecar: 'body Threat Model Summary: the MCP-over-stdio vector''s broad line names the stdin line cap its boundary-mcp group now details'
    rationale: 'the broad line the boundary-mcp group details; the group gains the stdin bound (report Symbols / APIs, `serve`; Decisions & corrections: refused whole, never truncated or skipped)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:69
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 28
    section: '## Input Validation'
    change: 'add after the MCP tool inputs entity a new entity: `- **Boundary:** `viola mcp` stdin (the driver''s JSON-RPC lines)` / `  - **What to validate:** **Line cap:** each line passes through `viola_mcp::stdin::CappedLines` at `MAX_FRAME`, the `\n` counted, before rmcp sees it; a line of exactly the cap passes whole; a longer line releases no byte and is refused whole, never truncated or skipped, and the reader then reads as end of stream, so the session ends as a stdin close does; at the inner end a held unterminated line within the cap is released.` / `  - **How (stack-specific):** `viola-mcp` `stdin::CappedLines` (crate-private), handed to rmcp 3.4.1 as `(CappedLines::new(stdin, MAX_FRAME), stdout)` through the `(reader, writer)` transport `transport-io` already enables (no rmcp feature added); `viola_mcp::serve`.`'
    sidecar: 'input-validation: new Boundary entity for `viola mcp` stdin, each line capped at `MAX_FRAME` by `CappedLines`, an over-long line refused whole and ending the session'
    rationale: 'the report''s Symbols / APIs (`CappedLines`, `stdin.rs:17-29`, and `serve`''s transport) and Expected amendments (security-plan §Input Validation gains a `viola mcp` stdin row, `input-validation.md:34` / the MCP tool inputs row its nearest neighbour; `mcp-stdin` 0 sites, new text)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:10
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 28
    section: '## Input Validation'
    change: 'append to the new `viola mcp` stdin entity''s **What to validate:** «**On the cut:** after the session ends `serve` writes one WARN `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` through `obs_event!`, holding no byte of the line, and returns what the session gave: exit 0 for an initialized session, `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes; no test drives that pre-initialize path.»'
    sidecar: 'input-validation: the `viola mcp` stdin entity records the one `parse-rejected{mcp-stdin, oversize}` line and the exit on a cut (0 initialized, 11 before `initialize`, that path untested)'
    rationale: 'the report''s Symbols / APIs (`reject_oversize_line`, `lib.rs:93-102`; `Ok(())` or `ServeError::Initialize`) and Deviations 1 (the pre-initialize cut driven by no test)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:57
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 28
    section: '## Input Validation'
    change: 'append to the new `viola mcp` stdin entity''s **What to validate:** «**Why the reader, not the codec:** rmcp 3.4.1''s stdio transport reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` serves only the writer, so a bound on rmcp''s input belongs to the reader viola hands it, never to the codec.»'
    sidecar: 'input-validation: the `viola mcp` stdin entity carries why the bound sits on the reader (rmcp 3.4.1 reads lines with an unbounded `read_until`; its codec max length is writer-only)'
    rationale: 'the report''s Cross-project / external claims (rmcp 3.4.1 `async_rw.rs` lines 125-140) and Decisions & corrections (a bound on rmcp''s input belongs to the reader viola hands it)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:28
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 140
    section: '## Input Validation'
    change: 'add after the MCP tool inputs line a broad line: «MCP stdin (`viola mcp`) — each line capped at `MAX_FRAME` before rmcp reads it; an over-long line is refused whole, never truncated or skipped, and ends the session as a close does, with one `parse-rejected` line holding no byte of it.»'
    sidecar: 'body Input Validation: a broad line for the new `viola mcp` stdin boundary its detail entity holds'
    rationale: 'the body keeps a broad line per boundary its detail holds; the new `viola mcp` stdin entity (report Expected amendments: security-plan §Input Validation gains a `viola mcp` stdin row) has none'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:41
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 65
    section: '## Input Validation'
    change: 'append to the PTY output bytes (vt100) entity''s **What to validate:** «**Witness:** after a caught feed panic `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the human passthrough on the outer terminal: on Unix the child''s bytes (`e4 b8 ad`) reach it; on every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` appears; the Windows byte reading is owed to «Windows-only live measurements» (a PROVISIONAL CARRY).»'
    sidecar: 'input-validation PTY output bytes (vt100): the passthrough after a caught feed panic is witnessed on the outer terminal (Unix bytes, every-OS literal absence; Windows bytes owed)'
    rationale: 'the report''s Coverage of new surfaces (the tui passthrough after a feed panic, `tests/chaos_feed_panic.rs:109-130`) and Outcome (a11y criterion met; Windows reads the literal absence only, owed to «Windows-only live measurements» by the P5 card''s CARRY)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:53
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 90
    section: '## Input Validation'
    change: 'in the **Constants:** `MAX_FRAME` list, after «`answer` input,» add «`viola mcp`''s stdin lines (`CappedLines` at `MAX_FRAME`, the `\n` counted, an over-long line ending the session),»'
    sidecar: 'input-validation Constants: the `MAX_FRAME` list names `viola mcp`''s stdin lines'
    rationale: 'the report''s Expected amendments (`constants` security-plan 2: `input-validation.md:90` changes, the list gains the reads) and Symbols / APIs (`serve`''s transport `CappedLines::new(stdin, MAX_FRAME)`)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:41
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 90
    section: '## Input Validation'
    change: 'append to the **Constants:** entity «**Bounded by its own length, not `MAX_FRAME`:** `replace_private_shared`''s fallback compare reads the target through `take(bytes.len() + 1)` (`holds_exactly`), never the whole file; its outcome equals a whole-file compare for an equal, a longer, a shorter, a different and an absent target.»'
    sidecar: 'input-validation Constants: `replace_private_shared`''s compare read is bounded at `bytes.len() + 1` by `holds_exactly`, its outcome a whole-file compare''s'
    rationale: 'the report''s Symbols / APIs (`holds_exactly`, `fs.rs:359-368`) and Expected amendments (the Constants list gains `replace_private_shared`''s compare read; `input-validation.md:90` changes)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:12
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 90
    section: '## Input Validation'
    change: 'append to the **Constants:** entity «**Enumerated and censused:** `tests/contract_bounded_readers.rs` (an E2E `contract_` binary) names each external boundary''s bounded read and its witness in `BOUNDARIES`; its census `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in-statement or named in `EXCEPTIONS`, and a self-check reads a planted unbounded read as a hit; five rows (the statusline command''s stdout, the child pipe drain of `--version` and `agents --json`, the three `config.json` readers) name a witness that never drives past the cap, their cap held by the census alone.»'
    sidecar: 'input-validation Constants: the bounded-reader enumeration and census (`tests/contract_bounded_readers.rs`) hold every product reader bounded or named, five rows'' caps held by the census alone'
    rationale: 'the report''s Coverage of new surfaces (the enumeration and census, its four tests, and the five rows whose witness drives nothing past the cap, deviation 2)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:52
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 90
    section: '## Input Validation'
    change: 'append to the **Constants:** entity «**Not external input:** the census''s `EXCEPTIONS` rows name the own-executable reads, the reads that hash viola''s own binary for the pinned copy, as no external input.»'
    sidecar: 'input-validation Constants: the own-executable reads are named no external input by the census''s `EXCEPTIONS`'
    rationale: 'the report''s Expected amendments (the own-executable reads named as no external input, new text beside Constants; `own-exe` sites no-change) and the new-text listing (`EXCEPTIONS`, `contract_bounded_readers.rs:392-405`)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:41
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan.md
    near: 176
    section: '## Input Validation'
    change: 'in the **Constants:** line, after «`answer` input,» add «`viola mcp`''s stdin lines,» and after «the stamps read)» add «; `replace_private_shared`''s compare read is bounded by its own bytes (`take(bytes.len() + 1)`)»'
    sidecar: 'body Input Validation Constants: names `viola mcp`''s stdin lines and the bounded compare read of `replace_private_shared`'
    rationale: 'the report''s Expected amendments (`constants` security-plan 2: `security-plan.md:176` changes, the list gains the two reads)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:41
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 14
    section: '## Dependency Security'
    change: 'replace «(probe `channel-tokio-edge` and its channel control, `13/13 banned, control clean`)» with «(probe `channel-tokio-edge` and its channel control; `14/14 banned, control clean`, by the rule the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe)»'
    sidecar: 'dependency-security: the deny-probes verdict moves 13/13 → 14/14 banned, control clean (the `probes=(` rows plus the planted channel probe)'
    rationale: 'the report''s Counts / qualifiers moved (`probe-count`: `security-plan-detail/dependency-security.md:14` changes, `14/14`, the rule the `probes=(` rows now 13 plus `channel-tokio-edge`) and Harness / gate surface (the `serde_json-unbounded-depth` row, `deny-probes.sh:32`)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 14
    section: '## Dependency Security'
    change: 'add after the arch-bans entity: «- `deny.toml` also holds a `[[bans.features]]` table banning serde_json''s `unbounded_depth` feature (reason: every parse of external bytes keeps serde_json''s default recursion limit), proven by the deny-probes row `serde_json-unbounded-depth` and by the test `serde_json_unbounded_depth_is_banned_in_deny_toml`.»'
    sidecar: 'dependency-security: the serde_json `unbounded_depth` feature ban in `deny.toml`, proven by its deny-probes row and its contract test'
    rationale: 'the report''s Schema / config (`deny.toml:58-62`, the `[[bans.features]]` table and its reason), Harness / gate surface (`deny-probes.sh:32`) and Coverage of new surfaces (the serde_json depth ban''s probe and test)'
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:16
    fact: F5
note: 'not mine (no picked entity): body security-plan.md:452 «NEVER enable serde_json''s `unbounded_depth` feature.» is now a `deny.toml` feature ban with a firing probe (report: change), the dependency-security `[[bans.features]]` list (rmcp bans only) gains the serde_json deny, body :174 Parser surfaces may name the MCP stdin line reader, and input-validation#1.4 hook stdin was ruled out as unchanged'
