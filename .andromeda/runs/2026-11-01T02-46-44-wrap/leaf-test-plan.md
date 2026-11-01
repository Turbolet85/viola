group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 107 · «- 2026-10-31: A case that runs on every OS must not call a h»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»
- leaf: .claude/docs/tests-summary.md:33   class: stale
  leaf-text: «secret scan + canary (after each job's end-to-end run, PROVISIONAL)»
  source: .andromeda/test-plan.md:606 «the no-`Cookie`, stripped-value and canary clauses are `viola-harness secret-scan`'s, after each job's end-to-end run (`v1-47`'s reading), the 0600 clause»
  replace-with: «secret scan + canary (after each job's end-to-end run)»
