group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»

- leaf: .claude/docs/gotchas.md:111   class: stale
  leaf-text: «(`<test>.report`, `<test>.test.report`)»
  source: .andromeda/architecture.md:415 «`<test name>.<pid>.report` and its sibling `<test name>.<pid>.test.report` (`<pid>` the test process's id, so two test processes running the same test never share a report file)»
  replace-with: «(`<test>.<pid>.report`, `<test>.<pid>.test.report`, `<pid>` the test process's id)»

- leaf: .claude/docs/services/viola-pty.md:33   class: stale
  leaf-text: «the child streams `<temp dir>/viola-pty-watch/<test>.report` (`start`, `size` from its watcher thread, `byte`, `restored`) and the test streams `<test>.test.report`»
  source: .andromeda/architecture.md:415 «`<test name>.<pid>.report` and its sibling `<test name>.<pid>.test.report` (`<pid>` the test process's id, so two test processes running the same test never share a report file)»
  replace-with: «the child streams `<temp dir>/viola-pty-watch/<test>.<pid>.report` (`<pid>` the test process's id, so two test processes running the same test never share a report file; `start`, `size` from its watcher thread, `byte`, `restored`) and the test streams `<test>.<pid>.test.report`»
