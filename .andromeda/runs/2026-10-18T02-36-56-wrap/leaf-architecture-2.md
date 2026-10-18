group: architecture-2
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 9 · «None: the `viola_ui` log target in `src/obs.rs`, the reserve»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «called by `run`, `revive`, `verify`, `send`, `wait`, `last`, `list`, `answer`, `pause`, `release` and the `main` catch site»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:21 «called by `run`, `revive`, `verify`, `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `plugin install` and the `main` catch site»
  replace-with: «called by `run`, `revive`, `verify`, `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `plugin install` and the `main` catch site»

- leaf: .claude/docs/services/viola.md:34   class: stale
  leaf-text: «`send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list`;»
  source: .andromeda/architecture.md:128 «so `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list` and `plugin install`)»
  replace-with: «`send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list`, `plugin install`;»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «the five tools over stdio until stdin closes)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:30 «plugin.rs: `plugin install` (`check_path` on the home and the four `marketplace/` tree dirs before any write or log, exit 21 on a refusal; then the tree, the `ok` document or the two printed commands);»
  replace-with: «the five tools over stdio until stdin closes); **Plugin verb:** `src/cmd/plugin.rs` (`viola plugin install [--json]`: `check_path` on the home and the four `marketplace/` tree dirs before any write or log, exit 21 on a refusal (nothing created, no log opened); then the `marketplace/` tree (dirs 0700 through `create_private_dir`, `.claude-plugin/marketplace.json`, `viola/.claude-plugin/plugin.json` and `viola/.mcp.json` 0600 through `replace_private`, rewritten whole on every run, the documents from `viola_agent_claude::marketplace::marketplace_files`, no `claude` spawned), the `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` document or the two printed commands (`write_plugin_installed` in `src/human/verbs.rs`); exits 0 / 21 / 1 / 2)»
