proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "Replace «its detail is opaque, so design prints no hint rather than a guess» with «its detail is a code word or nothing, opaque to a reader, so design prints no hint rather than a guess»."
    sidecar: "cognitive-errors: `unknown`'s detail is a code word or nothing, a reader's opaque rule kept (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Changes › Symbols / APIs: a string `detail` that is no code word reads None / null; Expected amendments name `cognitive-accessibility.md:19` under pattern `code-word`, all 9 sites changed, with a reader's opaque rule kept."
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "Replace «`--json` gives one document with no hint.» with «`--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths: a fixed stderr line and no document.»"
    sidecar: "cognitive-errors: `--json`'s one document carries its measured exception, the `viola answer` and `viola send` exit-2 paths (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Spec claims disproved: the exit-2 paths write the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says, no document, named in `NOT_ERROR_BODIES`."
    basis: src/cmd/answer.rs:91
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 257
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "Replace «whose detail is opaque» with «whose detail is a code word or nothing, opaque to a reader»."
    sidecar: "§8 Error recovery: `unknown`'s detail is a code word or nothing, matching cognitive-errors (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Expected amendments: layout-templates and a11y-plan §8 Error recovery carry `unknown`'s detail the same; `a11y-plan.md:257` is a `code-word` site, changed."
    basis: .andromeda/a11y-plan.md:257
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 257
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "Replace «`--json` gives one document with no hint.» with «`--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (a fixed stderr line, no document).»"
    sidecar: "§8 Error recovery: the broad line names `--json`'s exception, the `viola answer` and `viola send` exit-2 paths (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Spec claims disproved: the exit-2 paths write a fixed line to stderr whatever `--json` says, no document; the broad line details cognitive-errors and states one document without exception."
    basis: src/cmd/send.rs:187
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 345
    section: "## 11. A11y Anti-Patterns (NEVER do these) › ### Cognitive"
    change: "Replace «`unknown` (exit 14, opaque detail, design cli)» with «`unknown` (exit 14, its detail a code word or nothing, design cli)»."
    sidecar: "§11 Cognitive: the hint ban's `unknown` carve-out reads a code word or nothing (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Expected amendments: `a11y-plan.md:345` is a `code-word` site, changed; design-system's `RefusalReason::unknown` moves to the same wording, a code word or nothing, not opaque free text."
    basis: .andromeda/a11y-plan.md:345
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "  - **CLI, a peer's refusal:** a refusal from a peer's reply is screened by `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a non-code `refusal` reads `unknown`, a non-code string `detail` reads null on `--json` and MCP (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "cognitive-errors: a peer's refusal is screened by `is_code_word` (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Changes › Symbols / APIs: `viola_core::is_code_word`, called by `reply_of` and `of_reply`, and the two readings of a non-code `refusal` and `detail`."
    basis: crates/viola-core/src/lib.rs:347
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "  - **CLI, a word outside the list:** a code word outside the closed refusal list stays visible by design, in the document and in `refused: <word>`, with `unknown`'s exit 14 (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "cognitive-errors: a code word outside the closed list stays visible, exit 14 (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Changes › Symbols / APIs: a code word outside the closed list stays visible in the document and in `refused: <word>`, its exit `unknown`'s 14 by `reason_of`; Outcome: visible by design."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "  - **CLI, a fault's body:** on `--json` and MCP a fault body is read through `viola_channel::sanitised_fault`; a code outside the five known reads the fixed `message` `wrapper fault` with null `data`, and the human line `error: wrapper fault  <code>` is unchanged (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "cognitive-errors: a fault body on `--json` and MCP is read through `sanitised_fault` (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Changes › Symbols / APIs on `sanitised_fault` and Coverage of new surfaces on the `message` literal `wrapper fault`, the human line unchanged."
    basis: crates/viola-channel/src/lib.rs:113
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "  - **CLI, the exception to one document:** the `viola answer` and `viola send` exit-2 paths write the fixed `UNUSABLE_RESPONSE` or `UNUSABLE_TEXT` line to stderr whatever `--json` says and no document, named in `NOT_ERROR_BODIES` (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "cognitive-errors: the exit-2 stderr lines of `viola answer` and `viola send`, no document (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Spec claims disproved: measured at `src/cmd/answer.rs:91` and `src/cmd/send.rs:187`, named in `NOT_ERROR_BODIES`."
    basis: src/cmd/answer.rs:91
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 69
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "  - **cli, error documents:** `cli_error_documents_hold_no_path_or_upstream_text` in `tests/cli_error_bodies.rs` holds each `--json` failure it drives to one document, a literal exit, empty stderr, no ESC and a clean body (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "cognitive-errors verification: the new witness of `--json` failure documents (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Outcome: each CLI `--json` failure the witness drives is one document, literal exit, stderr empty, no ESC, clean: met by the named case."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 69
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "  - **cli, the census of error writers:** `tests/contract_error_bodies.rs`: each body names a writer and a witness that stand, each product error writer is named, and no `thiserror` message interpolates (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "cognitive-errors verification: `tests/contract_error_bodies.rs`, the census of error writers (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, New text, by line: `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`; Spec claims disproved names the census of writers among what replaces the after-every-test check."
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 69
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "  - **cli, the body check:** `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "cognitive-errors verification: the error-body check in `tests/support/events.rs` (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Changes › Symbols / APIs: tests' check, `error_body_hit` and `assert_error_body_clean`, with the four classes."
    basis: tests/support/events.rs:18
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 90
    section: "## 8. Cognitive Accessibility › ### Plain language"
    change: "  - The `message` literal `wrapper fault` on `--json` and MCP, for a code outside the five, is pinned by the unit case `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` (`crates/viola-channel/src/lib.rs`, chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "cognitive-plain-language verification: the new fixed `message` `wrapper fault` and its pin (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Coverage of new surfaces: a new fixed output word on `--json` and MCP, tests unit `sanitised_fault_carries_nothing_of_the_peer_but_closed_values`; the entity lists how each fixed string is pinned."
    basis: crates/viola-channel/src/lib.rs:398
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 10
    section: "## 1. A11y Scope Summary"
    change: "  - **Failures under `--json` (chunk 2026-10-31-sanitised-error-surfaces):** each failure `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives is one document, a literal exit, empty stderr, no ESC and a clean body; the `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document."
    sidecar: "scope-entities, CLI verbs: the `--json` failure witness and the exit-2 exception (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Outcome on the witness and Spec claims disproved on the exit-2 paths; the entity records the verbs' `--json` behaviour per verb, and its head line is already past the category's line limit, so the addition is its own sub-item."
    basis: src/cmd/send.rs:187
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/a11y-scope-summary.md
    near: 102
    section: "## 1. A11y Scope Summary"
    change: "Replace «the revive refusal pairs carry no ESC byte;» with «the revive refusal pairs carry no ESC byte, and each `--json` failure document `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives carries none (chunk 2026-10-31-sanitised-error-surfaces);»."
    sidecar: "scope-surfaces, cli: the no-ESC rule gains the `--json` failure documents' witness (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report, Outcome: each CLI `--json` failure the witness drives carries no ESC, and no SGR in a `--json` document is met; the rule stands and gains a witness."
note: "D-a11y-surface: the report adds no interactive UI element. D-a11y-obs-schema: the report closes `channel-response`'s `detail` in `schemas/diag-line.v1.json:42`; a11y-plan's body and picked entities state no such field and no a11y violation schema, so no divergence was read, and its contracts registry was no input. `.andromeda/a11y-plan.md:270` and `:346` tie a code to the word `unable`, while the report has a word in `refused: <word>` that stays visible and names neither its surface nor a move, so no proposal is made: check it against design-system's wording."
