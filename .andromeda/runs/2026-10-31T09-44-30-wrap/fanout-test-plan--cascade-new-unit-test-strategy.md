proposals:
  - detector: D-new-detail
    severity: warning
    fact: F13
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 7
    section: '## 4. Unit Test Strategy'
    change: '- **viola-core:** `viola_core::is_code_word` accepts 1 to 64 bytes of `[a-z0-9-]` with a lower-case letter first, held by `code_word_literal_table` (`crates/viola-core/src/lib.rs`).'
    sidecar: 'Unit Test Strategy gains a viola-core group: `is_code_word` and its case `code_word_literal_table` (chunk 2026-10-31-sanitised-error-surfaces).'
    rationale: 'The report''s Symbols / APIs states the rule and its New text, by line lists `code_word_literal_table`; the category file holds no viola-core group, so a new one opens ahead of viola-channel, as the body''s §4 orders them.'
    basis: crates/viola-core/src/lib.rs:350
