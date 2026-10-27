# design · cascade read · group 2 of 4

## Carried
- (.andromeda/design-system-detail/color-palette.md:66) A failed, over-size, malformed or unjoined `claude agents --json` read must leave a wrapped row's STATUS reading the printed word `unknown`, never a guessed value. The chunk's size-cap and closed-type witnesses on that read must assert this word; the row's hexes are 0.2.0 frontend inputs and bind nothing here.
  > `unknown` printed in the box (`claude agents --json` missing, failed or not joined)
  (.andromeda/design-system-detail/color-palette.md:64)
  > `idle`
  (.andromeda/design-system-detail/color-palette.md:65)
  > `busy`. No spinner, pulse or animation.
  (.andromeda/design-system.md:102)
  > A `claude_agents` failure surfaces on the page as STATUS `unknown`.
- (.andromeda/design-system-detail/color-palette.md:86) A `budget.json` read that is over-size, malformed or absent must yield no reading. Each figure must then print `unknown` and nothing is blocked, because the gate fails open. The chunk's bounded-read witness on `budget.json` must assert both the word and the open gate.
  > `5H unknown  7D unknown  read unknown`. Nothing is blocked.
  (.andromeda/design-system-detail/color-palette.md:91)
  > `5H unknown`
- (.andromeda/design-system-detail/color-palette.md:71) An unwrapped session's name comes in as external `claude agents --json` text. It must stay display text only and never become a `target`, so it must never reach a name-keyed path join. The chunk's names-before-path-joins rule and its reader enumeration must keep it on the escaped-text side.
  > The name is shown as text, never as a target.
  (.andromeda/design-system.md:99)
  > a bare strip is never cocked and its name never a target

## Bindings
- **color-palette STATUS `unknown` ↔ Surface: cli `viola list`**: the closed STATUS words `idle` / `busy` / `unknown` (.andromeda/design-system-detail/color-palette.md:66) are the same words the CLI board prints when a join is made or a read fails (.andromeda/design-system.md:292). One witness of a bounded, failed agents read therefore covers both: STATUS `unknown`, never `idle` / `busy`.
- **color-palette budget `unknown` ↔ Surface: cli `viola list` BAY line**: a bounded `budget.json` read that gives no reading prints `unknown` and blocks nothing (.andromeda/design-system-detail/color-palette.md:86). That is the same `5H` / `7D` cell the BAY line prints (.andromeda/design-system.md:278).
- **color-palette unwrapped name ↔ Surface: cli `viola list` escaping**: an unwrapped name is text, never a target (.andromeda/design-system-detail/color-palette.md:71). Its row's control characters must be escaped as `\xHH` (.andromeda/design-system.md:289). Together these make the agents-origin name a display-only external input in the chunk's enumeration.
