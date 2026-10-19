# design extract

## Relevance
partial — the chunk takes live readings and plans no new product behaviour or rendered surface. Its design reach is the cli surface's existing human forms that the rig reads or prints (`viola plugin install`'s lines, `viola list`'s board as a read of `idle`, `viola mcp`'s stdout). There is no web surface: §Surface: web-spa defers it to 0.2.0, and the §Self-Validation Protocol §4 Token Test is a 0.2.0 input.

## Constraints
- Per design-system §Surface: cli → Component Patterns (4. Wheel, link and gate verbs), `viola plugin install` prints four plain-ASCII stdout lines in one write, with no SGR: `viola@viola  written  <dir>`, then `run these to add it to Claude Code:`, then the two indented `claude plugin …` commands. `<dir>` is quoted only where the host's shell would split it. The plan marks this as a planning form as built, with the founder's wording still owed. A rig that copies these commands reads them as printed. Whether the built output matches this form byte for byte is a question for research.
- Per design-system §Surface: cli → Platform-Specific Notes, `viola mcp` is a hidden verb with no human design surface. It writes only MCP frames on stdout and appears in no `viola --help` row. The exit-1 reading (the home at 0770, the home check refusing) must therefore find no human text, hint or SGR on `viola mcp`'s stdout. The plan does not say what the refusal writes to stderr. Research should read that from the code.
- Per design-system §Surface: cli → Colour decision order, rule 3 (`viola mcp`) and rule 1 (`--json`) each mean no colour, no glyph and no SGR. The MCP `list` tool's answer, which the precedence reading reads from the session, is a machine document and never the styled board.
- Per design-system §Surface: cli → Component Patterns (1. `viola list`), an unwrapped session shows in the board's `-- UNWRAPPED - READ-ONLY --` section with LIVE `live`, its STATUS (`idle` / `busy`) and `n/a` in WHEEL, DIALOG and CLI. A failed, oversize or malformed agents read prints the board as if no `claude` were running. If the exit-1 reading uses this board to judge "reaches a usable session (`idle`)", it reads STATUS on that row. That a failed read leaves no unwrapped row must not be taken to mean the start was blocked.
- Per design-system §Surface: cli → Streams, results go to stdout. Refusals, with their `hint:` lines, and errors go to stderr. `viola plugin install`'s exit-21 refusal of its strict-modes check is the home-state `unable:` / `hint:` pair, or under `--json` the `instance-unreachable` document with stderr empty (per design-system §Surface: cli → Exit-code phraseology). A rig that makes the home 0770 before an install, not after, meets this refusal and not the `viola mcp` exit 1.
- Per design-system §Anti-Patterns → Per-Surface Bans (cli), no error or hint prints upstream text, paths, pids or anyhow chains. If a reading forces a fix or a wording amendment on a refusal line, the amendment must keep that ban.

## Patterns to follow
- The fixed-message refusal form `unable: <text>` / `hint: <text>`, per design-system §Surface: cli → Component Patterns (2. `viola send`: the readback mirror). Pattern 1's Refusal reuses it for the home-state pair. Any refusal shape a reading surfaces is compared against this form.
- The colour decision order's first-match-wins list, per design-system §Surface: cli → Colour decision order. Every non-human channel the rig touches (`--json`, `viola run` passthrough, `viola hook`, `viola mcp`) sits above any SGR.
- Under `viola run`, nothing is printed once the child starts, per design-system §Surface: cli → Component Patterns (5. `viola verify` / `viola run`). The wrapped start's screen belongs to the child, so readings come from `diagnostics/mcp.ndjson` and `events.ndjson`, never from viola-printed text.

## Anti-patterns to avoid
- No colour, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` or `viola run`/`revive` passthrough, per design-system §Anti-Patterns → Per-Surface Bans (cli). This covers any harness or rig output a forced fix would add.
- No green `✓` / red `✗` prefixes, emoji, spinner or progress bar in any human line a forced fix adds, per design-system §Anti-Patterns → Per-Surface Bans (cli).

## Contract bindings
- design ↔ architecture: the MCP `list` tool's payload is arch §Standard Contracts' `list` document (through `list::board_document`), not the styled board in design-system §Surface: cli → Component Patterns (1). The precedence reading binds to the arch shape.
- design ↔ security: §Anti-Patterns → Per-Surface Bans (cli) "never print … paths, pids" is the security plan's NEVER-log floor. It covers every refusal line a reading observes or an amendment rewords. `plugin install`'s `<dir>` on its stdout result line is a result, not an error.

## Acceptance criteria contributions
- If the chunk changes no product code, the human output of `viola plugin install` and `viola list` is unchanged. Each stays plain ASCII, with no SGR beyond the bold NAME / amber `DIALOG` / dim stale spans (per design-system §Surface: cli → Component Patterns (1. `viola list`) and (4. Wheel, link and gate verbs)).
- In the exit-1 reading, `viola mcp`'s stdout holds no human text, hint or SGR (MCP frames only) (per design-system §Surface: cli → Platform-Specific Notes).
- Any refusal line a forced fix adds or rewords takes the `unable:` / `hint:` fixed form on stderr and names no path, pid or upstream text (per design-system §Anti-Patterns → Per-Surface Bans (cli)).
