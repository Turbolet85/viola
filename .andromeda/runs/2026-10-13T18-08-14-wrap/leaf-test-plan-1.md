group: test-plan-1
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
read: .claude/rules/testing.md · 93 · «- 2026-10-13: A `+=` → `-=` mutant on an unsigned sum that s»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «disposed at chunk 2026-10-13-epoch-4-survivors: 29 by a nextest case in the owning package (confirmed by the Epoch 5 boundary audit's own run), the 2 `prepare` rows as recorded, and the 16 in `src/cmd/verify/typed.rs` owed to the route entry "Epoch 4 survivors II"»
  source: .andromeda/test-plan.md:1226 «and the 16 in `src/cmd/verify/typed.rs` (`framing_turns` 5, `SharedPty` 5, `Run`'s ending sequence 6) were disposed by chunk 2026-10-13-epoch-4-survivors-ii in its ledger»
  replace-with: «disposed: 29 by a nextest case in the owning package at chunk 2026-10-13-epoch-4-survivors, the 2 `prepare` rows as recorded, and the 16 in `src/cmd/verify/typed.rs` by a case in `typed.rs`'s `mod tests` after a behaviour-preserving restatement at chunk 2026-10-13-epoch-4-survivors-ii, none owed; the Epoch 5 boundary audit's own run is the reader that confirms the 45 kills»
