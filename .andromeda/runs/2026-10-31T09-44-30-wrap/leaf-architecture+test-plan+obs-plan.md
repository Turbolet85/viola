group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»
- leaf: .claude/docs/commands.md:22   class: stale
  leaf-text: «(exit 0 once initialized, 11 before `initialize`)»
  source: .andromeda/architecture-detail/occupied-resources.md:29 «which ends the session as a close does (exit 0 once initialized, 1 before `initialize`, both streams empty)»
  replace-with: «(exit 0 once initialized, 1 before `initialize`, both streams empty)»
