proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/aria-patterns-roles.md
    near: 10
    section: '## 4. ARIA Patterns & Roles'
    change: 'In the cli row, replace the closing clause `the handoff: row is owned by "Explicit session links"` with: the `handoff:` row built (chunk 2026-10-26-explicit-session-links): `handoff:  link, unlink` between `wheel:` and `setup:`, so the group rows now read `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:` over the served verbs only, plain ASCII, held by the `HELP` literal in `help_is_the_grouped_verb_table` and by `help_lists_only_served_verbs` (`tests/cli_output_plain.rs`)'
    sidecar: 'aria-landmarks cli row: the handoff: help row owed to "Explicit session links" is now built (handoff:  link, unlink between wheel: and setup:), the hold retired'
    rationale: 'Report Retired bullet retires the hold on the explicit viola link / viola unlink verbs and every master line stating it; Counts names this line stale under the owner pattern title and under handoff (aria-patterns-roles.md:10); Symbols / APIs: viola --help gains the row handoff:  link, unlink between wheel: and setup:; Outcome (layouts) met by help_is_the_grouped_verb_table and help_lists_only_served_verbs'
    basis: .andromeda/a11y-plan-detail/aria-patterns-roles.md:10
note: 'D-a11y-surface and D-a11y-obs-schema hold for this file (no interactive element added, the a11y / obs schema untouched); the body line a11y-plan.md:104 (handoff: row owed to "Explicit session links") is the same retired hold, outside this category file, for the body pass.'
