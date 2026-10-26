group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »

- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «on `send` / `wait` / `last` / `answer` / `pause` / `release`, a server that is not the recorded wrapper»
  source: .andromeda/architecture-detail/conventions.md:40 «Exit `21` also covers, on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, a server that is not the recorded wrapper»
  replace-with: «on `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink`, a server that is not the recorded wrapper»

- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «`marketplace/` tree dir, nothing created, no log opened)»
  source: .andromeda/architecture-detail/conventions.md:34 «`viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens) is the same exit and document»
  replace-with: «`marketplace/` tree dir, nothing created, no log opened; and `viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens), no log opened, no line written)»

- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «20 wrapper fault»
  source: .andromeda/architecture-detail/conventions.md:33 «`20` protocol fault from the wrapper, a `-32602` from `link` / `unlink` included»
  replace-with: «20 wrapper fault (a `-32602` from `link` / `unlink` included)»

- leaf: .claude/docs/services/viola-channel.md:26   class: retired
  leaf-text: «`pause` and `release`; it answers `link` and `unlink` `-32601` (neither is built).»
  source: .andromeda/architecture.md:193 «`link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}`, each sent to the driven instance's wrapper»
  replace-with: «`pause`, `release`, `link` (→ `{since, already_linked}`) and `unlink` (→ `{}`), the last two sent to the driven instance's wrapper.»

- leaf: .claude/docs/services/viola-channel.md:25   class: stale
  leaf-text: «(CLI `send` / `wait` / `last` / `answer` / `pause` / `release`, `viola hook`'s»
  source: .andromeda/architecture-detail/standard-contracts.md:51 «Before its first frame, the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.event` and `hook.dialog`»
  replace-with: «(CLI `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink`, `viola hook`'s»
