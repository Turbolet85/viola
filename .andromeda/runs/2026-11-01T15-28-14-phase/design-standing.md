# design · standing conventions

- S1 · each new refusal line goes to stderr with its hint line directly after it, results staying on stdout · Surface: cli › Streams (.andromeda/design-system-detail/surface.md)
- S2 · under `--json` each new refusal is one stdout document with the typed exit code, no hint line and stderr empty · Surface: cli › Streams (.andromeda/design-system-detail/surface.md)
  (.andromeda/design-system-detail/surface.md:167) cli-exit-codes
- S3 · each new line is plain ASCII with no colour, glyph or cursor control · Anti-Patterns › Per-Surface Bans › cli
  (.andromeda/design-system-detail/brand-identity.md:18) expression-motion
  (.andromeda/design-system-detail/color-palette.md:12) palette
  (.andromeda/design-system-detail/color-palette.md:35) palette
  (.andromeda/design-system-detail/color-palette.md:53) status-states
  (.andromeda/design-system-detail/color-palette.md:79) status-states
  (.andromeda/design-system-detail/surface.md:8) cli-tokens
  (.andromeda/design-system-detail/surface.md:11) cli-tokens
  (.andromeda/design-system-detail/surface.md:12) cli-tokens
  (.andromeda/design-system-detail/surface.md:13) cli-tokens
  (.andromeda/design-system-detail/surface.md:14) cli-tokens
  (.andromeda/design-system-detail/surface.md:15) cli-tokens
  (.andromeda/design-system-detail/surface.md:110) cli-verbs
- S4 · each new refusal or hint prints no path, pid, upstream text or anyhow chain · Anti-Patterns › Per-Surface Bans › cli
  (.andromeda/design-system-detail/color-palette.md:146) status-states
- S5 · a new fault prints no stack trace, its full detail going only to the instance's diagnostics · Surface: cli › Platform-Specific Notes
- S6 · each new word is short fixed phraseology, `unable` plus a typed reason, with no filler and no guessing · Brand Identity › Domain anchors
  (.andromeda/design-system-detail/color-palette.md:147) status-states
  (.andromeda/design-system-detail/color-palette.md:148) status-states
  (.andromeda/design-system-detail/color-palette.md:149) status-states
  (.andromeda/design-system-detail/color-palette.md:151) status-states
- S7 · each new hint is one line per reason, detail or cause, names no `viola release`, quotes no upstream text, and a fault or `unknown` takes none · Surface: cli › Component Patterns, pattern 2 (.andromeda/design-system-detail/surface.md)
