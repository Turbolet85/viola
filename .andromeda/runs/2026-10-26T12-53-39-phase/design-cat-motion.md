# design · Motion — category read

## Carried
- (.andromeda/design-system-detail/motion.md:10) The `link` / `unlink` verbs' human output must sit in the base expression row: no transition and no entrance, so a result line is printed whole, at once.
  > | 0.0-0.2 | 0-100ms | instant/none | none | none |
- (.andromeda/design-system.md:32) The expression level the chunk's two new verbs inherit is the project's single base value, with no per-verb override.
  > **Expression level:** `0.2` (base). This value governs animation intensity, layout freedom and interaction complexity across the whole project.
- (.andromeda/design-system.md:38) The two verbs are cli output, and the cli must have no motion at all, on a TTY or off it.
  > At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
- (.andromeda/design-system.md:330) `link` / `unlink` must add no progress pattern of their own; `verify`'s step counter stays the project's only one.
  > `verify` is a step counter, the project's one progress pattern
- (.andromeda/design-system.md:377) The verbs' `--json` and piped output must carry no cursor control.
  > - **NEVER emit colour, glyphs or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`.**
- (.andromeda/design-system.md:378) While the verbs wait on the channel round-trip, they must show no spinner, progress bar or redraw; each prints its one static result line.
  > - **NEVER use a spinner, progress bar or live-redrawing dashboard.** `wait` prints one static `waiting:` line and `verify` prints step-counter lines, because a spinner is progress presumed before it is confirmed.

## Bindings
- **Motion's base row ↔ Brand Identity**: the expression level `0.2` is the single source for how much motion there is, and its 0.0–0.2 row is the one the cli takes (.andromeda/design-system-detail/motion.md:10), (.andromeda/design-system.md:38)
- **No cli motion ↔ Surface: cli**: the verbs' `link` and `unlink` result lines (Component Patterns item 4) are static lines, with no spinner and no cursor control (.andromeda/design-system.md:378), (.andromeda/design-system.md:377)
- **No cli motion ↔ a11y-plan reduced motion**: the verbs animate nothing, so they owe no reduced-motion override (.andromeda/design-system.md:38)
