group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola-mcp.md:32   class: stale
  leaf-text: «with the rmcp client (none in the tree yet: they land with "MCP server for drivers")»
  source: .andromeda/architecture.md:30 «its E2E owning the `viola mcp` child and awaiting its exit (chunk 2026-10-17-mcp-server-for-drivers)»
  replace-with: «with the rmcp client (feature `client` only): `mcp_tools.rs`, landed at chunk 2026-10-17-mcp-server-for-drivers»

- leaf: .claude/docs/services/viola-mcp.md:36   class: tree
  leaf-text: «`mcp_schema_violations.rs`, Paths 2–5 MCP legs, `cross_exit_causes.rs` (CLI/MCP parity), none in the tree yet: they land with "MCP server for drivers"»
  source: .andromeda/test-plan.md:844 «— no `mcp_schema_violations.rs` binary (chunk 2026-10-17-mcp-server-for-drivers) —»
  replace-with: «`crates/viola-e2e/tests/mcp_tools.rs` (the Paths 2–6 MCP legs, each result compared with the CLI's `--json` document less `v`; `tools/list` exactly the five tools and the schema violations in `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`)»

- leaf: .claude/docs/services/viola-pty.md:27   class: stale
  leaf-text: «the pin is re-decided on current evidence by the PTY chunk (requirements v1-25)»
  source: .andromeda/architecture.md:44 «0.9.0 is excluded because of the Windows garbage-read bug wezterm#6783. Swap triggers: portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run`»
  replace-with: «it is excluded, and the swap trigger is portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run`»
