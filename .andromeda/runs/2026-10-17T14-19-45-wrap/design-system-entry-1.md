
## 2026-10-17-mcp-server-for-drivers — viola mcp, a hidden verb
**Section:** §Surface: cli → Platform-Specific Notes
**Change:** the `mcp` note adds that `mcp` is a hidden verb: run by the plugin, never by a person, it appears in no `viola --help` row.
**Why:** the chunk built `viola mcp` with `#[command(hide = true)]`; `--help` stays the served verbs' table.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
