# Task — a11y · 2026-10-31-sanitised-error-surfaces

## The chunk
Makes every error a `viola` process gives a machine reader a code and a fixed message only, on three surfaces: CLI `--json` error documents, `isError` results of `viola mcp`'s tools, and the wrapper's JSON-RPC error data.
Makes every product `thiserror` `Display` that can reach those surfaces a fixed message, with no path and no payload.
Proves it with one witness per surface, driving the failure with a path, an upstream text and a planted stripped `CLAUDE*` value in play, and with a checkable enumeration of the three surfaces' error bodies.
Folds in carried work: `viola-agent-claude`'s `Refusal` enum folded into `AgentError` or ratified as an exception, and the `channel-response` line and `fault_of` telling the `-32602` causes apart.
Adds owed witnesses: a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes, and a cap-driving case for each of five `BOUNDARIES` rows.

## Reached files
- crates/viola-agent-claude/src/lib.rs · edits the `Refusal` enum, `AgentError`
- crates/viola-channel/src/lib.rs · edits `ProtocolError::InvalidParams`
- crates/viola-channel/src/client.rs · edits `fault_of`
- tests/contract_bounded_readers.rs · edits five `BOUNDARIES` rows: the status-line command's stdout, the child pipe drain, the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep`
- .andromeda/obs-plan-detail/metric-coverage.md · re-reads the `viola.compat.v_rejected` contract the `channel-response` line is read against

## Needs
- CLI verbs in `--json` mode (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`): each verb's error document is closed to a code and a code-shaped or null `detail`.
- The `--json` refusal form: one document, no `hint:`, a typed exit code; the chunk's reshaped documents stay inside it, and its assertions are the assert_cmd reading of that form.
- The human-mode refusal layout on stderr, `unable <reason> <detail>` then a final `hint:`: reached indirectly through any `thiserror` `Display` the chunk makes fixed that also feeds this form; the layout and the hint rule still bind that text.
- The one-line `unable: <text>` form of `viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair: the text is a product error string, reached through the same `Display` work.
- The `unknown` refusal (exit 14) with its opaque detail and no hint: the carve-out a failure a verb cannot name meets when it stops being free text.
- Faults are not refusals and carry no hint: binds any CLI fault form the chunk reshapes while removing an anyhow chain.
- Stack trace or bare status number as the only error text is banned on the CLI: binds the removal of anyhow chains and serde sources, so a word and a code remain.
- Fixed CLI strings are pinned by trycmd or by literal, including the server-verify hint and the `viola link` / `viola unlink` result lines: a message the chunk changes moves its pin on purpose, and the chunk's enumeration of error bodies must agree with those pins.
- Plain-word rule: a kebab-case reason or detail code on the CLI's human mode always sits beside `unable`; binds any code the chunk newly surfaces there in place of free text.
- Refusal codes the plan's paths name, `not-delivered`, `human-typing`, `unverified-cli`, on `viola send` and `viola answer`: their words, stream and typed exit codes survive the closed-code work and any `Refusal` fold.
- The variants of `viola-agent-claude`'s `Refusal` (`BatchScriptChild`, `NotFound`, the `resolve_program` refusal), to the extent they reach a printed CLI refusal: word, code, hint and exit code hold whichever way the fold-or-ratify goes.
- Output discipline under `--json`: no SGR, glyph or cursor control, and C0/C1 control characters escaped except `\n` and `\t` over any value a document still prints; `unable` and `fail` never coloured.
- The output-discipline stand-ins for SC 3.3.1 and SC 3.3.3 on the CLI: the refusal reason and detail given in text, carried by the fixed messages.
- Carve-out for a CLI witness that drives the frame verbs' server-verify refusal: asserted on windows-2025 and ubuntu-latest only, since macOS reports no peer pid.
- Plugin stdio MCP server and wrapper IPC channel, the plan's not-assertable entity: machine-to-machine with no UI surface, so the `isError` and JSON-RPC witnesses, `ServeError::Initialize` (exit 11) among them, owe no a11y assertion of their own.

## Standing conventions
- S1 · A new CLI assertion uses the existing `assert_cmd` / `trycmd` / portable-pty drivers inside `nextest-integration` / `nextest-e2e` (CI: `coverage`), with no new `suite` value and no separate a11y driver · §1 harness specification, §3 contracts, §9 CI Integration, §11 CI
- S2 · A new CLI output case runs on all three OS legs, each leg's `gate --require coverage,doctest` its own verdict · §9 CI Integration, §10 SLO Invariants
- S3 · Zero flakiness: `retries: 0`, `#[ignore]` banned, no retry-once for a violation · §10 SLO Invariants, §11 SLO
- S4 · Output-discipline results stand in for SCs and carry no WCAG conformance claim · §11 Strategy, .andromeda/a11y-plan-detail/a11y-scope-summary.md
- S5 · Evidence is machine output from an agent-runnable command; a manual pass only supplements it · §11 Universal, §11 CI

## Not needed
- The TUI half of path P4: keystroke delivery past the atomic paste, focus, mouse and resize sequences, and the outer-PTY boundary reading; the chunk's scope names no change to the passthrough.
- `viola list` board presentation: the `CAPTION` literal and caption order, the BAY context line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels; the chunk changes error bodies, not the board or the help.
- Row styling on the board: amber on `DIALOG`, dim on stale rows, bold on NAME values, and the unwrapped NAME cut; the dim-on-stale outer-PTY witness is owed to "Linux and macOS parity".
- The owed `viola revive --json` document for a refusal `run`'s start makes after a passed preflight: out of scope, owed to "Exit-cause code catalogue".
- The Windows byte reading of the vt100 feed panic boundary: out of scope, owed to "Windows-only live measurements".
- Any frontend surface, browser a11y leg or WCAG mapping for a rendered page: the frontend is deferred to 0.2.0, and the scope carries the GUI clauses of the cap to residuals.
- The manual NVDA, VoiceOver and Orca passes on P4 and P6: a founder-owned supplement; the chunk adds none, and tests nothing past what the plan lists.
- The time bounds of the plan's timeout section (`viola wait`, the MCP `wait` cap, the confirmation windows, the budget pause): the chunk's owed cap-driving cases concern byte caps of bounded reads, and no time bound changes.
- Plain-language copy outside errors: the empty-board line and the `n/a` value are not error bodies.
