# design · Iconography — category read

## Carried
- (.andromeda/design-system-detail/iconography.md:8) The `viola link` / `viola unlink` result lines must mark the pair with the ASCII transfer arrow `->` (no `→`, no inbound `<-` form), and the link must print only as that verb output, never as a `viola list` column.
  > The CLI uses ASCII `->` and has no inbound form. `viola list` carries no link column in v1, so the six columns shared with the web strip stay fixed. Links print only as `link` / `unlink` verb output (`overseer -> builder`).
- (.andromeda/design-system-detail/iconography.md:10) The verbs' human output must be ASCII only, with two spaces separating columns and separating a refusal's reason from its detail.
  > The CLI is ASCII only:
  >   - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
- (.andromeda/design-system.md:209) The body's glyph rule for the CLI binds the same three result lines: `->` with no inbound form, two-space columns, and no link column on the board.
  > The CLI is ASCII only: `->` with no inbound form, two spaces between columns, ` - ` inside one field; `viola list` carries no link column in v1, so its six columns stay fixed.
- (.andromeda/design-system.md:215) The `link` / `unlink` results and refusals must carry no checkmark, status dot or warning glyph; the printed words (`link`, `unlink`, `since`, `already linked`, `unable`) carry the state.
  > In viola there are no robot or person icons, no status dots, no checkmarks and no warning triangles. Every one of those states is already a printed word.
- (.andromeda/design-system.md:244) The `->` in a link line must print in every human output (piped, `NO_COLOR` and `TERM=dumb` included), and only `--json` replaces it with its JSON document.
  > here, as in the 0.0 expression row and the cli bans, a glyph is a non-ASCII symbol, so the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that still print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document
- (.andromeda/design-system.md:270) The verbs' human output must stay within the ASCII set, `->` its only arrow.
  > Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).

## Bindings
- **Transfer arrow `->` ↔ Surface: cli, Component Patterns 4 (the body)**: the arrow fixed in iconography (.andromeda/design-system-detail/iconography.md:8) is the one the three result shapes print, `link    overseer -> builder  since <time>`, `link    overseer -> builder  already linked` and `unlink  overseer -> builder` (.andromeda/design-system.md:323), (.andromeda/design-system.md:324).
- **Link names ↔ Color Palette / Surface: cli Tokens (the body)**: the handoff blue that marks link names on the web (.andromeda/design-system-detail/iconography.md:8) is never used on the CLI, so the link lines print unstyled (.andromeda/design-system.md:242).
- **No link column ↔ Surface: cli, `viola list` (the body)**: the board's six columns stay fixed and show no link (.andromeda/design-system-detail/iconography.md:8), (.andromeda/design-system.md:279), which matches the scope's boundary "No link column on `viola list`".
- **Two-space separator ↔ Surface: cli Streams (the body)**: a refused `link` / `unlink` prints the two-space `unable  <name>  <reason>  <detail>` form on stderr (.andromeda/design-system-detail/iconography.md:11), (.andromeda/design-system.md:261).
- **ASCII-only / no glyph ↔ Anti-Patterns, cli bans (the body)**: no `✓` / `✗` prefix or emoji, and no glyphs under `--json` (.andromeda/design-system.md:375), (.andromeda/design-system.md:377).
