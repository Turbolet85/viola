# design · standing conventions

- S1 · Every new line the console draws for an action, a selection or a result is ASCII-only, using the CLI's text glyphs and separators, with no emoji or check-mark prefix. · surface-cli · iconography
  contract/cli-glyphs/field-separator · CLI glyphs · " - " — ` - ` separates parts inside one field.
  contract/cli-separators/space-hyphen-space · CLI separators · ` - ` — Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).
  decision/cli-two-space-columns · Two spaces between CLI columns — CLI spacing follows the fixed-width terminal grid, with two spaces between columns.
- S2 · Every new human-facing line escapes control characters and carries no internals in its errors or hints. · anti-patterns-never-do-these
- S3 · Every new console state or key added to the fleet console is a contract of the console surface: its keys, its view parts and its refusals. · surface-console
- S4 · Every new coloured span the console adds uses only the CLI colour tokens (attention, dim, bold, reset) and amber and dim alone, colour never the only cue. · color-palette · surface
  contract/cli-colour-tokens/reset · CLI colour tokens · reset — `\x1b[0m` at every colour depth, after every styled span.
  decision/cli-colour-amber-and-dim-only · CLI colour limited to amber and dim — CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted. · why: The quiz limits CLI colour to amber and dim.
  decision/no-green-accent · No run-green accent — The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
  decision/stale-rows-sgr-dim · Stale rows in SGR dim — In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.
