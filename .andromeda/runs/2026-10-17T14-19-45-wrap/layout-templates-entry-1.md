
## 2026-10-17-mcp-server-for-drivers — viola mcp in --help and as the driver's machine view
**Section:** §Surface: cli → Output structure — `viola --help` (As built); Primary screens (commands)
**Change:**
- `--help` As built: `viola mcp` is a hidden verb (no argument but `--home`), run only by the plugin (`.mcp.json` registers the pinned path with `["mcp"]`); it appears in no row and the `HELP` literal is unchanged.
- Primary screens: `viola mcp` is no screen; its five tools are a driver's machine view of those verbs — each success and refusal the CLI verb's `--json` document less `v`, a refusal `isError: true` with text `refused: <code>`, bodies codes only (`invalid-params` with a closed field word), and no tool, description or result naming `release`, `pause`, `link` or `unlink`.
**Why:** the chunk built the hidden verb and its tools; `layout-templates.md`'s "no human surface" stays true.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
