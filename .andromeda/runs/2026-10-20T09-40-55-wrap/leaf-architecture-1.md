group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
- leaf: .claude/docs/gotchas.md:133   class: omission
  leaf-text: «(the keepers make the target).»
  source: .andromeda/architecture.md:388 «A linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` (`f_bavail × f_frsize`) is refused too»
  replace-with: «(the keepers make the target, and refuse a start whose linked base's filesystem reads less than the 9 GiB `HEADROOM_FLOOR_MIB` free, with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`).»
