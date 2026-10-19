group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola-state.md:6   class: retired
  leaf-text: «`viola mcp` resolves its home through the same `resolved_home`; the CLI verbs' step, and `mcp`'s, are not ratified, the founder's word owed)»
  source: .andromeda/architecture.md:323 «`viola mcp` (chunk 2026-10-17-mcp-server-for-drivers) resolves its home through the same `resolved_home` and reads `VIOLA_NAME` for its own name (the `from` it adds and its detail sink).»
  replace-with: «`viola mcp` resolves its home through the same `resolved_home`)»

- leaf: .claude/docs/services/viola.md:20   class: retired
  leaf-text: «before the collision check, the pin and the endpoint bind; not ratified, the founder's word owed)»
  source: .andromeda/architecture.md:402 «`run`'s start canonicalises the home after its wrapper log has created it, before the collision check, the pin and the endpoint bind.»
  replace-with: «before the collision check, the pin and the endpoint bind)»

- leaf: .claude/docs/services/viola-mcp.md:14   class: omission
  leaf-text: «the tools reach a session as `mcp__plugin_viola_viola__<tool>` (as measured live on claude 2.1.287).»
  source: .andromeda/architecture.md:96 «A wrapped session that also has the installed plugin enabled starts one `viola` MCP server, the `--plugin-dir` plugin's, and that server answers its tools: the installed plugin's `viola mcp` never starts there»
  replace-with: «the tools reach a session as `mcp__plugin_viola_viola__<tool>` (as measured live on claude 2.1.287). A wrapped session that also has the installed plugin enabled starts one `viola` MCP server, the `--plugin-dir` plugin's, which answers its tools; the installed plugin's `viola mcp` never starts there (as measured on claude 2.1.287 on the Linux dev host; no capability-ledger row; its Windows leg owed to "Windows-only live measurements").»

- leaf: .claude/docs/services/viola-mcp.md:23   class: omission
  leaf-text: «a refusal exits 1 with nothing on stdout or stderr and no role file.»
  source: .andromeda/architecture.md:95 «A registered `viola mcp` that exits 1 (its home refused by `home_passes`) does not block the CLI's start: the session reaches `idle` and the CLI logs that server's connection as failed»
  replace-with: «a refusal exits 1 with nothing on stdout or stderr and no role file, and does not block the CLI's start: the session reaches `idle` and the CLI logs that server's connection as failed (as measured on claude 2.1.287 on the Linux dev host; its Windows leg owed to "Windows-only live measurements").»
