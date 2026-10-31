# Cascade compare — a11y-plan · wrap · 2026-10-31-sanitised-error-surfaces
E items 4 · held 2 · lost at classify 0 · lost at strong 0 · beside a placed fact 2 · cascade only 3 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### cognitive-accessibility#9.1 · lost at classify · beside a placed fact F5 (near 69, cognitive-accessibility#10.1) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:61
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 61
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "New record: - **CLI, a peer's refusal as the client reads it:** `reply_of` (`src/cmd/client.rs`) reads a `refusal` that is no code word (`viola_core::is_code_word`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first) as `unknown`, and a string `detail` that is no code word as none; a code word outside the closed list stays visible in the document and in `refused: <word>`, its exit `unknown`'s 14; pinned by the unit `reply_of_reads_the_three_shapes` (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§8 Error recovery detail: the client's screen of a peer's refusal and detail recorded, a word outside the closed list staying visible at exit 14 (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Symbols / APIs, the `reply_of` and `of_reply` bullet, and Coverage of new surfaces: this is what the printed word and detail of a refusal now hold, which Error recovery's pattern depends on."
    basis: src/cmd/client.rs:77

### cognitive-accessibility#12.1 · lost at classify · beside a placed fact F6 (near 69, cognitive-accessibility#10.1) · .andromeda/a11y-plan-detail/cognitive-accessibility.md:81
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 81
    section: "## 8. Cognitive Accessibility › ### Error recovery"
    change: "New record: - **Verification, `--json` failure documents:** `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) reads, for each CLI `--json` failure it drives, one document, the literal exit code, stderr empty, no ESC byte and a body clean of paths and upstream text (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "§8 Error recovery detail: the `--json` failure documents' witness recorded (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Outcome: each CLI `--json` failure the witness drives gives one document, literal exit, stderr empty, no ESC, clean; the case witnesses this file's one-document, hint-free `--json` claim."
    basis: tests/cli_error_bodies.rs:115

## Held

- cognitive-accessibility#13.1
- cognitive-accessibility#2.1

## Cascade only

- a11y-scope-summary#1.1
- a11y-scope-summary#10.1
- cognitive-accessibility#10.1
