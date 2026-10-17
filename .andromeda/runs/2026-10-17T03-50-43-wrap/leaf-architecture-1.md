group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/conventions.md:33   class: stale
  leaf-text: «or 2 for a usage error, and takes no `--json` yet.»
  source: .andromeda/architecture.md:98 «`viola revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check or a preflight refusal prints `{"v":1,"error":"<code>","detail":null}`»
  replace-with: «or 2 for a usage error; it takes `--json` on both arms, where a refused strict-modes check or a preflight refusal is instead one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`), stderr empty, exit 1, and `<name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order (a refusal `run`'s start makes after a passed preflight prints no document; owed to "Exit-cause code catalogue").»

- leaf: .claude/docs/conventions.md:44   class: stale
  leaf-text: «(as landed, the CLI verbs take `--home`, else the default, and only `hook` takes the `VIOLA_DIR` step; that step for the CLI verbs is owed to the route entry "CLI machine contract")»
  source: .andromeda/architecture.md:400 «`resolve_home(flag, name, dir, user_home)` in `src/cmd/mod.rs` resolves `--home` as given, else the `instance_of` home when both `VIOLA_NAME` and `VIOLA_DIR` pass the one shape check»
  replace-with: «(as landed, `hook` and every CLI verb, `run` and `revive` included, take `--home`, else the `VIOLA_DIR` grandparent when `VIOLA_NAME` and `VIOLA_DIR` pass `instance_of`'s shape check (a failing pair falls through to the default, never refusing), else the default; a home that exists is canonicalised; the CLI verbs' `VIOLA_DIR` step and the canonicalisation are NOT ratified, the founder's own word owed)»
