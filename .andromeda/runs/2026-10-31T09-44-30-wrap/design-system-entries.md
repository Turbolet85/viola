## 2026-10-31-sanitised-error-surfaces — `unknown`'s detail, a code word or nothing
**Section:** §Color Palette, Semantic Colors (color-palette.md: status-states, the `RefusalReason::unknown` row) · §Surface: cli, Component Patterns (surface.md: cli-send, the no-hint line; cli-exit-codes, row 14)
**Change:**
- `RefusalReason::unknown`'s row: was `unable · unknown` with the detail printed as opaque plain text; now with the detail, a code word or nothing, printed as plain text. The row's colours are unchanged.
- The exit-14 row: was `unable  unknown  <detail as opaque text>`; now `unable  unknown  <detail>`, the detail a code word or nothing. Its `--json` cell stays the refusal object.
- The no-hint line: `unknown` (exit 14) still has no hint line; its reason was that the detail is opaque; now that the detail is a code word or nothing and a hint would have to guess.
**Why:** both clients now screen a peer's reply, so a free-text detail reaches no document or text and the opaque-text wording no longer holds.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the code word and the screen on a peer's reply
**Section:** §Surface: cli, Tokens (surface.md: cli-output, three new items after the stderr line)
**Change:**
- New: a code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first (`viola_core::is_code_word`); both clients, `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`), screen a peer's `refusal` and `detail` through it.
- New: a peer's `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is no known reply.
- New: a code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does (`reason_of`).
**Why:** `viola_core::is_code_word` and its two callers landed in this chunk, so a peer's `refusal` or `detail` reaches no document or text except as a code word. The second item's contradiction with the exit-14 row's line was ruled apply by the overseer as the founder's delegate (Q6), PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — exit 20's closed `detail` and the fixed `wrapper fault` message
**Section:** §Surface: cli, Component Patterns (surface.md: cli-send, the no-hint line; cli-exit-codes, two new items after the exit-20 link / unlink item)
**Change:**
- The no-hint line: `error: wrapper fault` (exit 20) and `error: internal error` (exit 1) still have no hint line, being faults, not refusals; was their detail goes only to `diagnostics/`; now the human line holds the code only, further detail goes to `diagnostics/`, and the `--json` document's `detail` is row 20's closed object.
- New: exit 20's `--json` `detail` is closed for the five known codes (`viola_channel::sanitised_fault`). `message` is each code's fixed text. `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values; `data.reason` exactly `release-from-driver` reads that body; any other `-32602` reads `invalid params` with null `data`. Every other known code has null `data`.
- New: a code outside the five prints the same human line, `error: wrapper fault  <code>`, and under `--json` the document `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
**Why:** `viola_channel::sanitised_fault` replaced the two private `fault_detail` copies, so a peer's `message` and `data` reach no document or text, and `wrapper fault` is a new fixed output word on `--json` and MCP. The two new items' contradiction with the no-hint line was ruled apply by the overseer as the founder's delegate (Q1), PROVISIONAL, with that line's last clause reading as above.
**Kept:** row 20's own cells stand, its document still elided as `"detail":{…}`; the closed contents are the two items below the table.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — exit 2's fixed stderr lines, no `--json` document
**Section:** §Surface: cli, Component Patterns (surface.md: cli-exit-codes, row 2) · §Surface: cli, Tokens, Streams (design-system.md: the `--json` line) · §Surface: cli, Component Patterns, Exit-code phraseology (design-system.md: the exit-code sentence)
**Change:**
- The exit-2 row: was clap usage text (never produced by `viola hook`) with `—` in the `--json` cell; now that clap text, or the fixed `UNUSABLE_RESPONSE` line (an oversize `viola answer`) or `UNUSABLE_TEXT` line (an unusable `viola send`) on stderr, whatever `--json` says, with `tests/contract_error_bodies.rs` naming them in `NOT_ERROR_BODIES`; the `--json` cell reads no document.
- The `--json` line: was the whole result is one JSON document on stdout with the typed exit code, no exception stated; now except an oversize `viola answer` and an unusable `viola send`, which exit 2 with their fixed stderr line and no document.
- The exit-code sentence: was 2 clap usage; now 2 clap usage or the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` stderr line of those two paths (no document under `--json`).
**Why:** measured, the exit-2 paths write the fixed line to stderr whatever `--json` says, with no document, so one document for every result does not hold. The row's contradiction, and the two master lines that depend on it, were ruled apply by the overseer as the founder's delegate (Q5), PROVISIONAL, the clap text kept in the row.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the `--json` failures the witness drives
**Section:** §Surface: cli, Component Patterns (surface.md: cli-exit-codes, a new item after the exit-20 items)
**Change:** New: each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
**Why:** `cli_error_documents_hold_no_path_or_upstream_text` measures this shape and reads met; no standing text of the file stated it across the failures.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/
