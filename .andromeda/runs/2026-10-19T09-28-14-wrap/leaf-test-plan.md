group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 101 · «- 2026-10-17: A harness-session case never assumes the fake »
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/rules/testing.md:24   class: omission
  leaf-text: «so its table-driven cases are a labelled case table of `(label, …)` rows in one `#[test]`, each assertion naming its label).»
  source: .andromeda/test-plan.md:461 «`viola-core` carries no rstest dev-dependency either, and its table-driven cases are literal tables in one `#[test]`»
  replace-with: «so its table-driven cases are a labelled case table of `(label, …)` rows in one `#[test]`, each assertion naming its label; `viola-core` carries no rstest dev-dependency either, so its table-driven cases are literal tables in one `#[test]`, e.g. `dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred`).»
