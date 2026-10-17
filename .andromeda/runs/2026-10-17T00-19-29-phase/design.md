# design extract

## Relevance
partial — the chunk builds tests and harness only (no product output changes), but its legs assert human and `--json` output whose wording and stream rules the cli surface fixes: the board's budget cells, the start refusals of Path 1, the `release --budget` line and the exit-11 send refusal of Path 6.

## Constraints
- A leg that reads `viola list` in human mode asserts the board as built, which is uncoloured: no SGR, no bold NAME, no amber `DIALOG`, no dim row (the colour layer is owed to "CLI output discipline"); a budget-paused instance's WHEEL cell reads `driver - budget-paused` or `human - budget-paused`, padded to 23 (per design-system §Surface: cli → Component Patterns, pattern 1 "As built").
- The BAY line carries the budget state as words and figures (`5H <n> %`, `read <n>m ago`, `gate open` or the paused word), never a colour; a window figure is rounded down to a whole number (per design-system §Surface: cli → Component Patterns, pattern 1 "As built").
- Colour decision order puts `--json` first and `viola run` second: a `--json` result (the `status` document's nested `list`, `list --json`, `release --budget --json`) carries no colour, glyph or SGR, and `run` prints nothing once the child starts, so the test-owned outer PTY of Path 1 reads only the child's screen (per design-system §Surface: cli → Colour decision order).
- Streams: results go to stdout and refusals with their `hint:` line to stderr; under `--json` the whole result, refusals included, is one stdout document with the typed exit and no `hint:` line (per design-system §Surface: cli → Streams).
- Path 1's start-refusal steps meet the fixed-message pairs with no path or pid: `unable: <name> is already live` / `hint: viola list`, the squatted endpoint's `unable: the endpoint for <name> is held by another process` and its hint, the pinned copy's `unable: the pinned viola copy failed its integrity check` and its hint (per design-system §Surface: cli → Component Patterns, pattern 2 send hints, the `viola run` start refusals).
- Path 6's exit-11 send to the paused instance is the `budget-paused` refusal with detail `five-hour`, whose hint never names `viola release` (per design-system §Surface: cli → Exit-code phraseology; per design-system §Surface: cli → Component Patterns, pattern 2 `budget-paused` hint and T3).
- `viola release <name> --budget` in human mode prints one unstyled stdout line, `<name>  budget gate lifted until <time>  wheel human` when a pause stood, else `<name>  budget gate open  wheel human` (per design-system §Surface: cli → Component Patterns, pattern 4).

## Patterns to follow
- Assert on the fixed phraseology words in their columns (`live` / `stale`, `busy` / `idle` / `unknown`, `budget-paused`, `unable`), never on a colour or a glyph (per design-system §Surface: cli → Component Patterns, pattern 1).
- Prefer the `--json` document for machine assertions (the harness `status` nests `viola list --json` verbatim) and keep human-mode reads to the cells this chunk owes (the board's budget cell) (per design-system §Surface: cli → Streams).
- Whether the board's budget cell for each item already prints `- budget-paused` as built is research's question; the leg reads what the snapshot holds (per design-system §Surface: cli → Component Patterns, pattern 1 "As built").

## Anti-patterns to avoid
- Asserting colour, a green / red mark or any SGR in a human `list` read, or any SGR in a `--json` or piped read (per design-system §Anti-Patterns → Per-Surface Bans, cli).
- A test or harness diagnostic that copies a path, pid or upstream text into an asserted refusal or hint line (per design-system §Anti-Patterns → Per-Surface Bans, cli: "NEVER print upstream text, paths, pids").

## Contract bindings
- design ↔ tests: test-plan §6 Path 1 and Path 6 signals read the cli surface's refusal pairs, exit-code phraseology and the board's WHEEL / BAY cells (design-system §Surface: cli → Component Patterns, patterns 1, 2 and 4; §Exit-code phraseology).
- design ↔ architecture: the `list --json` `budget` member and each item's `budget_paused` are architecture's §Standard Contracts shapes; the design plan owns only their human rendering (design-system §Surface: cli → Component Patterns, pattern 1).

## Acceptance criteria contributions
- A human-mode `viola list` read in the Path 6 leg finds `builder`'s WHEEL cell without `budget-paused` and `overseer`'s with `- budget-paused`, uncoloured (per design-system §Surface: cli → Component Patterns, pattern 1 "As built").
- The exit-11 `send` to `overseer` prints `unable  budget-paused  five-hour` on stderr with a hint naming no `viola release`, and under `--json` the refusal object on stdout with stderr empty (per design-system §Surface: cli → Exit-code phraseology).
- Path 1's duplicate `run` reads stderr `unable: builder is already live` then `hint: viola list`, exit 1, no path or pid on either line (per design-system §Surface: cli → Component Patterns, pattern 2 `viola run` start refusals).
- Every `--json` read the legs make (`list`, `release --budget`, `send`, the `status` document's `list`) holds no ESC byte (per design-system §Surface: cli → Colour decision order).
