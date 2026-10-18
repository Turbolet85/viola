group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:15   class: stale
  leaf-text: «Every exec-form `command` in `hooks.json`, `.mcp.json`, `viola plugin install`'s `.mcp.json` and the `settings.json` statusline is the absolute pinned path — never a PATH lookup.»
  source: .andromeda/security-plan.md:251 «`marketplace/` is written only by `viola plugin install`: two manifests and `viola/.mcp.json`, whose `mcpServers.viola` `command` is the canonical exe that ran the install»
  replace-with: «Every exec-form `command` in `hooks.json`, `.mcp.json` and the `settings.json` statusline is the absolute pinned path, and in `viola plugin install`'s `marketplace/viola/.mcp.json` the canonical exe that ran the install (`args` `["--home", <the canonical home>, "mcp"]`) — never a PATH lookup.»

- leaf: .claude/rules/security.md:14   class: omission
  leaf-text: «at every entry point that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`.»
  source: .andromeda/security-plan.md:185 «`viola plugin install` checks only its home and each existing `marketplace/` tree dir through `check_path` before anything under the home is created, written or opened»
  replace-with: «at every entry point that reads `snapshot.json`, `ledger/stamps.json` or `statusline_command`. `viola plugin install` runs `check_path` only on its home and each existing `marketplace/` tree dir before anything under the home is created, written or opened (a refusal exits 21, `instance-unreachable` under `--json`, nothing created, no log opened); it alone writes `marketplace/` (two manifests and `viola/.mcp.json`, built by `serde_json`, no hooks in any document, each file rewritten whole at 0600 through `replace_private` on every run, each dir 0700, any other entry left as found), never reads the tree back nor re-hashes its command (`marketplace/` is no trusted file; its integrity owed to the route entry "Home and code-bearing file integrity"), spawns no `claude` and writes nothing under `~/.claude`.»

- leaf: .claude/docs/security-summary.md:35   class: stale
  leaf-text: «NEVER let an exec-form command viola writes resolve `viola` through PATH — absolute pinned path only.»
  source: .andromeda/security-plan.md:251 «whose `mcpServers.viola` `command` is the canonical exe that ran the install and whose `args` are `["--home", <the canonical home>, "mcp"]`»
  replace-with: «NEVER let an exec-form command viola writes resolve `viola` through PATH — the absolute pinned path only, or, in `viola plugin install`'s `.mcp.json`, the absolute path of the exe that ran the install.»

- leaf: .claude/docs/security-summary.md:17   class: omission
  leaf-text: «single writers for snapshot and stamps.»
  source: .andromeda/security-plan.md:251 «`marketplace/` is written only by `viola plugin install`: two manifests and `viola/.mcp.json`, whose `mcpServers.viola` `command` is the canonical exe that ran the install»
  replace-with: «single writers for snapshot and stamps; `marketplace/` (whose `viola/.mcp.json` Claude Code runs for MCP in a session that installed the plugin) written only by `viola plugin install`, after `check_path` on the home and each existing `marketplace/` dir (a refusal exits 21, nothing created), each file rewritten whole at 0600, never read back and its command not re-hashed — its integrity owed to the route entry "Home and code-bearing file integrity".»

- leaf: .claude/docs/security-summary.md:28   class: omission
  leaf-text: «`plugin/*/hooks.json`, pinned `bin/`»
  source: .andromeda/security-plan.md:39 «`marketplace/viola/.mcp.json`, written by `viola plugin install`, holds the absolute exec path Claude Code runs for MCP in a session that installed the plugin.»
  replace-with: «`plugin/*/hooks.json`, `marketplace/viola/.mcp.json` (written by `viola plugin install`), pinned `bin/`»

- leaf: .claude/docs/security-summary.md:73   class: omission
  leaf-text: «`viola` never names the backing; no CI runner has the link.»
  source: .andromeda/security-plan.md:262 «a live round that installs the plugin runs the CLI's own four commands (`claude plugin marketplace add` / `install` / `uninstall` / `marketplace remove`) at `--scope local`»
  replace-with: «`viola` never names the backing; no CI runner has the link. Accepted residual (a live round of `viola plugin install`): the round runs the CLI's own four commands (`claude plugin marketplace add` / `install` / `uninstall` / `marketplace remove`) at `--scope local` from a rig dir under `target/` that is its own git repo (the CLI takes the local scope's project from the git work tree root); the CLI's writes under `~/.claude` are its own — after the removal its orphaned plugin cache `~/.claude/plugins/cache/viola/viola/<version>/` stays, `installed_plugins.json` and `known_marketplaces.json` back as before (as measured on claude 2.1.287 at chunk 2026-10-17-optional-plugin-install); viola itself writes nothing under `~/.claude`.»
