# Task — design · 2026-10-27-bounded-inputs-at-every-boundary

## The chunk
Makes every input that crosses into a `viola` process bounded and validated where it enters: a `Read::take(MAX_FRAME)` (or the `MAX_FRAME + 1` over-size form) on every external reader, serde_json default depth only, closed types for every decision read from outside, `ViolaName::try_new` (or a closed-shape validator) before every path join.
Re-verifies the paste rule (`validate_paste_text`, `not-delivered` / `control-character`, `empty-text`) at every client (CLI `send`, MCP `send`) and authoritatively in the wrapper, never a strip.
Re-verifies that a vt100 screen-model panic degrades readiness to `input-not-ready` without harming the child's passthrough to the human's terminal.
Adds a unit-level witness per property and per boundary, and an enumeration of the external readers, each with its cap, depth and type rule; the candidate claim is `v1-46`, its web / `Last-Event-ID` clause out.

## Reached files
- src/run/gate.rs · re-reads the vt100 screen-model `catch_unwind` degrade to `input-not-ready` and the passthrough
- crates/viola-agent-claude/src/screen.rs · re-reads the vt100 screen-model `catch_unwind` degrade to `input-not-ready`

## Needs
- The `viola send` readback mirror's refusal line for the paste rule: `[/ ] unable  <name>  not-delivered  control-character` (and `empty-text`) on stderr, exit 13, then its `hint:` line; the client refuses before any frame.
- A `send` hint for a refused paste never quotes the sent text or any upstream text, and the refused character is never echoed or stripped.
- `send --json` under the paste rule: one JSON document on stdout carrying the typed refusal, the typed exit code, no `hint:` line.
- Exit-code phraseology for a refusal at a bounded input: 13 `not-delivered`, 2 clap usage, 1 `error: internal error` or a fixed `unable:` line, 20 `error: wrapper fault`; which code an over-size, malformed or invalid-name input takes follows that table.
- Prompt text reaches `send` only from stdin or `--file` (Git Bash rewrites leading-slash arguments), and the rewritten-path warning is the one plain stderr line `warning: argument looks like a Git Bash rewritten path`.
- `viola list` rows of `claude agents --json` origin: every control character escaped as `\xHH` text, `\n` and `\t` included, so each row stays one fixed-width line; only an unwrapped NAME is cut, only where stdout's terminal width is known.
- A failed, over-size or malformed `claude agents --json` read leaves the board as read: a wrapped row's STATUS reads `unknown`, never a guessed `idle` / `busy`.
- The BAY line's `skipped 0 - 0 - 0`: the `events.ndjson` reader's three skip counts stay visible, so an unknown kind or field skipped and counted shows there, never auto-cleared.
- The BAY line's `5H` / `7D` figures come only from a bounded `budget.json` read; no reading prints `unknown`, an old one `expired`.
- Message mode, the escaper `wait` and `last` share and `viola link` uses for a `since` that is no RFC 3339 instant: every C0, DEL and C1 except `\n` and `\t` prints as `\xHH` (two uppercase hex digits), nothing stripped; `--json` stays serde-escaped.
- `wait`'s result line for a closed event kind: `<kind>  <name>  <time>  cursor <n>`, the dialog form for `question` / `permission` / `plan` / `dialog-expired`, and `dialog unknown` when the event carries no `dialog_id`.
- `viola answer <name> <id> < response.json`: a decision read from stdin as a closed type, refused as `unable  <name>  not-delivered  unknown-dialog` or its reason, never passed through as free text.
- `viola revive --id` and `--json`: a refusal prints only closed codes (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) and session-shaped ids, never a path or pid.
- `viola hook` writes nothing to stderr and always exits 0, and `viola mcp` writes only MCP frames on stdout: a bounded-input refusal at their stdin has no human output surface.
- The cli bans over a new refusal for an over-size, malformed or invalid-name input: no upstream text, path, pid, anyhow chain with a serde source or stack trace; full detail only in `diagnostics/`.
- `viola list`'s human-mode environment reads as a boundary in the enumeration: `NO_COLOR`, `TERM` and `COLORTERM` only compared, never echoed, deciding only SGR bytes; `COLUMNS` never read, width only from stdout's own terminal.

## Standing conventions
- S1 · Human output is ASCII only, with no colour, glyph or cursor control under `--json`, a non-TTY stdout, `NO_COLOR`, `TERM=dumb` or `viola run` · Surface: cli › Tokens (.andromeda/design-system-detail/surface.md)
- S2 · Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr; under `--json` one document on stdout with the typed exit code and no hint · Surface: cli › Streams (.andromeda/design-system-detail/surface.md)
- S3 · A refusal is `unable` plus a typed reason in standard phraseology, never "done" or "success" without context, never green / red prefixes or emoji · Anti-Patterns › Per-Surface Bans › cli

## Not needed
- The Color Palette's core colors, surface scale, text hierarchy, semantic and domain status colours and border progression: 0.2.0 frontend design inputs; 0.1.0 renders no page.
- Typography's faces and per-OS fallback stacks: a 0.2.0 frontend input, no page rendered.
- Spacing tokens, the depth strategy and border radius: 0.2.0 frontend inputs.
- Motion and the expression level's duration scale: the chunk adds no animated or redrawn output.
- Iconography's web glyphs and the readback box's 16px drawing: the frontend is deferred to 0.2.0.
- The Self-Validation Token Test over `--c-*` values and lengths: no stylesheet exists in 0.1.0.
- Surface: web-spa: deferred to 0.2.0, technology not chosen.
- `viola --help`'s verb grouping and the hidden `mcp` verb's absence from it: the chunk adds no verb.
- `viola plugin install`'s four stdout lines: its output is unchanged by input bounding.
- `viola pause` / `viola release` / `release --budget` phraseology lines: the wheel and budget-gate wording is not changed by this chunk.
