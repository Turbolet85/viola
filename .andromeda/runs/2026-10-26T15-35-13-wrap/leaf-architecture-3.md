group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»

- leaf: .claude/rules/events.md:26   class: retired
  leaf-text: «As landed only the implicit link is built: the driven wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new pair, for an issued `send` (directly before its `send-issued`) or a taken `answer` whose `from` is another instance's name;»
  source: .andromeda/architecture.md:220 «on an implicit link (an issued `send` or a taken `answer` from another instance, [Links]) or a served `link` call»
  replace-with: «The driven wrapper appends `link` (`source: wrapper`, `{driver, driven}`) once per new pair, for an issued `send` (directly before its `send-issued`), a taken `answer` whose `from` is another instance's name, or a served `link` call;»

- leaf: .claude/rules/events.md:26   class: retired
  leaf-text: «No product path appends `unlink`, and the `link` / `unlink` methods and verbs are not built (the wrapper answers both `-32601`).»
  source: .andromeda/architecture-detail/established-decisions.md:150 «The explicit `viola link` / `viola unlink` and their two channel methods are built, with no dated gap: the wrapper serves `link` and `unlink`»
  replace-with: «The wrapper serves `link` `{driver, driven}` → `{since, already_linked}` and `unlink` `{driver, driven}` → `{}` (the verbs `viola link` / `viola unlink <driver> <driven> [--json]`; neither method reads `from`): a `link` of a held pair appends nothing and answers its `since` with `already_linked: true`; `unlink` appends `unlink` (`source: wrapper`, `{driver, driven}`) once per held pair it removes (`LinkSet::remove`) and answers `{}` either way; a missing, non-string or invalid name, a `driven` that is not the wrapper's own name and a `driver` equal to `driven` are each `-32602` with no line; each answers only after its event line and the snapshot write.»

- leaf: .claude/rules/events.md:31   class: stale
  leaf-text: «(CLI `send` / `wait` / `last` / `answer` / `pause` / `release`, `hook.event` / `hook.dialog`, the MCP»
  source: .andromeda/architecture-detail/standard-contracts.md:51 «the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.event` and `hook.dialog`»
  replace-with: «(CLI `send` / `wait` / `last` / `answer` / `pause` / `release` / `link` / `unlink`, `hook.event` / `hook.dialog`, the MCP»
