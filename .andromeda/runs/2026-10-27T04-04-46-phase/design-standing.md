# design · standing conventions

- S1 · Human output is ASCII only, with no colour, glyph or cursor control under `--json`, a non-TTY stdout, `NO_COLOR`, `TERM=dumb` or `viola run` · Surface: cli › Tokens (.andromeda/design-system-detail/surface.md)
  (.andromeda/design-system-detail/color-palette.md:12) palette
- S2 · Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr; under `--json` one document on stdout with the typed exit code and no hint · Surface: cli › Streams (.andromeda/design-system-detail/surface.md)
  (.andromeda/design-system-detail/surface.md:138) cli-exit-codes
- S3 · A refusal is `unable` plus a typed reason in standard phraseology, never "done" or "success" without context, never green / red prefixes or emoji · Anti-Patterns › Per-Surface Bans › cli
  (.andromeda/design-system-detail/color-palette.md:53) status-states
  (.andromeda/design-system-detail/color-palette.md:98) status-states
  (.andromeda/design-system-detail/color-palette.md:99) status-states
  (.andromeda/design-system-detail/color-palette.md:100) status-states
  (.andromeda/design-system-detail/color-palette.md:147) status-states
  (.andromeda/design-system-detail/color-palette.md:148) status-states
  (.andromeda/design-system-detail/color-palette.md:149) status-states
  (.andromeda/design-system-detail/color-palette.md:151) status-states
