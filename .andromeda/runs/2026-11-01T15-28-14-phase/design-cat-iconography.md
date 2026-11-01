# design · Iconography — category read

## Carried
- (.andromeda/design-system-detail/iconography.md:10) Every human line of this chunk's new refusals, on a refused socket directory and on a peer euid mismatch, and each one's hint, must stay ASCII only, so no `·` or other non-ASCII symbol may reach the terminal.
  > The CLI is ASCII only:
- (.andromeda/design-system-detail/iconography.md:11) A new refusal printed in the reason-and-detail form must separate its reason from its detail by two spaces, as separate fields.
  > Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
- (.andromeda/design-system-detail/iconography.md:12) Any separator inside one field of a new refusal or hint must be ` - `, never `·`.
  > ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.
- (.andromeda/design-system.md:207) The new refusals must carry their meaning in phraseology words, with no icon added.
  > Phraseology words carry the meaning.
- (.andromeda/design-system.md:209) The new refusal and hint strings must hold the CLI's glyph rule as the body states it broad.
  > The CLI is ASCII only: `->` with no inbound form, two spaces between columns, ` - ` inside one field
- (.andromeda/design-system.md:215) A refused directory or a euid mismatch must read as a printed word, never as a warning triangle or status mark.
  > In viola there are no robot or person icons, no status dots, no checkmarks and no warning triangles. Every one of those states is already a printed word.
- (.andromeda/design-system.md:270) The surface's own rule holds for the new refusal and hint lines, each being human output.
  > Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
- (.andromeda/design-system.md:377) A new refusal must take no check, cross or emoji prefix.
  > **NEVER use green `✓` / red `✗` prefixes or emoji.** The readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.

## Bindings
- **Two-space field separator ↔ Surface: cli, the refusal forms**: the separator rule (.andromeda/design-system-detail/iconography.md:11) governs the spacing of the two-space refusal form the surface names (.andromeda/design-system.md:261); the surface reader's refusal forms and this rule must agree on any new refusal line.
- **ASCII only ↔ Surface: cli, the exit-21 phraseology**: an exit-21 reading of a new refusal must hold ASCII only (.andromeda/design-system-detail/iconography.md:10) in the human form the exit-code phraseology fixes (.andromeda/design-system.md:338); its `--json` document stays that surface's.
