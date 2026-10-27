# Task — security-plan · 2026-10-27-bounded-inputs-at-every-boundary

## The session
Bounded the one unbounded external reader left: `viola mcp`'s stdin. Each line now passes through a new crate-private `CappedLines` reader at `MAX_FRAME` before rmcp sees it. A longer line is refused whole and ends the session, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line that holds no byte of the line.
Bounded `replace_private_shared`'s fallback compare. It now reads the target through `take(bytes.len() + 1)` (new private `holds_exactly`) rather than the whole file. The outcome is unchanged and so are the callers.
Added a `deny.toml` `[[bans.features]]` ban on serde_json `unbounded_depth`, with a firing deny probe. The deny-probes verdict moved from `13/13` to `14/14 banned, control clean`.
Added a bounded-reader enumeration and census test (`tests/contract_bounded_readers.rs`). Extended the vt100 feed-panic chaos test so it asserts passthrough on the outer terminal: the bytes on Unix, and on every OS that no viola literal appears.
Measured that rmcp 3.4.1's stdio transport reads each line with an unbounded `read_until`, and that its codec's max length applies only to the writer side. v1-46 was marked implemented, and CI was green on all three OSes. No IPC method, MCP tool, endpoint, env var, CLI flag, exit code, error variant, dependency or rmcp feature changed. Nothing was retired.

## Reached files
- crates/viola-mcp/src/stdin.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-state/src/fs.rs
- schemas/diag-line.v1.json
- deny.toml
- scripts/deny-probes.sh
- tests/contract_bounded_readers.rs
- tests/chaos_feed_panic.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/verification-matrix.json

## Touches
- The Input Validation Constants list (body §Input Validation Constants and its detail twin) must now name two more reads that `MAX_FRAME` and its kin bound: `viola mcp`'s stdin lines (capped at `MAX_FRAME`) and `replace_private_shared`'s compare read (`bytes.len() + 1`).
- The Input Validation rows for the MCP surface: the MCP tool-inputs row and the hidden `viola mcp` home-resolution row are the nearest neighbours for a new row on the `viola mcp` stdin line bound. Those rows themselves do not move.
- The anti-pattern "NEVER enable serde_json's `unbounded_depth` feature" (§Security Anti-Patterns Input) is now enforced by a `deny.toml` feature ban that a deny probe proves, so it is no longer a bare rule.
- The Dependency Security `deny.toml` additions, `[[bans.features]]` list: it currently names only the rmcp bans and must gain the serde_json `unbounded_depth` deny.
- The deny-probes verdict count in dependency-security moves from `13/13` to `14/14 banned, control clean`. The rule behind the count is the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe. The claim that every ban is proven live by `scripts/deny-probes.sh` now covers the new ban.
- The bootstrap phase dep-audit-tooling-install (`[[bans.features]]` additions) now includes the serde_json ban. The input-validation-library-install line "`Read::take` wrappers on every external reader" was re-read and is unmoved and true.
- The Channel frames row's "never `unbounded_depth`" (body and input-validation) stands as written; the session made it gate-enforced.
- The anti-pattern "NEVER call `read_line` … without `Read::take(MAX_FRAME)`" is touched indirectly. It now reaches rmcp's own stdin reader, because viola bounds the reader it hands rmcp.
- The API anti-pattern "NEVER skip request size limits: `MAX_FRAME` on the channel" now also holds for the MCP stdio transport.
- The API Security MCP transport row: rmcp features stay exactly `server` + `transport-io`, and `scripts/rmcp-features.sh` still reads that. The stdin is now wrapped through rmcp's `(reader, writer)` transport, which `transport-io` already enables.
- The threat-model attack-surface vector "MCP over stdio" and the boundary-mcp group: a driver's over-long stdin line is now refused whole and ends the session. It is never truncated or dispatched.
- The **Parser surfaces** scope list (body §Input Validation): the MCP stdin line reader is a new bounded parser front. Judge whether the list must name it.
- The PTY output bytes (vt100) row: its claim that passthrough continues after a caught panic is now witnessed on the outer terminal. The Windows byte reading is owed to «Windows-only live measurements». The row's `panicked` / `oversize` detail wording was re-read and is already correct.
- The `replace_private` / `replace_private_shared` lines in data-protection (the compare behaviour) were re-read and are true. Only the read's bound is new, and it belongs at the Constants sites.
- The own-executable reads in authentication-authorization, data-protection and input-validation (the `checked_exe` / pin re-hash purpose) were re-read and are true. The new census names these reads as no external input.
- The NEVER-log floor (logging-redaction-wire): the new `mcp-stdin` rejection line carries no byte of the refused line, and an E2E canary checks this in every home-level file.
- Error Handling: no error variant or MCP `isError` shape was added. An oversize cut exits like a stdin close: 0 for an initialized session, and `ServeError::Initialize` (exit 11) before `initialize` completes.
- The v1-46 security criterion (every external reader size-capped, depth-limited and closed-typed) is now implemented, with its resume-cursor clause out of scope. Five boundary rows have a witness that never drives past the cap: the status-line command stdout, the child pipe drain for `--version` and `agents --json`, and the three `config.json` readers. The census alone holds those caps.

