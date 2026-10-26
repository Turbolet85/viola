# design · Color Palette — category read

## Carried
- (.andromeda/design-system-detail/color-palette.md:13) The handoff blue's `link` / `unlink` use is the names in tape lines only, so the CLI `link` / `unlink` result lines this chunk adds must take no blue, and `already linked` must never be drawn as a status hue.
  > Transfer-marker arrows and names (outbound `→ builder`, inbound `← overseer`), the marker's 2px left tick (`--rule-strong`, the only blue rule), and the names in `link` / `unlink` tape lines only. Never a fill, never text on holder and never a status.
- (.andromeda/design-system-detail/color-palette.md:121) The `link` and `unlink` event kinds keep their tape rows (a 0.2.0 frontend input): the `unlink` lines the served method appends must carry the same driver → driven pair the row draws, and nothing in this chunk renders that row's colour on the CLI.
  > | EventKind::link | #1E2124 | none | #5B8DB8 names / #E6D8AE word | `link  overseer → builder` |
  > | EventKind::unlink | #1E2124 | none | #5B8DB8 names / #E6D8AE word | `unlink  overseer → builder` |
- (.andromeda/design-system.md:52) Blue stays locked to the handoff role; no link result, refusal or `already linked` word may borrow it as a state colour.
  > **Departure-holder blue #5B8DB8:** transfer markers only.
- (.andromeda/design-system.md:70) The core-role summary restricts the handoff blue to transfer markers and `link` / `unlink` names, never a status.
  > Role (handoff) #5B8DB8 departure blue, transfer markers and `link` / `unlink` names only, never a fill, never text on holder, never a status.
- (.andromeda/design-system.md:93) A link result (`since …Z`, `already linked`, `unlink …`) and its refusals must be carried by the printed word alone, never by a success or error hue.
  > viola deliberately carries them with **ink, fill, rule, strike and a printed word, never with a green/yellow/red hue**.
- (.andromeda/design-system.md:97) A new enum variant this chunk adds that gets a visual (a new refusal reason or detail word for `link` / `unlink`) needs its own domain status row, a body change; a variant with no visual adds none.
  > These cover every enum in architecture.md's data model and contracts that gets a visual, one row per variant.
- (.andromeda/design-system.md:242) The CLI colour tokens forbid the handoff blue on every CLI line, the `link` / `unlink` verbs' result lines included.
  > Handoff blue is never used, and backgrounds are never set: the terminal background belongs to the user.
- (.andromeda/design-system.md:361) Blue carries handoff meaning only, so no decorative use may appear on the new verbs' output.
  > Blue means handoff and nothing else.
- (.andromeda/design-system.md:376) The `link` / `unlink` result lines must print unstyled: only `DIALOG` (amber) and `stale` rows (dim) are ever coloured on the CLI.
  > **NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim), and never either without its word.**
- (.andromeda/design-system.md:377) The verbs' `--json` form, and any piped, `NO_COLOR` or `TERM=dumb` run, must emit no colour, glyph or cursor control.
  > **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.**

## Bindings
- **Handoff blue (no CLI use) ↔ Surface: cli Component Patterns**: the palette forbids blue on the CLI, and the surface fixes the plain result lines `link    overseer -> builder  since …Z` / `unlink  overseer -> builder` with ASCII `->` (.andromeda/design-system.md:242), (.andromeda/design-system.md:323)
- **EventKind::link / unlink rows ↔ Iconography and architecture's event line**: the tape row's `→` glyph is web-only, the CLI keeps `->`, and the row's names are the logged `driver` / `driven` the second CARRY re-parses through `ViolaName::try_new` before any surface shows them (.andromeda/design-system-detail/color-palette.md:121), (.andromeda/design-system.md:209)
- **Closed status set ↔ a11y-plan never-colour-alone and architecture's refusal codes**: any new `link` / `unlink` refusal word with a visual gets a row printed as a word, colour never standing alone (.andromeda/design-system.md:97)
