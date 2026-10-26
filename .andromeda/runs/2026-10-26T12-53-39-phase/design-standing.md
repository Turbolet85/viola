# design · standing conventions

- S1 · Every new human output line is ASCII only, uses two spaces between columns, is never wrapped, and is never truncated where the terminal width is unknown · Surface: cli › Width (.andromeda/design-system-detail/surface.md, cli-output)
  (.andromeda/design-system-detail/iconography.md:10) icons
  (.andromeda/design-system-detail/surface.md:27) cli-output
- S2 · Results go to stdout. Refusals, `hint:` lines and errors go to stderr. Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code, and no `hint:` line is printed · Surface: cli › Streams (.andromeda/design-system-detail/surface.md, cli-output)
- S3 · The colour decision order applies. There is no SGR, glyph or cursor control under `--json`, on a non-TTY, under `NO_COLOR` or under `TERM=dumb`, and colour is spent only on `DIALOG` (amber) and `stale` (dim) · Surface: cli › Tokens (.andromeda/design-system-detail/surface.md, cli-tokens) and Per-Surface Bans › cli
  (.andromeda/design-system-detail/brand-identity.md:18) expression-motion
  (.andromeda/design-system-detail/brand-identity.md:21) expression-motion
  (.andromeda/design-system-detail/color-palette.md:12) palette
  (.andromeda/design-system-detail/color-palette.md:74) status-states
  (.andromeda/design-system-detail/color-palette.md:75) status-states
  (.andromeda/design-system-detail/color-palette.md:76) status-states
  (.andromeda/design-system-detail/color-palette.md:77) status-states
  (.andromeda/design-system-detail/color-palette.md:79) status-states
  (.andromeda/design-system-detail/color-palette.md:88) status-states
  (.andromeda/design-system-detail/surface.md:8) cli-tokens
  (.andromeda/design-system-detail/surface.md:11) cli-tokens
  (.andromeda/design-system-detail/surface.md:15) cli-tokens
- S4 · Every refusal, error and hint is a fixed message. None carries a path, pid, endpoint, start time, upstream text, anyhow chain or stack trace, and full detail goes only to `diagnostics/` · Per-Surface Bans › cli
  (.andromeda/design-system-detail/color-palette.md:146) status-states
