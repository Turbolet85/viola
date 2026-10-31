# Task — a11y-plan · 2026-10-31-sanitised-error-surfaces

## The session
Changed: CLI `--json` documents, MCP results and channel error data now carry only codes and fixed messages from a peer's reply, no paths or upstream text, through `viola_core::is_code_word` and `viola_channel::sanitised_fault` in `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`).
Changed: a refusal that is no code word reads `unknown`, a string `detail` that is no code word reads null or none, and `unknown` keeps exit 14; a code word outside the closed list stays visible, also with exit 14.
Changed: `channel-response`'s `detail` closes to the enum `unsupported-version`, `invalid-params`, `release-from-driver` in `schemas/diag-line.v1.json`; wire bodies and `v` 1 unchanged.
Retired: `viola_agent_claude::Refusal` folded into `AgentError` (`BatchScriptChild`, `ProgramNotFound`, messages unchanged) and both private `fault_detail` copies; `CoreError` measured as never existing.
Added: three test binaries (`contract_error_bodies`, `channel_error_bodies`, `cli_error_bodies`) and the check `assert_error_body_clean` in `tests/support/events.rs`; the after-every-test helper is replaced by them and not built.
Measured: `viola answer` and `viola send` exit-2 paths write a fixed stderr line whatever `--json` says, no document; `viola mcp`'s line past the frame cap before `initialize` exits 1, not 11; `ci#41188681876` green 15/15 over `fa7be1f3`.

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
- `.andromeda/a11y-plan.md:257`, §8 Error recovery pattern: the clause that `unknown` (exit 14) has an opaque detail is stale; the detail is now a code word or nothing, and a reader's opaque rule is kept.
- `.andromeda/a11y-plan.md:345`, §11 Cognitive anti-pattern: its `unknown (exit 14, opaque detail, design cli)` carries the same stale clause.
- `.andromeda/a11y-plan-detail/cognitive-accessibility.md:19`, cognitive-errors lines 18–19: the same stale clause, named by the report's `code-word` sweep.
- Any other cognitive-errors entity on a refusal's reason or detail: a peer's free text no longer reaches a document or a text; a non-code `refusal` reads `unknown`, a non-code string `detail` reads null or none.
- Any statement that the refusal words shown are only those of a closed list: a code word outside the list stays visible in the document and in `refused: <word>`, its exit 14 by `reason_of`.
- Any statement that a `--json` failure always gives one document (§8 pattern, cognitive-errors, the CLI verbs entity): the `viola answer` and `viola send` exit-2 paths give a fixed stderr line and no document (`src/cmd/answer.rs:91`, `src/cmd/send.rs:187`).
- Any statement on faults (not refusals, no hint): `wrapper-fault`'s fields in a document are closed by `sanitised_fault`; the hint rule itself stands.
- cognitive-plain-language, the rule that every fixed CLI string is pinned: `wrapper fault` is a new fixed `message` literal on `--json` and MCP, pinned by a unit case, for a code outside the five.
- cognitive-plain-language, the no-stack-trace rule: indirectly, through the check's `caused-by` class and the census case `thiserror_messages_interpolate_nothing`.
- `.andromeda/a11y-plan.md:346` and cognitive-plain-language, the rule that a kebab-case code sits beside the word `unable`: a word now also shows as `refused: <word>`; test whether the rule's wording still covers that form.
- Cognitive-errors verification and any test inventory there (trycmd, assert_cmd, hint-free `--json`): `tests/cli_error_bodies.rs` is a new witness of literal exit, empty stderr and no ESC.
- The SC 3.3.1 stand-in, refusal reason and detail given in text (§1 Path 6, path-p6-board in both detail files): for `unknown` the detail may now be absent.
- §1 Paths 2, 3 and 4, §4 P2, P3, P4 and P6: only if an entity says a refusal's detail is free text; closed words such as `not-delivered`, `human-typing`, `unverified-cli` pass the screen unchanged.
- scope-entities, the CLI verbs entity in `--json` mode, and scope-surfaces' no-SGR rule: a new witness reads each driven document for no ESC, no absolute path, no `Caused by`, no serde path.
- Any text naming `viola_agent_claude::Refusal`, `Refusal::NotFound`, `fault_detail` or `CoreError`: retired or never real.
- Any text on `resolve_program`: it now returns `Result<PathBuf, AgentError>`, in the crate and in `src/run/mod.rs`; outputs unchanged, including the `.cmd` refusal on `windows-2025`.
- Line citations into any reached file: lines moved, chiefly `src/cmd/client.rs`, `crates/viola-channel/src/lib.rs`, `crates/viola-channel/src/server.rs`, `crates/viola-agent-claude/src/lib.rs`, `src/cmd/run.rs`, `src/cmd/verify.rs`.
- Any count of test binaries or tests, or of the `ci` profile's kill-line filter: 3 new binaries; the filter's `binary(…)` terms 22 to 24; jobs 3347 ubuntu, 3301 macOS, 3300 Windows.
- The no-new-suite rule and the `coverage` suite statements, indirectly: the new binaries run inside the existing suites, no new `suite` value; any list of binaries that carry CLI discipline gains `cli_error_bodies`.
- Typed-exit statements: `viola mcp`'s line past the frame cap before `initialize` exits 1 and 11 is only the unit child's code; `budget-paused`'s exit 11 is a separate code and did not move.
- Any statement that leans on an after-every-test error-body check in the `events` helper: replaced by the three witnesses, the enumeration and the census, and not built.
- cognitive-timeouts' list of bounds: indirectly only; `src/cmd/hook/statusline.rs`, `src/run/budget.rs`, `src/run/dialog.rs` gained frame-cap test cases, and no time bound moved.
- §1 scope-surfaces, `viola run` diagnostics going to `diagnostics/`: indirectly; `channel-response`'s `detail` word is new on the log line, level WARN unchanged, no terminal byte added.
- Agreement across masters: the same `unknown` correction lands in design-system (`surface.md`, `color-palette.md`) and layout-templates, so a11y-plan's wording must match a code word or nothing.

