group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/conventions.md:13   class: stale
  leaf-text: «The two ratified exceptions are the test seams `FAKE_AGENT_PUMP_DELAY_MS` (u64 ms capped at 5 000; security-plan Decisions Log 2026-09-27) and `FAKE_AGENT_HOOK_PANIC` (fires only on exactly `1`, a forced fail-open `hook` panic; Decisions Log 2026-09-28), neither prefix: each read only by a `viola` built with the `fake-agent` feature, absent from release builds, configuring nothing (arch §Occupied Resources → Environment variables).»
  source: .andromeda/architecture.md:157 «A third ratified exception, in every build: `NO_COLOR`, `TERM` and `COLORTERM`, the terminal's own cross-tool names, which the product reads (never sets)»
  replace-with: «Two of the ratified exceptions are the test seams `FAKE_AGENT_PUMP_DELAY_MS` (u64 ms capped at 5 000; security-plan Decisions Log 2026-09-27) and `FAKE_AGENT_HOOK_PANIC` (fires only on exactly `1`, a forced fail-open `hook` panic; Decisions Log 2026-09-28), neither prefix: each read only by a `viola` built with the `fake-agent` feature, absent from release builds, configuring nothing. A third, in every build: `NO_COLOR`, `TERM` and `COLORTERM`, the terminal's own names, read (never set) only by viola's colour layer (`src/human/style.rs`, `viola list`'s human mode) to decide whether and how deep its SGR bytes are written — never echoed, configuring nothing, disabling no control and widening no redaction (the founder's own word, chunk 2026-10-17-cli-output-discipline; arch §Occupied Resources → Environment variables).»
