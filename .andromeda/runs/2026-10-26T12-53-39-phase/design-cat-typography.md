# design · Typography — category read

## Carried
- (.andromeda/design-system.md:139) The `viola link` / `viola unlink` result lines (`link … since …Z`, `link … already linked`, `unlink …`) and their refusals are cli output: they must be set in the user's terminal monospace with the brand carried only by fixed columns and phraseology words, must be ASCII only (`->`, never a non-ASCII arrow), and must carry no bold, since bold is reserved for the NAME column of `viola list` and SGR dim and amber for `stale` rows and the `DIALOG` word.
  > **cli:** Terminal monospace (the user's configured font). The brand shows through the fixed-column structure and the phraseology words (`open`, `RB`, `unable`, `DIALOG`, `stale`). The readback mirror is `[RB]` / `[  ]` / `[/ ]`. Bold is used only for the NAME column. SGR dim marks `stale` rows, and amber the `DIALOG` word. Output is ASCII only, and density is high.
- (.andromeda/design-system.md:122) The Typography detail file's roles, faces, stacks and sizes are a 0.2.0 frontend input, so no web type role (the Callsign NAME cell, the Bahnschrift / Cascadia Mono stacks) binds the verbs this chunk builds; an `/api/links` surface does not exist in 0.1.0.
  > A 0.2.0 frontend design input; 0.1.0 renders no page.

## Bindings
- **cli type rule (ASCII only, bold only for NAME) ↔ Surface: cli Component Patterns "Wheel, link and gate verbs"**: the link / unlink result lines the surface fixes (the `link    overseer -> builder  since …` line, `unlink  overseer -> builder`) are printed in terminal monospace, ASCII only, unstyled; (.andromeda/design-system.md:139)
- **cli type rule (ASCII only) ↔ Iconography**: the CLI's `->` (no inbound form) is the only marker the link lines carry, the web `→` / `←` never printed; (.andromeda/design-system.md:139)
- **cli type rule (no styling beyond NAME bold, `stale` dim, `DIALOG` amber) ↔ Anti-Patterns per-surface cli bans and the cli colour decision order**: the verbs' lines and `--json` documents carry no SGR, and under `--json`, a pipe, `NO_COLOR` or `TERM=dumb` nothing styled at all; (.andromeda/design-system.md:139)