## New facts
- F1 · cognitive-accessibility · A refusal from a peer's reply is screened by `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a non-code `refusal` reads `unknown`, a non-code string `detail` reads null on `--json` and MCP.
- F2 · cognitive-accessibility · A code word outside the closed refusal list stays visible by design, in the document and in `refused: <word>`, with `unknown`'s exit 14.
- F3 · cognitive-accessibility · A fault body on `--json` and MCP is read through `viola_channel::sanitised_fault`; a code outside the five known reads the fixed `message` `wrapper fault` with null `data`, and the human line `error: wrapper fault  <code>` is unchanged.
- F4 · cognitive-accessibility · An exception to one document under `--json`: the `viola answer` and `viola send` exit-2 paths write the fixed `UNUSABLE_RESPONSE` or `UNUSABLE_TEXT` line to stderr and no document, named in `NOT_ERROR_BODIES`.
- F5 · cognitive-accessibility · `cli_error_documents_hold_no_path_or_upstream_text` in `tests/cli_error_bodies.rs` holds each `--json` failure it drives to one document, a literal exit, empty stderr, no ESC and a clean body.
- F6 · cognitive-accessibility · `tests/contract_error_bodies.rs` is the census of error writers: each body names a writer and a witness that stand, each product error writer is named, and no `thiserror` message interpolates.
- F7 · cognitive-accessibility · `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes.

## Untouched
- visual-sgr in `visual-design-verification.md`: no colour, SGR or NAME-cut behaviour changed; the session reads for ESC in documents and writes none.
- aria-landmarks: the BAY context line, caption line, `-- UNWRAPPED - READ-ONLY --` separator and `--help` group labels did not move.
- cognitive-consistency: the `CAPTION` literal and caption order did not move.
- The A11y tier `Standard (1)`, the triggers table and the no-WCAG-conformance rule: no surface or regime was added.
- The not-assertable classification of the plugin, stdio MCP server and wrapper IPC: the new MCP and channel witnesses read machine-to-machine bodies, and no UI surface was gained.
- §9 CI Integration, the `ci.yml` matrix and `gate --require coverage,doctest`: no harness verb, CI step or status shape moved.
- The manual NVDA, VoiceOver and Orca passes: founder-owned, unchanged, not run by this session.
- `viola revive`'s one-line `unable:` exit-1 refusals: `src/cmd/revive.rs`'s private `Refusal` is another type and stays.
- The frontend's deferral to 0.2.0: the session has no page surface.
