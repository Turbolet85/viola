# design · Spacing — category read

## Carried
- (.andromeda/design-system.md:153) The `viola link` / `viola unlink` human result lines must take the CLI's spacing: no spacing token of their own, laid on the fixed-width terminal grid with exactly two spaces between columns.
  > CLI spacing is platform-controlled: a fixed-width terminal grid, two spaces between columns.
- (.andromeda/design-system.md:147) The px spacing scale (and the detail file's tokens, among them `space-micro`'s gap "between `→` and a name") must not be applied by this chunk: it is a 0.2.0 frontend input, and the chunk's verbs print on the CLI only.
  > A 0.2.0 frontend design input; 0.1.0 renders no page.

## Bindings
- **CLI column spacing ↔ Surface: cli (the link verbs' lines)**: the two-space column rule (.andromeda/design-system.md:153) governs the three result lines the chunk prints, whose literals are fixed at (.andromeda/design-system.md:323) and (.andromeda/design-system.md:324) — `link    overseer -> builder  since <time>` (the sample's time a placeholder here), `link    overseer -> builder  already linked`, `unlink  overseer -> builder`: the verb word padded to one column width, two spaces before `since` / `already linked`.
- **CLI column spacing ↔ Iconography (the CLI's ASCII separators)**: the pair is joined by ASCII `->`, never `→`, and columns by two spaces (.andromeda/design-system.md:209); the web `→` and its 4px gap (.andromeda/design-system.md:213) are 0.2.0 inputs that do not bind.
- **CLI column spacing ↔ Anti-Patterns (cli)**: no hardcoded width and no wrap on the result lines (.andromeda/design-system.md:382).
- **CLI column spacing ↔ Navigation Pattern (help row)**: the `handoff: link, unlink` row of `viola --help` takes the grouped table's own layout between `wheel:` and `setup:` (.andromeda/design-system.md:348).
