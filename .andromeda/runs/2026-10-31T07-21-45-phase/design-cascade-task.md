# Task — design · 2026-10-31-sanitised-error-surfaces

## The chunk
Makes every error a `viola` process gives a machine reader a code and a fixed message and nothing else (no absolute path, upstream text, anyhow chain holding a serde source, or stripped `CLAUDE*` value) on three surfaces: CLI `--json` error documents, `isError` results of `viola mcp`'s tools, and `error.data` of the JSON-RPC errors a wrapper answers.
Holds every product `thiserror` `Display` that can reach one of those surfaces to a fixed message with no path and no payload.
Proves it with a witness per surface that drives the failure with a path, an upstream text and a planted stripped `CLAUDE*` value in play, and with an enumeration of the three surfaces' error bodies a reader can check, so a new free-text body is visible.
Takes the folded freight: `viola-agent-claude`'s second enum `Refusal` folded into `AgentError` or ratified as an exception, and the `-32602` causes told apart on the `channel-response` line and in the client's `fault_of`.
Adds the owed tests: a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes, and a cap-driving case for each of five `BOUNDARIES` rows; re-reads at HEAD whether the owner-readable detail files and the post-test secret scan of `v1-47` stand.

## Reached files
- crates/viola-agent-claude/src/lib.rs · edits the `Refusal` enum and `AgentError`
- crates/viola-channel/src/lib.rs · edits `ProtocolError`'s `-32602` causes
- crates/viola-channel/src/client.rs · edits `fault_of`
- tests/contract_bounded_readers.rs · edits the five `BOUNDARIES` rows' witnesses
- .andromeda/obs-plan-detail/metric-coverage.md · re-reads the `viola.compat.v_rejected` row

