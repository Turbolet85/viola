# design · Iconography — category read

## Carried
- (.andromeda/design-system-detail/iconography.md:10) Every human refusal and `hint:` line this chunk adds or rewords for an exit-21 or exit-1 cause must be ASCII only, with no `·` or other non-ASCII glyph.
  > The CLI is ASCII only:
- (.andromeda/design-system-detail/iconography.md:11) Where a cause's human refusal prints a reason and a detail, the two must stay separate fields split by two spaces, so a per-cause code shown as a detail follows `unable  instance-unreachable  <detail>` spacing and is never joined to the reason by another separator.
  > Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
- (.andromeda/design-system-detail/iconography.md:12) Any division inside one field or label of a new human refusal or hint must use ` - `, never `·`.
  > ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.
- (.andromeda/design-system.md:207) Each cause must be told apart in its human form by its own fixed words, so a hint names its cause in phraseology and leans on no symbol or styling for the meaning.
  > Phraseology words carry the meaning.
- (.andromeda/design-system.md:209) The CLI half of the glyph rule binds this chunk's human lines even though the section's web half waits for 0.2.0, and the chunk must leave `viola list`'s six columns fixed when its refusal joins the table.
  > The CLI is ASCII only: `->` with no inbound form, two spaces between columns, ` - ` inside one field; `viola list` carries no link column in v1, so its six columns stay fixed.
- (.andromeda/design-system.md:215) A refusal or error state in the catalogue must be carried by its printed word and code, with no warning triangle, checkmark or status dot beside it.
  > In viola there are no robot or person icons, no status dots, no checkmarks and no warning triangles. Every one of those states is already a printed word.

## Bindings
- **ASCII-only separators ↔ body, Surface: cli Width**: the separator rule and the surface's human-output rule must both hold on every cause's stderr pair, read from (.andromeda/design-system-detail/iconography.md:10) and (.andromeda/design-system.md:270).
- **Two-space reason and detail ↔ body, Surface: cli Exit-code phraseology**: the field split must match the typed exit-21 human form and the refusal form the send pattern states, read from (.andromeda/design-system-detail/iconography.md:11), (.andromeda/design-system.md:303) and (.andromeda/design-system.md:338).
- **Printed words over icons ↔ body, cli Per-Surface Bans**: the no-icon rule and the ban on green or red prefixes and emoji must both hold on each cause's refusal, read from (.andromeda/design-system.md:215) and (.andromeda/design-system.md:377).
