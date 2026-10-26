# design · standing conventions

- S1 · Every new or changed human refusal is one ASCII two-space `unable` line on stderr with no colour, followed by its `hint:` line · body §Surface: cli (Streams, Component Patterns 2) and .andromeda/design-system-detail/surface.md
  (.andromeda/design-system-detail/brand-identity.md:18) expression-motion
  (.andromeda/design-system-detail/color-palette.md:53) status-states
  (.andromeda/design-system-detail/iconography.md:10) icons
- S2 · Every `--json` refusal is one stdout document with the typed exit code, stderr empty and no hint · body §Surface: cli (Streams, Exit-code phraseology) and .andromeda/design-system-detail/surface.md
  (.andromeda/design-system-detail/surface.md:93) cli-verify-run
  (.andromeda/design-system-detail/surface.md:104) cli-exit-codes
  (.andromeda/design-system-detail/surface.md:105) cli-exit-codes
  (.andromeda/design-system-detail/surface.md:106) cli-exit-codes
  (.andromeda/design-system-detail/surface.md:108) cli-exit-codes
- S3 · No human output, error or hint prints upstream text, a path, a pid or an anyhow chain with a serde source · body §Anti-Patterns (Per-Surface Bans, cli)
  (.andromeda/design-system-detail/color-palette.md:146) status-states
  (.andromeda/design-system-detail/surface.md:86) cli-verify-run
  (.andromeda/design-system-detail/surface.md:102) cli-exit-codes
- S4 · `viola hook` writes nothing to stderr and exits 0, and `viola mcp` writes only MCP frames on stdout · body §Surface: cli (Platform-Specific Notes)
  (.andromeda/design-system-detail/surface.md:103) cli-exit-codes