## New facts
- F1 · input-validation · `viola mcp` stdin: viola reads each line through `CappedLines` at `MAX_FRAME`, counting the `\n`, before rmcp sees it. A line of exactly the cap passes. A longer line releases no byte and is refused whole, never truncated or skipped, and it ends the session as a stdin close does.
- F2 · input-validation · On that cut `viola mcp` writes one WARN `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` holding no byte of the line. It exits 0 for an initialized session and with `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes. No test drives that pre-initialize path.
- F3 · input-validation · Rationale: rmcp 3.4.1's stdio transport reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` serves only the writer. A bound on rmcp's input therefore belongs to the reader viola hands it, never to the codec.
- F4 · input-validation · `replace_private_shared`'s fallback compare reads the target through `take(bytes.len() + 1)` (`holds_exactly`), never the whole file. Its outcome equals a whole-file compare for an equal, a longer, a shorter, a different and an absent target.
- F5 · dependency-security · `deny.toml` holds a `[[bans.features]]` table banning serde_json `unbounded_depth`. Two things prove it: the deny-probes row `serde_json-unbounded-depth` and the test `serde_json_unbounded_depth_is_banned_in_deny_toml`.
- F6 · input-validation · A bounded-reader enumeration, `tests/contract_bounded_readers.rs`, runs as an E2E `contract_` binary. Its `BOUNDARIES` table names each external boundary's bounded read and its witness. Its census (`product_readers_are_each_bounded_or_named`) requires every product reader call to be bounded in-statement or named in `EXCEPTIONS`, and a self-check reads a planted unbounded read as a hit.
- F7 · input-validation · The census's `EXCEPTIONS` rows name the own-executable reads as no external input. These are the reads that hash viola's own binary for the pinned copy.
- F8 · threat-model-summary · On the MCP-over-stdio boundary, an over-long line from the driver ends the MCP session after the earlier lines are answered. It is never dispatched in part.
- F9 · input-validation · After a caught vt100 feed panic, `chaos_feed_panic_refuses_send_and_keeps_passthrough` witnesses the human passthrough on the outer terminal. On Unix the child's bytes reach it. On every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` appears. The Windows byte reading is owed to «Windows-only live measurements» (a PROVISIONAL CARRY).

## Untouched
- IPC access control, the peer check and client server verification: no channel method, endpoint, pid or start-time check changed.
- Paste text and dialog free-text rows, and their anti-patterns: the paste rule and `validate_paste_text` are unchanged. Their existing client and wrapper witnesses were only re-run.
- Hook stdin row: the `MAX_FRAME` cap and the fail-open behaviour are unchanged.
- Home strict-modes, pinned-copy placement and re-hash rows: `replace_private_shared`'s callers and its write path are unchanged. Only its compare read is bounded.
- Secret Management and the CI artifact canary scan: no secret, seam or scan class changed.
- The screen-poisoning detail (`panicked` for a caught panic, `oversize` for a dropped copy) in security-plan and input-validation is already correct. The disproved line is architecture's, not this plan's.
- zizmor's cache-poisoning lines in Dependency Security and the v1.x release prerequisites are another subject, despite the shared word.
- Pinning, `Cargo.lock` and update policy: no dependency was added or bumped, and the rmcp range is unchanged.
- The ConPTY vendor audit, the DLL search restriction and the sideload rows are untouched.
- The statusline source and shell-out rows: the stdout cap is unchanged and only listed in the enumeration.
- Error Handling's external-response codes and the channel `-32600` oversize format: no new error code or variant.
- Data lifecycle and `events.ndjson` retention are untouched.
