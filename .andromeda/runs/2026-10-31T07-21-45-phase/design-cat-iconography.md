# design · Iconography — category read

## Carried
- (.andromeda/design-system-detail/iconography.md:10) Every fixed message a `thiserror` `Display` impl or a CLI error path holds that can print on the CLI must be ASCII only, with no separator or symbol outside ASCII.
  > The CLI is ASCII only:
- (.andromeda/design-system-detail/iconography.md:11) A human refusal the chunk's message work reaches must keep its reason and its detail as separate fields, set apart by two spaces, and never fold them into one free-text string.
  > Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
- (.andromeda/design-system-detail/iconography.md:12) A separator inside one field of a fixed message that prints on the CLI must be ` - `, never `·`.
  > ` - ` (space, hyphen, space) replaces `·` inside one field or label
- (.andromeda/design-system.md:244) The test for a sanitised error body must read "glyph" as this definition gives it, any non-ASCII symbol.
  > here, as in the 0.0 expression row and the cli bans, a glyph is a non-ASCII symbol
- (.andromeda/design-system.md:379) Every CLI `--json` error document must carry no glyph, colour or cursor control, in its code, its `detail` or any fixed message it holds.
  > **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.**

## Bindings
- **CLI ASCII separators ↔ Surface: cli**: the separator rule of the category file `(.andromeda/design-system-detail/iconography.md:11)` sets the two-space gap that the refusal form of the streams list must print `(.andromeda/design-system.md:261)`, and the ASCII rule `(.andromeda/design-system-detail/iconography.md:10)` is the same requirement as the surface's width notes `(.andromeda/design-system.md:270)`.
- **Glyph ban under `--json` ↔ the cli NEVER-log floor**: an error body must hold no glyph `(.andromeda/design-system.md:379)` and no upstream text, path, pid or anyhow chain with a serde source `(.andromeda/design-system.md:382)`; the chunk's witnesses and enumeration of error bodies answer to both.
