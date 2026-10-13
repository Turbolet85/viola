group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 93 · «- 2026-10-13: A `+=` → `-=` mutant on an unsigned sum that s»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-11: On the Linux dev host `schema-check` (G4) read»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «nine `mutants (<label>)` jobs, the root package split by file»
  source: .andromeda/test-plan.md:1163 «ten `mutants (<label>)` jobs on `windows-2025` (`fail-fast: false`, 120 min), one per `matrix.include` item (the count of its `- package:` items; the root package `viola` is five of them»
  replace-with: «ten `mutants (<label>)` jobs, the root package split by file into five, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`»

- leaf: .claude/rules/verification-harness.md:44   class: stale
  leaf-text: «nine `mutants (<label>)` jobs with the root package split by file»
  source: .andromeda/test-plan.md:1163 «ten `mutants (<label>)` jobs on `windows-2025` (`fail-fast: false`, 120 min), one per `matrix.include` item (the count of its `- package:` items; the root package `viola` is five of them»
  replace-with: «ten `mutants (<label>)` jobs with the root package split by file into five, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`»
