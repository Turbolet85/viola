group: architecture+test-plan
read: .claude/docs/commands.md · 95 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:53   class: omission
  leaf-text: «and `clippy.toml` `disallowed-macros` bans the tracing level macros.»
  source: .andromeda/test-plan.md:1157 «Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13»
  replace-with: «and `clippy.toml` `disallowed-macros` bans the tracing level macros. Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints: `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings` (nothing linked, the target std installed as host preparation; neither a CI step nor a `pre-push` stage).»
