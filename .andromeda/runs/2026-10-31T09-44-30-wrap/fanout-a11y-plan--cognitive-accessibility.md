proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 19
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "In the sentence on `unknown`, replace «its detail is opaque, so design prints no hint rather than a guess» with «its detail is a code word or nothing, not opaque free text, so design prints no hint rather than a guess»; the rest of the line stands."
    sidecar: "§8 Error recovery detail: `unknown`'s detail is a code word or nothing, no longer opaque free text (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Expected amendments: a11y-plan §8 Error recovery takes `unknown`'s detail the same as design-system, a code word or nothing; pattern `code-word` lists `cognitive-accessibility.md:19` as changed, and Symbols / APIs has `reply_of` read a string `detail` that is no code word as None."
    basis: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 61
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "New record: - **CLI, a peer's refusal as the client reads it:** `reply_of` (`src/cmd/client.rs`) reads a `refusal` that is no code word (`viola_core::is_code_word`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first) as `unknown`, and a string `detail` that is no code word as none; a code word outside the closed list stays visible in the document and in `refused: <word>`, its exit `unknown`'s 14; pinned by the unit `reply_of_reads_the_three_shapes` (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§8 Error recovery detail: the client's screen of a peer's refusal and detail recorded, a word outside the closed list staying visible at exit 14 (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Symbols / APIs, the `reply_of` and `of_reply` bullet, and Coverage of new surfaces: this is what the printed word and detail of a refusal now hold, which Error recovery's pattern depends on."
    basis: src/cmd/client.rs:77
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 81
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "New record: - **Verification, `--json` failure documents:** `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) reads, for each CLI `--json` failure it drives, one document, the literal exit code, stderr empty, no ESC byte and a body clean of paths and upstream text (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§8 Error recovery detail: the `--json` failure documents' witness recorded (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Outcome: each CLI `--json` failure the witness drives gives one document, literal exit, stderr empty, no ESC, clean; the case witnesses this file's one-document, hint-free `--json` claim."
    basis: tests/cli_error_bodies.rs:115
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 90
    section: "## 8. Cognitive Accessibility › ### Plain language"
    change: "New record: - **Verification, the `wrapper fault` message:** for a fault code outside the five known, `wrapper-fault`'s detail on `--json` and MCP carries the fixed `message` `wrapper fault` with null `data`, pinned by the unit `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` (`crates/viola-channel/src/lib.rs`), not by trycmd; the human line `error: wrapper fault  <code>` (`src/human/mirror.rs`) is unchanged (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§8 Plain language detail: the new fixed `wrapper fault` message on `--json` and MCP and its unit pin recorded (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Coverage of new surfaces names this a new fixed output word on `--json` and MCP with that unit test; line 90 lists how fixed CLI strings outside trycmd are pinned, and this one is not yet listed."
    basis: crates/viola-channel/src/lib.rs:123
note: "D-a11y-surface: the report adds no interactive UI element. D-a11y-obs-schema: `channel-response.detail` closes in `schemas/diag-line.v1.json:42`, and this file states no a11y violation schema or that field, so no divergence here. The `code-word` hits at `a11y-plan.md:257` and `:345` are in the body, outside this file, and are not proposed here."
