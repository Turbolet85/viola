# design extract

## Relevance
partial — design-system binds only the chunk's CLI half (`--json` error documents, and any `thiserror` message that can reach CLI stderr); MCP and channel error bodies are IPC, outside this domain, and the chunk adds no rendered surface, so no colour, type, spacing or motion token applies beyond the no-styling rule below.

## Constraints
- Errors and hints must carry no upstream text, path, pid or anyhow chain with serde sources; design-system names this the security plan's NEVER-log floor (per design-system §Anti-Patterns › Per-Surface Bans › cli).
- Each typed exit code has one fixed human stderr form and, where it has one, one `--json` document; the chunk must hold to that table and not add a shape beside it (per design-system §Surface: cli › Exit-code phraseology; rows in `.andromeda/design-system-detail/surface.md`, cli-exit-codes).
- Under `--json` the whole result, refusals and `error` objects included, must be one JSON document on stdout with the typed exit code, and no `hint:` line prints; the typed fields are the whole answer (per design-system §Surface: cli › Streams).
- The `--json` cause must travel as a detail code, never prose; per-cause `detail` codes for exit 21 are pending an arch amendment, and until then `--json` keeps arch's shape, so this chunk must not coin them in the design's name (per design-system §Surface: cli › Component Patterns › 2. `viola send`; `.andromeda/design-system-detail/surface.md`, cli-send).
- Faults get a fixed message and no hint, with full detail only under `instances/<name>/diagnostics/`, and stack traces never print (per design-system §Surface: cli › Platform-Specific Notes).
- `hook` writes nothing to stderr and `mcp` writes only MCP frames on stdout, so sanitising must not open a human error line on either verb (per design-system §Surface: cli › Platform-Specific Notes).
- A `--json` error document must hold no colour, glyph or SGR: `--json` is the first match of the decision order (per design-system §Surface: cli › Tokens › Colour decision order).

## Patterns to follow
- The fixed-message `unable:` / `hint:` form: fixed text, the instance's own parsed name at most, no path or pid; a message rewritten by this chunk should read in this form (per design-system §Surface: cli › Component Patterns › 5. `viola verify` / `viola run`).
- A hint never quotes the sent text or any upstream text (per design-system §Surface: cli › Component Patterns › 2. `viola send`).
- Content reports name a code and never the content, as the `verify` fixture-clean refusal's codes do; the shape to read when checking the capability's content-report leg (per design-system §Surface: cli › Exit-code phraseology; `.andromeda/design-system-detail/surface.md`, cli-exit-codes row 1).
- Standard phraseology: short fixed words are the vocabulary of every refusal, with no filler and no guessing; any new fixed message takes these words (per design-system §Brand Identity › Domain anchors).
- Human output is ASCII only, including every error line (per design-system §Surface: cli › Width).

## Anti-patterns to avoid
- Printing upstream text, paths, pids or anyhow chains in an error or hint, in either human or machine form (per design-system §Anti-Patterns › Per-Surface Bans › cli).
- Mixing streams: moving an error or refusal line onto stdout in human mode, or a hint into a `--json` run (per design-system §Anti-Patterns › Per-Surface Bans › cli).
- Printing a stack trace on any surface (per design-system §Anti-Patterns › Per-Surface Bans › cli).

## Contract bindings
- design ↔ scope, for research: design-system's exit-code table gives exit 14 an `<detail as opaque text>` human field and a refusal object under `--json`, and gives exit 20 a `wrapper-fault` document whose `detail` is an object, while the scope asks for a `detail` that is code-shaped or null. Whether the code's values at those two rows meet the scope's rule, and whether the plan rows need amending, is research's question (design-system §Surface: cli › Exit-code phraseology).
- design ↔ architecture: the `--json` error shapes and the pending per-cause `detail` codes are arch's to fix; design-system follows arch §Standard Contracts and names the arch amendment as owner (design-system §Surface: cli › Component Patterns › 2. `viola send`).
- design ↔ route: the revive start-refusal documents are owed to "Exit-cause code catalogue", which the scope also places out of bounds, so the enumeration should name that gap and not fill it (design-system §Surface: cli › 5. `viola verify` / `viola run`).
- design ↔ security plan: the cli ban cites the security plan's NEVER-log floor as its source, so that plan holds the floor's terms (design-system §Anti-Patterns › Per-Surface Bans › cli).
- design ↔ obs-plan: D-20's log detail codes are named beside the `--json` cause; a log code is not a `--json` `detail` until arch names it (design-system §Surface: cli › Component Patterns › 2. `viola send`; `.andromeda/design-system-detail/surface.md`, cli-send).

## Acceptance criteria contributions
- (design) With a path, an upstream text and a planted stripped `CLAUDE*` value in play, each driven CLI failure's stderr and `--json` document contain none of the three, and no anyhow chain or stack trace (per design-system §Anti-Patterns › Per-Surface Bans › cli).
- (design) Each driven `--json` failure writes exactly one JSON document to stdout with its typed exit code, no `hint:` line and no SGR byte; stderr is empty wherever the plan's exit-code row says so (per design-system §Surface: cli › Streams).
- (design) The human stderr form and `--json` document of each driven exit code match the plan's exit-code row, with any rewrite of a row recorded as a plan amendment rather than a silent change (per design-system §Surface: cli › Exit-code phraseology).
- (design) `viola hook` writes nothing to stderr and `viola mcp` writes only MCP frames on stdout across the driven failures (per design-system §Surface: cli › Platform-Specific Notes).
