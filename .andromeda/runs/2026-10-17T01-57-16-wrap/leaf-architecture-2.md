group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»

- leaf: .claude/docs/services/viola.md:48   class: omission
  leaf-text: «answers `agents --json` from `agents-mode`, `agents-rows.json` and the live `sessions/<pid>.json` registry beside its own executable)»
  source: .andromeda/architecture.md:367 «the session's `bin/claude[.exe]` copy and, beside it, `bin/agents-mode` — `boot --agents-mode recorded|oversize|malformed`'s value for the fake agent's `agents` mode»
  replace-with: «answers `agents --json` from `agents-mode` (`recorded` · `oversize` · `malformed`; under the harness, `boot --agents-mode`'s value in the session's `bin/agents-mode`), `agents-rows.json` and the live `sessions/<pid>.json` registry beside its own executable)»
