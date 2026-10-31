# design · standing conventions

- S1 · Each new refusal or hint line is ASCII only and is never wrapped · ## Surface: cli, Width
- S2 · Each new refusal or hint line carries no colour, glyph or SGR, and no such byte reaches stdout or stderr under `--json` · ## Anti-Patterns, Per-Surface Bans, cli
  (.andromeda/design-system-detail/brand-identity.md:18) expression-motion
  (.andromeda/design-system-detail/color-palette.md:8) palette
  (.andromeda/design-system-detail/color-palette.md:12) palette
  (.andromeda/design-system-detail/color-palette.md:35) palette
  (.andromeda/design-system-detail/color-palette.md:79) status-states
  (.andromeda/design-system-detail/color-palette.md:96) status-states
  (.andromeda/design-system-detail/color-palette.md:98) status-states
  (.andromeda/design-system-detail/color-palette.md:99) status-states
  (.andromeda/design-system-detail/color-palette.md:100) status-states
  (.andromeda/design-system-detail/color-palette.md:101) status-states
  (.andromeda/design-system-detail/color-palette.md:102) status-states
  (.andromeda/design-system-detail/color-palette.md:103) status-states
  (.andromeda/design-system-detail/color-palette.md:104) status-states
  (.andromeda/design-system-detail/color-palette.md:105) status-states
  (.andromeda/design-system-detail/color-palette.md:106) status-states
  (.andromeda/design-system-detail/color-palette.md:107) status-states
  (.andromeda/design-system-detail/color-palette.md:108) status-states
  (.andromeda/design-system-detail/color-palette.md:109) status-states
  (.andromeda/design-system-detail/color-palette.md:110) status-states
  (.andromeda/design-system-detail/color-palette.md:111) status-states
  (.andromeda/design-system-detail/color-palette.md:112) status-states
  (.andromeda/design-system-detail/surface.md:13) cli-tokens
  (.andromeda/design-system-detail/surface.md:14) cli-tokens
  (.andromeda/design-system-detail/surface.md:15) cli-tokens
- S3 · Each new message is fixed text with no stack trace, and results never mix with refusals and hints across stdout and stderr · ## Anti-Patterns, Per-Surface Bans, cli
- S4 · Each new human word or code is short fixed phraseology with a typed reason, no filler and no guessing · ## Brand Identity, Domain anchors
