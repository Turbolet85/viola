group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »

- leaf: .claude/docs/services/viola-agent-claude.md:11   class: stale
  leaf-text: «`SIGNATURES` (`input_box: ["for agents"]`, `modals: ["Yes, I trust this folder", "Yes, allow external imports"]`)»
  source: .andromeda/architecture.md:45 «The signatures are compiled in `viola_agent_claude::screen::SIGNATURES`: `input_box: ["for agents", "(shift+tab to cycle)"]` and `modals: ["Yes, I trust this folder", "Yes, allow external imports"]`»
  replace-with: «`SIGNATURES` (`input_box: ["for agents", "(shift+tab to cycle)"]`, the second the mode-independent resting footer a CLI keeps once it hides the first (2.1.302), `modals: ["Yes, I trust this folder", "Yes, allow external imports"]`)»
