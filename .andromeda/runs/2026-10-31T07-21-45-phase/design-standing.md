# design · standing conventions

- S1 · every new or changed human refusal or error line goes to stderr, results to stdout, data and messages never mixed · Surface: cli, Streams; Anti-Patterns, Per-Surface Bans (cli)
- S2 · human output stays ASCII only and lines are never wrapped, with no emoji and no check or cross prefix · Surface: cli, Width; Anti-Patterns, Per-Surface Bans (cli)
- S3 · nothing new emits colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR` or `TERM=dumb` · Anti-Patterns, Per-Surface Bans (cli)
  (.andromeda/design-system-detail/brand-identity.md:21) expression-motion
- S4 · each new code or fixed message is short standard phraseology, `unable` plus a typed reason, with no filler and no guessing · Brand Identity, Domain anchors
  (.andromeda/design-system-detail/color-palette.md:53) status-states
  (.andromeda/design-system-detail/color-palette.md:98) status-states
  (.andromeda/design-system-detail/color-palette.md:99) status-states
  (.andromeda/design-system-detail/color-palette.md:100) status-states
