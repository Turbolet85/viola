group: architecture+test-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:39   class: omission
  leaf-text: «— doctests (nextest cannot run them)»
  source: .andromeda/test-plan-detail/test-strategy.md:11 «**Runner:** nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported»
  replace-with: «— doctests (nextest cannot run them); otherwise nextest is the runner: a plain `cargo test` run (`cargo test --bin viola`) is not supported, and under it three root-bin cases are known-cause reds (test-plan §2 Runner)»
