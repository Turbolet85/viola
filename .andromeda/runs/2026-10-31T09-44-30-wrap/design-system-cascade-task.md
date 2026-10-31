# Task — design-system · 2026-10-31-sanitised-error-surfaces

## The session
Closed the error bodies of CLI `--json`, MCP results and channel error data to codes and fixed messages: no path and no upstream text reaches them, and a peer's `message`, `data`, `refusal` or `detail` reaches no document or text.
Added `viola_core::is_code_word` and `viola_channel::sanitised_fault`; both clients, `reply_of` in `src/cmd/client.rs` and `of_reply` in `crates/viola-mcp/src/result.rs`, screen a peer's reply through them.
Retired `viola_agent_claude::Refusal`, folded into `AgentError` with its messages and outputs unchanged, and both private `fault_detail` copies.
Closed `channel-response`'s `detail` in `schemas/diag-line.v1.json` to three words; the seven `ProtocolError` wire bodies and `v` 1 are unchanged.
Added three test binaries, `contract_error_bodies`, `channel_error_bodies` and `cli_error_bodies`, plus the check in `tests/support/events.rs`, which replace the after-every-test helper test-plan described and which was never built.
Measured and disproved master claims: `viola mcp`'s exit 11 (it exits 1), `CoreError` (absent from the code), and one `--json` document on an oversize `viola answer` (a fixed stderr line, no document).

## Reached files
- tests/contract_error_bodies.rs
- tests/channel_error_bodies.rs
- tests/cli_error_bodies.rs
- crates/viola-core/src/lib.rs
- crates/viola-channel/src/lib.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/server.rs
- schemas/diag-line.v1.json
- src/cmd/client.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-agent-claude/src/lib.rs
- src/run/mod.rs
- src/cmd/run.rs
- src/cmd/verify.rs
- tests/support/events.rs
- Cargo.toml
- .config/nextest.toml
- crates/viola-e2e/tests/mcp_tools.rs
- src/cmd/hook/statusline.rs
- src/run/version_gate.rs
- src/run/budget.rs
- src/run/dialog.rs
- src/run/env.rs
- tests/contract_bounded_readers.rs
- viola-0.1.0/verification-matrix.json

## Touches
- Exit-code phraseology, the row for exit 14 (`.andromeda/design-system-detail/surface.md:147`): `unknown`'s detail is a code word or nothing, no longer opaque free text.
- The `viola send` refusal group (`.andromeda/design-system-detail/surface.md:81`): the same change to `unknown`'s detail, as named by the report's `code-word` pattern.
- Semantic Colors, the `RefusalReason::unknown` row (`.andromeda/design-system-detail/color-palette.md:102`): a code word or nothing, no longer opaque free text.
- The stderr refusal form `unable  <name>  <reason>  <detail>` in Streams: its `<reason>` and `<detail>` now pass the code-word screen when they come from a peer's reply.
- The `--json` paragraph of Streams (the whole result is one JSON document with the typed exit code): the oversize `viola answer` and unusable `viola send` paths at exit 2 are outside it, measured.
- Exit-code phraseology, the row for exit 2 (clap usage): the same two paths also exit 2, with a fixed stderr line.
- Exit-code phraseology, the row for exit 20 and any `--json` shape stated for `wrapper-fault`: its `code`, `message` and `data` are now closed by `sanitised_fault`.
- The cli ban on printing upstream text, paths, pids or anyhow chains in errors or hints: it now also holds on `--json` documents, MCP results and channel error data, and has a census and witnesses behind it.
- Platform-Specific Notes, stack traces and fixed error messages: the census test reads that no `thiserror` message interpolates a value.
- Component pattern 2, faults and `unknown` get no hint: re-read green by the integration pins, claim unmoved.
- The `--json` no-hint rule and the ban on SGR under `--json`: re-read green (no SGR in a document, stderr empty on the failures the witness drives), claims unmoved.
- Component pattern 5, `verify` and `run` refusals: their callers now match `AgentError::BatchScriptChild` and `AgentError::ProgramNotFound`-class errors instead of `Refusal`; the printed lines, including `measure`'s not-found pair and `start`'s `internal-error`, are unchanged, so only a text naming `Refusal` moves.
- Any group naming `viola_agent_claude::Refusal` or `Refusal::NotFound`: retired. `RefusalReason` and the private `Refusal` of `src/cmd/revive.rs` are other types and stay.
- Any group naming `fault_detail` or citing lines of `src/cmd/client.rs` or `crates/viola-mcp/src/result.rs`: `fault_detail` is gone and lines moved; no public signature moved.
- Any group citing lines of `crates/viola-core/src/lib.rs`, `crates/viola-channel/src/lib.rs`, `crates/viola-channel/src/client.rs`, `crates/viola-channel/src/server.rs`, `crates/viola-agent-claude/src/lib.rs`, `src/cmd/run.rs`, `src/cmd/verify.rs` or `src/run/mod.rs`: lines moved.
- Any group stating `viola mcp`'s exit: a stdin line past the frame cap before `initialize` exits 1, both streams empty. The exit 11 of `ServeError::Initialize` is the unit child's own code. The report's `exit-eleven` sweep names no design-system site, so a hit here is a new finding.
- Any group stating that `mcp` writes a `process-exit` line: measured, it writes none. The report names no design-system site.
- Any group listing `CoreError` among crate enums: it never existed in the code. The report names no design-system site.
- `budget-paused`'s exit 11 is a different code from `ServeError::Initialize`'s; a statement of it is not touched by the exit correction.
- `Refusal` as a bare token also reads `RefusalReason`; a mention of `RefusalReason` alone is not touched by the retirement.
- A test inventory or named test in any group: three new binaries, and five `BOUNDARIES` witnesses renamed in `tests/contract_bounded_readers.rs`.

