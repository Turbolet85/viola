## 2026-10-31-sanitised-error-surfaces — the `--json` failure documents' witness
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: scope-entities, CLI verbs; scope-surfaces, cli) · §8 Error recovery (cognitive-accessibility.md: cognitive-errors, verification)
**Change:**
- CLI verbs gains the sub-item Failures under `--json`: each failure `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives is one document, a literal exit, empty stderr, no ESC byte and a clean body.
- The cli no-ESC rule: was the `viola revive --list` rows and the revive refusal pairs; now those and each `--json` failure document that case drives.
- The cognitive-errors verification gains the item `--json` failure documents: that case holds each failure it drives to one document, a literal exit, empty stderr, no ESC byte and a clean body.
**Why:** The chunk added this case, and its report reads each failure the case drives as one document, a literal exit, empty stderr, no ESC and clean. The no-ESC rule stands and gains a witness.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `--json`'s one document and its exit-2 exception
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: scope-entities, CLI verbs) · §8 Error recovery (a11y-plan.md; cognitive-accessibility.md: cognitive-errors)
**Change:**
- The `--json` rule: was one document with no hint, without exception; now one document with no hint, except the `viola answer` and `viola send` exit-2 paths (a11y-plan.md: a fixed stderr line, no document; cognitive-errors: "below").
- cognitive-errors gains the item CLI, the exception to one document: those paths write the fixed `UNUSABLE_RESPONSE` or `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document; `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) names them.
- The CLI verbs sub-item Failures under `--json` carries the same: those paths write a fixed stderr line and no document, the text at Cognitive Accessibility › Error recovery.
**Why:** Measured: the exit-2 paths write that fixed line to stderr whatever `--json` says, and no document, so a one-document rule without exception did not hold. The new item read as a contradiction of the standing one-document sentence and was escalated; the overseer, as the founder's delegate, answered apply: PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — `unknown`'s detail, a code word or nothing
**Section:** §8 Error recovery (a11y-plan.md; cognitive-accessibility.md: cognitive-errors) · §11 Cognitive (a11y-plan.md)
**Change:**
- §8 Pattern: was `unknown` (exit 14) "whose detail is opaque"; now "whose detail is a code word or nothing, opaque to a reader".
- cognitive-errors: was "its detail is opaque, so design prints no hint rather than a guess"; now "its detail is a code word or nothing, opaque to a reader, so design prints no hint rather than a guess".
- §11's hint ban: was `unknown` (exit 14, opaque detail, design cli); now `unknown` (exit 14, its detail a code word or nothing, design cli).
**Why:** A string `detail` that is no code word now reads nothing, so `unknown`'s detail can no longer be free text.
**Kept:** `unknown` still prints no hint, and its detail stays opaque to a reader: design's reason for the carve-out stands.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — a peer's refusal screened to a code word
**Section:** §8 Error recovery (cognitive-accessibility.md: cognitive-errors)
**Change:**
- Gains the item CLI, a peer's refusal: `reply_of` and `of_reply` screen a peer's reply with `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first); a `refusal` that is no code word reads `unknown`, and a string `detail` that is no code word reads nothing (null under `--json` and on MCP).
- Gains the item CLI, a word outside the list: a code word outside the closed refusal list is visible by design, in the document and in `refused: <word>`, and takes `unknown`'s exit 14.
**Why:** The chunk's contract is that a peer's `message`, `data`, `refusal` or `detail` reaches no document or text; a code word outside the closed list stays visible by design, its exit `unknown`'s 14.
**Kept:** §8 Plain language and §11 Cognitive still tie a kebab-case code to the plain word `unable`: that is the human form, `refused: <word>` is MCP's text, so neither moved.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — a fault's body and the `wrapper fault` literal
**Section:** §8 Error recovery (cognitive-accessibility.md: cognitive-errors) · §8 Plain language (cognitive-accessibility.md: cognitive-plain-language, verification)
**Change:**
- cognitive-errors gains the item CLI, a fault's body: under `--json` and on MCP a fault's body is read through `viola_channel::sanitised_fault`; a code outside the five known reads the fixed `message` `wrapper fault` with null `data`. The human line is `error: wrapper fault  <code>`.
- The plain-language verification gains the pin: the `message` literal `wrapper fault` under `--json` and on MCP, for a code outside the five known, is pinned by the unit case `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` (`crates/viola-channel/src/lib.rs`).
**Why:** `sanitised_fault` replaces the two private `fault_detail` copies, both gone, and its `wrapper fault` is a new fixed output word on `--json` and MCP, so it takes a literal pin like the entity's other fixed strings.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the census of error writers and the body check
**Section:** §8 Error recovery (cognitive-accessibility.md: cognitive-errors, verification)
**Change:**
- Gains the item the census of error writers: in `tests/contract_error_bodies.rs`, `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds each error body to a writer and a witness that stand, `product_error_writers_are_each_named` holds each product error writer to a name, and `thiserror_messages_interpolate_nothing` holds each `thiserror` message to no interpolation.
- Gains the item the body check: `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes.
**Why:** The planned after-every-test error-body check was measured absent and is not built; the three surface witnesses, the enumeration and the census of writers replace it, reading this check.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/
