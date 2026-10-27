# design · cascade read · group 1 of 4

## Carried
- (.andromeda/design-system-detail/brand-identity.md:21) The re-verified vt100 screen-model panic degrade must leave the passthrough exactly as this row states it: `viola run` (and a passed `viola revive`) writes nothing of its own to the human's terminal while the child runs, so the degrade to `input-not-ready` adds no panic text, colour, glyph or cursor control there; a bounded-input refusal at `hook` or `mcp` stdin likewise has no human output.
  > | cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
  (.andromeda/design-system.md:379)
  > **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.** LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen; a passed `viola revive` is the same passthrough.
- (.andromeda/design-system-detail/brand-identity.md:20) The enumeration's `viola list` environment boundary must record that `NO_COLOR`, `TERM` and `COLORTERM` decide only these SGR bytes (colour as a second cue, never alone), and the escaping of `claude agents --json`-origin controls must keep every row a static fixed-column line that nothing redraws.
  > | cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |
  (.andromeda/design-system.md:10)
  > A field with no reading prints `unknown`.
  (.andromeda/design-system.md:43)
  > Missing readings print `unknown` in their box instead of shimmering.
  (.andromeda/design-system.md:21)
  > There is no filler and no guessing.
  (.andromeda/design-system.md:14)
  > the ndjson tape played in order with `skipped` counts kept where they happened.
  (.andromeda/design-system.md:15)
  > Unwrapped sessions are bare strips: another facility's traffic, visible but never a `target`.
- (.andromeda/design-system.md:382) Any refusal the chunk adds for an over-size, malformed or invalid-name input must hold this ban, and the human output of `last` / `list` must keep escaping C0/C1 controls of upstream text.
  > **NEVER print upstream text, paths, pids or anyhow chains with serde sources in errors or hints.** This is the security plan's NEVER-log floor, and human output of `last` / `list` escapes C0/C1 controls.
- (.andromeda/design-system.md:385) A caught vt100 panic and any new bounded-input refusal must print no stack trace; its detail goes to `diagnostics/` only.
  > **NEVER print stack traces.** Full detail goes to `diagnostics/` only.

## Bindings
- **brand-identity 0.0 passthrough row ↔ Surface: cli decision order and the chunk's panic re-read**: the row's "`viola run` prints nothing at all while the wrapped `claude` TUI runs" (.andromeda/design-system-detail/brand-identity.md:21) is item 2 of the colour decision order (.andromeda/design-system.md:246); the panic degrade the chunk re-reads must keep both, with no stack trace on the terminal (.andromeda/design-system.md:385).
- **brand-identity 0.2 TTY row ↔ Surface: cli colour depth and `list` escaping**: the colour the row permits (.andromeda/design-system-detail/brand-identity.md:20) is chosen only by the decision order and depth reads (.andromeda/design-system.md:248, .andromeda/design-system.md:254), the boundary the enumeration records, and its fixed-column rows hold only because agents-origin controls are escaped (.andromeda/design-system.md:289).
- **Brand Identity `unknown` ↔ color-palette STATUS and budget rows (group 2)**: "A field with no reading prints `unknown`" (.andromeda/design-system.md:10) is the word group 2's rows give a failed agents read and an absent budget reading (.andromeda/design-system-detail/color-palette.md:66, .andromeda/design-system-detail/color-palette.md:86).
- **Brand Identity bare strip ↔ color-palette unwrapped name (group 2)**: "never a `target`" (.andromeda/design-system.md:15) is group 2's unwrapped name shown as text only (.andromeda/design-system-detail/color-palette.md:71), the display-only input of the names-before-path-joins rule.