## New facts
- F1 · surface · A code word is 1 to 64 bytes of lower-case letters, digits and hyphens with a lower-case letter first (`viola_core::is_code_word`); both clients screen a peer's refusal and detail through it.
- F2 · surface · A peer's `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is still no known reply.
- F3 · surface · A code word outside the closed refusal list stays visible in the document and in `refused: <word>`, by design, and exits 14 as `unknown` does.
- F4 · surface · `wrapper-fault`'s fields are closed: each of the five known codes carries its fixed `message`, `-32602` carries the unsupported-version body only with an integer `supported` and a version-text `wrapper`, the release-from-driver body only on that exact reason, else `invalid params` with null `data`; every other known code has null `data`.
- F5 · surface · A wrapper code outside the five reads `message` `wrapper fault` with null `data`: a new fixed output word on `--json` and MCP, while the human line `error: wrapper fault  <code>` is unchanged.
- F6 · surface · The exit-2 paths of an oversize `viola answer` and an unusable `viola send` write the fixed `UNUSABLE_RESPONSE` or `UNUSABLE_TEXT` line to stderr whatever `--json` says, with no document; `tests/contract_error_bodies.rs` names them in `NOT_ERROR_BODIES`.
- F7 · surface · `tests/cli_error_bodies.rs` witnesses each CLI `--json` failure it drives as one document, a literal exit, empty stderr, no ESC and no path or upstream text.
- F8 · surface · `tests/contract_error_bodies.rs` censuses the error writers: each CLI document and MCP result names a writer and a witness that stand, no product writer goes unnamed and no `thiserror` message interpolates.

## Untouched
- Brand Identity: personality, domain anchors, the readback box's three drawn states and the expression level.
- Color Palette hex values, the contrast table, Core Colors, Surface Scale, Text Hierarchy and Border Progression: no colour moved.
- Semantic Colors, every status row except `RefusalReason::unknown`: no other word or treatment moved.
- Typography, Spacing, Depth Strategy, Border Radius, Motion and Iconography: frontend inputs for 0.2.0 that the session does not reach.
- CLI colour tokens and the colour decision order: no styling rule moved.
- The human line for exit 20, `error: wrapper fault  <code>`: `src/human/mirror.rs` is unchanged; only the `--json` and MCP fields closed.
- `viola list`'s strip table, BAY line and escaping: no column or row form moved.
- Navigation Pattern and the `viola --help` rows: no verb added or renamed.
- Self-Validation Protocol, including the Token Test: no procedure moved.
- Surface: web-spa: still deferred to 0.2.0.
