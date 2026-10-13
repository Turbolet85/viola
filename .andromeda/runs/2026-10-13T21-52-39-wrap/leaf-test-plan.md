group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
read: .claude/rules/testing.md · 93 · «- 2026-10-13: A `+=` → `-=` mutant on an unsigned sum that s»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-11: On the Linux dev host `schema-check` (G4) read»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the framing tier by Run B's three added pastes»
  source: .andromeda/test-plan.md:438 «The framing-tier fixtures are recorded the same way by Run B's four added pastes as `UserPromptSubmit.paste-<n>.json`»
  replace-with: «the framing tier by Run B's four added pastes»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «four framing variants in 2.1.287»
  source: .andromeda/test-plan.md:1101 «and `2.1.287`'s five recorded framing variants, `UserPromptSubmit.paste-1.json`, `UserPromptSubmit.paste-2.json`, `UserPromptSubmit.paste-3.json`, `SessionEnd.clear-1.json` and `SessionStart.clear-1.json`»
  replace-with: «five framing variants in 2.1.287»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «2.1.287 stamped at seventeen rows»
  source: .andromeda/test-plan.md:666 «must list each of the eighteen literal ledger row ids exactly once, each with its post-condition result, and end `stamped <ver>  18 pass  0 fail`»
  replace-with: «2.1.287 stamped at eighteen rows»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «no recorded fixture of source `resume` exists yet, owed to the route entry "Paste newline ledger row"»
  source: .andromeda/test-plan.md:1093 «No reading became a fixture or a ledger row; the recorded payload is owed to the route entry "Owed ledger rows"»
  replace-with: «no recorded fixture of source `resume` exists yet, owed to the route entry "Owed ledger rows"»

- leaf: .claude/docs/tests-summary.md:26   class: stale
  leaf-text: «a shape with no ledger row yet»
  source: .andromeda/test-plan.md:752 «since chunk 2026-10-13-paste-newline-ledger-row the ledger row `paste-newline` and its `viola verify` probe, Run B's `paste-3` paste, cover that shape»
  replace-with: «a shape covered since chunk 2026-10-13-paste-newline-ledger-row by the ledger row `paste-newline` and its `viola verify` probe, Run B's `paste-3` paste (its Windows reading owed to "Windows-only live measurements")»

- leaf: .claude/rules/verification-harness.md:28   class: stale
  leaf-text: «(print probe + all four interactive runs, seventeen rows)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8 «stamp all eighteen ledger rows (`LedgerRow::ALL`)»
  replace-with: «(print probe + all four interactive runs, Run B with its four added pastes, eighteen rows)»
