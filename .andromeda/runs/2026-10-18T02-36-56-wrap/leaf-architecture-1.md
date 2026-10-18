group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/services/viola-agent-claude.md:23   class: retired
  leaf-text: «plugin precedence, local commands + post-conditions»
  source: .andromeda/architecture.md:87 «The `agents --json` join landed at chunk 2026-10-16-unwrapped-sessions-on-the-board (row `agents-join`); plugin precedence is no row ([Plugin Scope]).»
  replace-with: «local commands + post-conditions»

- leaf: .claude/docs/services/viola-agent-claude.md:6   class: omission
  leaf-text: «No adapter trait yet: a second agent arrives as a sibling `viola-agent-<name>` crate.»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:78 «`marketplace` module (`plugin install`'s three documents, `NAME`, `PLUGIN_REF`)»
  replace-with: «It also holds the pure `marketplace` module, `viola plugin install`'s three documents built by `serde_json` (`marketplace_files`; `NAME` the marketplace `viola`, `PLUGIN_REF` `viola@viola`; its `.mcp.json` registers the MCP server `viola`, no hooks). No adapter trait yet: a second agent arrives as a sibling `viola-agent-<name>` crate.»

- leaf: .claude/docs/services/viola-mcp.md:14   class: omission
  leaf-text: «the pinned copy's absolute path substituted at every start;»
  source: .andromeda/architecture.md:96 «its `.mcp.json` registers `viola` as `{"command": <the canonical current_exe, forward slashes>, "args": ["--home", <the canonical home, forward slashes>, "mcp"]}`»
  replace-with: «the pinned copy's absolute path substituted at every start, and, for an unwrapped driver, the `.mcp.json` of `viola plugin install`'s local marketplace (`viola@viola`, no hooks), which registers `viola` as `{"command": <the canonical current_exe, forward slashes>, "args": ["--home", <the canonical home, forward slashes>, "mcp"]}`;»

- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «which refuses the whole board, no log opened);»
  source: .andromeda/architecture.md:136 «`viola plugin install`'s refused strict-modes check of the home or an existing `marketplace/` tree dir is the same exit and document (human mode the home-state pair; nothing created, no log opened)»
  replace-with: «which refuses the whole board, no log opened; and `viola plugin install`'s refused strict-modes check of the home or an existing `marketplace/` tree dir, nothing created, no log opened);»
