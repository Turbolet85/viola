group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:13   class: omission
  leaf-text: «`cargo install --path .` — install `viola` from the repo root»
  source: .andromeda/architecture.md:301 «`plugin install` takes `[--json]` and nothing else (`viola plugin` alone is a clap usage error; clap's own `viola plugin help` is served); it exits 0 with its result (one stdout write), 21 when»
  replace-with: «`cargo install --path .` — install `viola` from the repo root; then, optional and only for the MCP verbs in an unwrapped driver, `viola plugin install [--json]` — writes only the local marketplace `viola` under `<home>/marketplace/` (plugin `viola@viola`, its `.mcp.json` registering the MCP server `viola` as the absolute exe path with `["--home", <home>, "mcp"]`, no hooks) and spawns no `claude`; exit 0 with its result (`--json` `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`, no path), 21 when its strict-modes check refuses the home or any existing `marketplace/` tree dir (nothing created, no log opened), 1 for an internal error, 2 for a clap usage error (`viola plugin` alone)»
