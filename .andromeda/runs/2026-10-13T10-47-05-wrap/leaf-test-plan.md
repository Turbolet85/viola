group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
read: .claude/rules/testing.md · 92 · «- 2026-10-11: A tree-equality assertion over a viola home co»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-11: On the Linux dev host `schema-check` (G4) read»

- leaf: .claude/rules/verification-harness.md:44   class: omission
  leaf-text: «measures, removes and recreates it (`scratch-wipe-failed`), and runs `cargo mutants` with `TMP`/`TEMP` and `--output` set to it»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:45 «the re-creation through `viola_state::fs::create_private_dir`: the scratch takes the protected user + SYSTEM DACL, so every temp copy and test home laid inside it inherits owner-only»
  replace-with: «measures, removes and recreates it (`scratch-wipe-failed`), the re-creation through `viola_state::fs::create_private_dir` so the scratch takes the protected user + SYSTEM DACL and every temp copy and test home inside it inherits owner-only (a plain re-create inherited BUILTIN\Users write and failed the instance check, run 38215104120; pinned by the `#[cfg(windows)]` unit `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not`), and runs `cargo mutants` with `TMP`/`TEMP` and `--output` set to it»
