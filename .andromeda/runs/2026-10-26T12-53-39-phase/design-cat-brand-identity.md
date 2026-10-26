# design · Brand Identity — category read

## Carried
- (.andromeda/design-system-detail/brand-identity.md:20) The human-TTY result lines of `viola link` / `viola unlink` (`link … since …Z`, `link … already linked`, `unlink …`) must be static fixed-column lines at expression 0.2: no colour on them (colour is spent only on `DIALOG` and `stale`), no in-place redraw, no spinner.
  > | cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |
- (.andromeda/design-system-detail/brand-identity.md:21) Under `--json`, a pipe, `NO_COLOR` or `TERM=dumb` the two verbs' output must sit at expression 0.0: no colour, no glyphs, no cursor control, no spinner.
  > | cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner.
- (.andromeda/design-system.md:14) The verbs are the handoff domain anchor: their lines must render a `driver → driven` transfer marker, driver first, driven second.
  > - **Handoff / transfer of control, and the tower voice recorder.** `driver → driven` transfer markers, and the ndjson tape played in order with `skipped` counts kept where they happened.
- (.andromeda/design-system.md:21) The verbs' lines and refusals must speak only short fixed phraseology words (`link`, `unlink`, `already linked`, `unable` plus a typed reason), with no filler and no guessing.
  > - **Standard phraseology and "unable" plus a reason.** Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal. There is no filler and no guessing.
- (.andromeda/design-system.md:32) The project's expression level 0.2 governs the new verbs' output as it governs every surface.
  > **Expression level:** `0.2` (base). This value governs animation intensity, layout freedom and interaction complexity across the whole project.
- (.andromeda/design-system.md:38) The verbs must add no motion of any kind to the CLI.
  > At 0.2 the duration scale's 0.0–0.2 row applies (0–100ms, no entrance), and viola's cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.

## Bindings
- **cli human TTY 0.2 row ↔ Surface: cli Tokens and the cli per-surface bans**: the row's colour-only-on-`DIALOG`/`stale` rule is the same rule the cli tokens state for handoff blue ("Handoff blue is never used") and the ban "NEVER colour anything except `DIALOG` (amber) and `stale` rows (dim)", so the `link` / `unlink` lines are unstyled (.andromeda/design-system-detail/brand-identity.md:20), (.andromeda/design-system.md:242), (.andromeda/design-system.md:376)
- **cli 0.0 row ↔ Surface: cli colour decision order**: `--json` is the decision order's first match, so the verbs' `--json` document carries no SGR and no glyph (.andromeda/design-system-detail/brand-identity.md:21), (.andromeda/design-system.md:245)
- **Handoff anchor ↔ Surface: cli Component Patterns and Iconography**: the `driver → driven` marker is printed on the CLI as ASCII `->` between driver and driven in the three result lines (`link … since …`, `link … already linked`, `unlink …`) (.andromeda/design-system.md:14), (.andromeda/design-system.md:209), (.andromeda/design-system.md:323), (.andromeda/design-system.md:324)
- **Standard phraseology ↔ Surface: cli Navigation Pattern**: the verbs' words join the flat lower-case verb list and the pending `handoff: link, unlink` help row (.andromeda/design-system.md:21), (.andromeda/design-system.md:345), (.andromeda/design-system.md:348)
