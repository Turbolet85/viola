group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
read: .claude/rules/testing.md · 92 · «- 2026-10-11: A tree-equality assertion over a viola home co»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-11: On the Linux dev host `schema-check` (G4) read»

- leaf: .claude/docs/tests-summary.md:42   class: omission
  leaf-text: «both read caught on `windows-2025` in run 38036448183, chunk 2026-10-10-windows-mutation-grade)»
  source: .andromeda/test-plan.md:1226 «and the 16 in `src/cmd/verify/typed.rs` (`framing_turns` 5, `SharedPty` 5, `Run`'s ending sequence 6) are owed to the route entry "Epoch 4 survivors II"»
  replace-with: «both read caught on `windows-2025` in run 38036448183, chunk 2026-10-10-windows-mutation-grade; the Epoch 4 boundary audit's Linux whole-member runs read 47 missed, disposed at chunk 2026-10-13-epoch-4-survivors: 29 by a nextest case in the owning package (confirmed by the Epoch 5 boundary audit's own run), the 2 `prepare` rows as recorded, and the 16 in `src/cmd/verify/typed.rs` owed to the route entry "Epoch 4 survivors II")»
