# design · Color Palette — category read

## Carried
- (.andromeda/design-system-detail/color-palette.md:12) Every exit-21 and exit-1 refusal line and `hint:` line this chunk adds or moves must carry no amber: a refusal is an error, and amber is held to the `DIALOG` word.
  > The only attention colour: the 4px holder band and the `DIALOG` word of a cocked strip, and the CLI `DIALOG` word in `viola list` (the kind word after it stays uncoloured). The `DIALOG builder · viola` document title is plain text with no colour. Never used for errors, warnings, budget or hover.
- (.andromeda/design-system-detail/color-palette.md:53) If the contract choice gives a catalogue code a domain status row, that row must take the error values, ink and rule on anthracite with no hue; this is a 0.2.0 frontend design input, so the chunk renders nothing from it.
  > | Error, "unable" plus a reason (refusal, HTTP problem) | #1E2124 | #E6D8AE (plus a 1px `/` strike where a readback box exists) | #E6D8AE |
- (.andromeda/design-system.md:97) The domain status rows must cover every contract enum that gets a visual, so a closed code set this chunk adds to the architecture's contracts must either take one row per variant, through a body change, or be named as not visualised; the rows themselves stay a 0.2.0 input.
  > **Domain status colors.** A 0.2.0 frontend design input; 0.1.0 renders no page. These cover every enum in architecture.md's data model and contracts that gets a visual, one row per variant. The fifth column holds the printed word and the treatment. Colour never stands alone.
- (.andromeda/design-system.md:101) A closed code set that takes no status row must be named in this list, the body's record of what v1 does not draw, by the same amendment that adds the codes.
  > Not visualised in v1:
  > - `/ready` checks (`ok` / `unavailable` / `error`) and `/health` status. They are probe routes for tests and obs. A `claude_agents` failure surfaces on the page as STATUS `unknown`.
  > - `answer` behaviours (`allow` / `deny`, `approve` / `revise`). They are not logged as events, so the tape has nothing to draw.
- (.andromeda/design-system.md:242) The CLI colour map must stay as it stands: no cause, code or hint gains a colour token, a background or handoff blue, `viola link` and `viola unlink` refusals included.
  > CLI colour tokens, at truecolor, 256-colour and 16-colour depth: attention (arrival amber) on the `DIALOG` word only, always followed by the kind word; SGR dim on a whole `stale` row, the cock winning for `DIALOG`; bold for NAME values; a reset after every styled span. Handoff blue is never used, and backgrounds are never set: the terminal background belongs to the user.
- (.andromeda/design-system.md:363) Causes must be told apart by their code and hint words alone, never by a new hue or a per-cause colour.
  > **NEVER use color purely for decoration. Every color communicates meaning.** Amber means `dialog_pending` and nothing else. Blue means handoff and nothing else. Any third hue has no domain meaning.
- (.andromeda/design-system.md:378) The `unable:` and `hint:` lines of every cause must print uncoloured on every terminal, a TTY included, since neither is a `DIALOG` word or a `stale` row.
  > **NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim), and never either without its word.** The quiz confirms colour as a second cue only.
- (.andromeda/design-system.md:379) Every `--json` document the catalogue names must hold no SGR, glyph or cursor control, under `viola revive --json` after a passed preflight as under every other verb.
  > **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.

## Bindings
- **Amber's one role ↔ Surface: cli**: the palette's accent limit and the CLI token map must agree that refusals and hints carry no attention colour, (.andromeda/design-system-detail/color-palette.md:12) and (.andromeda/design-system.md:242).
- **Domain status rows ↔ architecture's contracts**: a closed code set added to the error document must be matched in the body by status rows or by a not-visualised entry, (.andromeda/design-system.md:97) and (.andromeda/design-system.md:101).
- **Colour as a second cue ↔ a11y-plan**: a cause must read whole from its printed code and hint, with colour never the carrier, (.andromeda/design-system.md:378).
- **No styled bytes ↔ test-plan's Exit-cause matrix**: the `tests/cross_exit_causes.rs` table must be able to assert each `--json` document and last stderr `hint:` line with no SGR in them, (.andromeda/design-system.md:379).
