
## 2026-10-17-optional-plugin-install — plugin install's exits, document, names and files
**Section:** §Conventions (CLI exit codes 1, 21); §Standard Contracts CLI `--json` output; §Occupied Resources (Binary, subcommands and exit codes; Claude Code integration names; Filesystem `marketplace/`, `diagnostics/`); §Infrastructure Patterns → Project directory structure
**Change:**
- Subcommands: `plugin install` takes `[--json]` and nothing else (`viola plugin` alone a usage error; clap's own `viola plugin help` served); exits 0 / 21 (its strict-modes check refusing the home or an existing `marketplace/` tree dir, nothing created, no log opened) / 1 (the `cli` catch site) / 2.
- Exit 21 gains its refusal (the same document; human mode the home-state pair); exit 1's catch-site verbs gain `plugin install`.
- `--json`: `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}` (exit 0, no path); its refusal the `instance-unreachable` document through `client::refused_document`.
- Integration names: the local marketplace `viola` (`marketplace::NAME`), one plugin `viola` (source `./viola`), added as `viola@viola` (`PLUGIN_REF`), its `.mcp.json` registering server `viola`, no hooks.
- Filesystem: was "`marketplace/`: written only by `viola plugin install`"; now with its dirs `.claude-plugin/`, `viola/`, `viola/.claude-plugin/` (0700, `create_private_dir`) and three files (0600, `replace_private`, rewritten whole), after its strict-modes check, any other entry left as found, never read back. `diagnostics/`: `plugin install` writes the self pair into the caller's `cli-<VIOLA_NAME>.ndjson` only under a parsed name and after its check passed.
- Directory structure: `cmd/plugin.rs`; `human/verbs.rs`'s install lines (`write_plugin_installed`, `command_word`, a Windows-gated source in `windows-mutants.yml`'s `viola-main` job); `human.rs`'s callers gain `plugin install`; `viola-agent-claude`'s pure `marketplace` module.
**Why:** the chunk built the verb; its argument and exit shape follow `viola list`'s strict-check-before-log order.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/
