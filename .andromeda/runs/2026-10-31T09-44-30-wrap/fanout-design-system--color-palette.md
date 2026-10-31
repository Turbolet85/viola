proposals:
  - detector: D-design-cli-wording
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 102
    section: ## Color Palette › ### Semantic Colors
    change: In the `RefusalReason::unknown` row the fifth cell (Printed as · treatment) now reads `unable · unknown`, with the detail a code word or nothing, never free text; a refusal that is no code word reads as this reason, and a string detail that is no code word prints nothing after the reason (as `RefusalDetail::null`); the three colour cells stay.
    sidecar: Semantic Colors, `RefusalReason::unknown` row - the detail is a code word or nothing, no longer opaque plain text (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Changes, Symbols / APIs, has `reply_of` and `of_reply` read a non-code-word `refusal` as `unknown` and a non-code-word string `detail` as None or null; its Expected amendments and the `code-word` sweep name color-palette.md:102 as changed, so the row's opaque-plain-text wording stands retired.
    basis: src/cmd/client.rs:77
