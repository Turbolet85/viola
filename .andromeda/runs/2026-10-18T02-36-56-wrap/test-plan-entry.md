
## 2026-10-17-optional-plugin-install — the plugin install suite and the reshaped help pin
**Section:** §5 Integration Test Strategy → Cross-module patterns covered → CLI
**Change:**
- `help_lists_only_served_verbs`: was "each served verb answers `--help` exit 0 … no `plugin` in the table"; now each served verb path answers (`plugin install --help` included), its usage the program then the verb's words, the table's words read from the group rows alone (`plugin install` one entry of `setup:`), no bare `plugin`.
- New: `tests/cli_plugin_install.rs` pins `viola plugin install [--json]` against `CARGO_BIN_EXE_viola`, each case in a fresh home (the tree and its `.mcp.json`, the re-run, the modes under `umask 000`, the `ok` document, the planning form, no escape byte, the widened-home refusal, the self pair, `viola plugin` exit 2), its units in `marketplace.rs`, `src/cmd/plugin.rs` and `src/human/verbs.rs` named.
**Why:** the chunk added the verb and moved the help pin; the master names the functions and states no case count.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/