## Needs
- CLI `--json` error documents of every verb, the `{"v":1,"error":…,"detail":…}` form: a closed code, a code-shaped or null `detail`, no free text for a failure the verb cannot name.
- The `--json` stream rule: the whole result, refusals and `error` objects included, is one document on stdout with the typed exit code, no `hint:` line prints, and the typed `refusal` and `error` fields are the whole answer.
- The exit-code phraseology, code by code, as what the enumeration of CLI error bodies is read against: 1 `error: internal error` or a fixed `unable:` refusal, 2 clap usage, 10 to 14 an `unable` refusal with its reason, 20 `error: wrapper fault`, 21 `unable  instance-unreachable`.
- Carve-out through the exit-1 row: a `--json` document exists only for `viola revive`'s strict-modes and preflight refusals; a start refusal after a passed preflight prints none, pending on the entry "Exit-cause code catalogue", and this chunk leaves that gap standing.
- The exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `plugin install`: the `instance-unreachable` document with stderr empty under `--json`, the fixed home-state or instance-state `unable:` / `hint:` pair without it, no frame and no line written.
- The refusal mirror of `send` and `answer`: `[/ ]` `unable` with `reason  detail` and the matching refusal object under `--json`, typed reasons such as `not-delivered`, `no-prompt-submitted` and `unknown-dialog`.
- Carve-out for the closed code set: the per-cause `detail` code under `--json` is pending, its owner the arch amendment naming them, and it bears on what a code-shaped or null `detail` may hold there.
- Hint rules, wherever a changed message or fold reaches a `hint:` line: one per reason, detail or cause, never quoting sent or upstream text, none for faults and `unknown`, none naming `viola release`.
- The never-log floor as the cli bans state it, which the chunk's messages and witnesses enforce: no upstream text, path, pid or anyhow chain with a serde source in an error or a hint, and no stack trace; note that the ban names pids as well as the entry's own list.
- Where stripped detail goes: errors are fixed messages and full detail goes only to `instances/<name>/diagnostics/`.
- `run` and `revive` start refusals through any `Display` that reaches them: a cause's fixed-message `unable:` line and its hint, showing no path and no pid, `revive`'s read in a fixed order that stops at the first.
- `verify`'s refusal forms: a run stopped before its rows or refused by its home tree's strict-modes check prints only its fixed `unable:` / `hint:` pair and exits 1, and a refused `stamps.json` update prints the home-state pair.
- The `Refusal` fold or ratification over its variants `BatchScriptChild`, `NotFound` and the `resolve_program` refusal: where a variant reaches a printed refusal, it stays a cause's fixed message and hint.
- `viola mcp` as a hidden verb with no human design surface, stdout carrying MCP frames only: binds its error results, its handler fault, and the witness of `ServeError::Initialize` (exit 11) on an over-`MAX_FRAME` line before `initialize`.
- `viola hook` writing nothing to stderr and always exiting 0, wherever a `Display` change or an enumeration reaches that verb.
- MCP error results of `send`, `wait`, `last`, `answer` and `list`: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params`, text `refused: <code>`, a handler fault exactly `internal error` with `data: null`; drivers read these beside `--json` as the same typed answers.
- Channel error data: each JSON-RPC error's fixed message, `error.data` a code or null, `-32603` exactly `internal error` with `data: null`, `-32602`'s causes `invalid params`, unsupported-version and `release-from-driver`, the `channel-response` line's `error_code`, and the metric `viola.compat.v_rejected`.
- A channel fault reaching a verb through the client's `fault_of`: it surfaces only in the typed exit forms, 20 and 21 and their `--json` documents, and any change to how `-32602` is labelled carries no channel text into them.
- Carve-out from the floor for results: `wait` and `last` message text in message mode, `viola link`'s escaped `since`, and `list` fields of `claude agents --json` origin with controls escaped as `\xHH`, `--json` staying serde-escaped, are result data and not error bodies.
- Values a cap-driving case may read where it goes through `viola list`, for the status-line command's stdout and the child pipe drain of `claude --version` and `claude agents --json`: a field with no reading prints `unknown`, a wrapped row with no `claude agents --json` join reads STATUS `unknown`, a `claude_agents` failure surfaces as STATUS `unknown`.

## Standing conventions
- S1 · every new or changed human refusal or error line goes to stderr, results to stdout, data and messages never mixed · Surface: cli, Streams; Anti-Patterns, Per-Surface Bans (cli)
- S2 · human output stays ASCII only and lines are never wrapped, with no emoji and no check or cross prefix · Surface: cli, Width; Anti-Patterns, Per-Surface Bans (cli)
- S3 · nothing new emits colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR` or `TERM=dumb` · Anti-Patterns, Per-Surface Bans (cli)
- S4 · each new code or fixed message is short standard phraseology, `unable` plus a typed reason, with no filler and no guessing · Brand Identity, Domain anchors

## Not needed
- The Color Palette's core colors, surface scale, text hierarchy, border progression and contrast rules: a 0.2.0 frontend input, and any frontend surface is out of this chunk.
- The domain status rows and their guards in Semantic Colors, the access-refusal guard over token, URL and file path among them: frontend treatments, and the GUI token, cookie and launch URL clauses are carried to 0.2.0.
- Typography, Spacing, Depth Strategy, Border Radius and the web glyphs of Iconography: 0.1.0 renders no page.
- Motion and the expression level: the chunk adds no motion, and an error body has none.
- Surface: web-spa, deferred to 0.2.0 with its technology not chosen.
- The cocked strip's offset and amber band and the readback box's web drawing: the chunk draws nothing.
- The cli colour tokens' values across truecolor, 256 and 16 colour, and the depth choice: the chunk styles nothing.
- The `viola list` table's column order, BAY line figures, row order and NAME truncation: the chunk changes no board layout.
- The success lines of `pause`, `release`, `link`, `unlink` and `plugin install`: the chunk reaches their refusals, not their results.
- The `--help` rows and the flat verb navigation: the chunk adds no verb.
- Windows VT enabling, the conhost fallback and the Git Bash warning: unchanged, and Windows-only live readings are out of this chunk.
- The Swap, Squint, Token and Sameness tests of the Self-Validation Protocol: they judge a drawn interface, and the chunk draws none.
