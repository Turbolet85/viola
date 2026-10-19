# layouts extract

## Relevance
partial: the chunk plans no new or changed CLI surface (its scope's Boundaries: no new product behaviour). Its readings consume existing cli outputs: `viola plugin install`'s printed commands, `viola run`'s passthrough, `viola mcp`'s machine view and, possibly, `viola list`'s unwrapped rack. So the layout rules apply as constraints on what the readings rely on, and on any fix the readings force.

## Constraints
- layout-templates §Surface: cli, Output structure — `viola plugin install` requires one plain-ASCII stdout write of four lines. `<dir>` is the canonical home joined with `marketplace`. On Unix it is printed bare unless the shell would split it, and then single-quoted. Under `--json` the output is the `ok` document with no path. A rig that runs the two printed `claude plugin` commands takes `<dir>` in that printed form. Whether the rig reads the printed line or builds the path itself is research's question.
- layout-templates §Surface: cli, Primary screens (commands) requires that `viola mcp` has no screen. Each `list` tool result is the CLI verb's `--json` document less `v`. A refusal is `isError: true` with the text `refused: <code>`, and no tool, description or result names `release`, `pause`, `link` or `unlink`. So "which server answers `list`" is read through that machine view and its `diagnostics/mcp.ndjson` lines, never through a human surface.
- layout-templates §Surface: cli, Output structure — `viola run` requires the passthrough to print nothing while the wrapped `claude` TUI holds the terminal. The wrapped precedence start's screen holds only the child's bytes. A start refusal is the exit-1 `unable:` / `hint:` pair on stderr, with no path and no pid.
- layout-templates §Surface: cli, Component — Header / banner (the BAY context line) requires that `viola hook` and `viola mcp` have no human surface. A registered `viola mcp` that exits 1 on its home check (home at 0770) therefore prints no viola line onto the unwrapped session's screen. What the CLI itself shows for a failed MCP server belongs to the CLI, not to this plan.
- layout-templates §Surface: cli, Output structure — `viola list` (human TTY) (its As built paragraph) requires an unjoined `claude agents --json` row to print under `-- UNWRAPPED - READ-ONLY --`. Its NAME is the `sessionId` through `escape_row`, LIVE `live`, STATUS `idle` or `busy`, and `n/a` in WHEEL, DIALOG and CLI. A failed, oversize or malformed read prints the board as if no `claude` were running. If the exit-1 reading's "usable session (`idle`)" is read off the board, this row is where it lands. Whether the reading uses `viola list` at all is research's question.
- layout-templates §Surface: cli, Expression level (this surface) requires expression 0.0 (no colour, no non-ASCII glyph, no cursor control) for `--json`, non-TTY output and the `viola run` passthrough. Board or plugin-install output captured into `evidence/` through a pipe carries no SGR.
- layout-templates §Surface: cli, IA notes requires the human columns and words to stay stable as a contract. New fields go into `--json` first, and adding a human column is a design change. A fix the readings force (scope Boundaries) changes no human column or word without a design amendment.

## Patterns to follow
- Capture machine-readable evidence through `--json`, the machine view of every data verb. `viola plugin install --json`, `viola list --json` and the MCP `list` result are typed documents with codes only (per layout-templates §Surface: cli, Component — Primary navigation (verb structure)).
- A piped `viola list` prints the same characters with no SGR, and the words carry every state. A board capture for the record is the piped form, untruncated when the width is unknown (per layout-templates §Surface: cli, Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)).
- `viola plugin install`'s exit-21 home pair (`unable: the viola home is not private to you` / its `hint:`) is the shape a 0770 home gets on the human verb. The exit-1 rig's home is set to 0770 after the install, or the install itself refuses (per layout-templates §Surface: cli, Output structure — `viola plugin install`).

## Anti-patterns to avoid
- Adding a human-visible line, column or banner as part of a reading, for example a viola notice on the child's terminal or a precedence column on the board (per layout-templates §Surface: cli, IA notes; per layout-templates §Surface: cli, Component — Header / banner (the BAY context line)).
- Any driver-facing output (an MCP result, a hint) that names `viola release` (per layout-templates §Surface: cli, Component — Primary navigation (verb structure)).

## Contract bindings
- The MCP `list` tool result ↔ architecture §Standard Contracts `list` payload: it is the `viola list --json` `ok` document less `v` (per layout-templates §Surface: cli, Primary screens (commands); per layout-templates §Surface: cli, Output structure — `viola list --json`).
- The unwrapped rack's STATUS ↔ the `agents-join` ledger row's `claude agents --json` read: the board's join by child pid or `sessionId` (per layout-templates §Surface: cli, Output structure — `viola list` (human TTY)).

## Acceptance criteria contributions
- (layouts) Where the precedence evidence records the answering server's `list` result, it has the `viola list --json` `ok` shape less `v`, and no tool or result in it names `release`, `pause`, `link` or `unlink` (per layout-templates §Surface: cli, Primary screens (commands)).
- (layouts) The wrapped precedence start's terminal shows no viola-originated line while the child runs (per layout-templates §Surface: cli, Output structure — `viola run`).
- (layouts) If the exit-1 reading reads `idle` off `viola list`, the evidence shows it as a row under `-- UNWRAPPED - READ-ONLY --` with LIVE `live`, STATUS `idle` and `n/a` in WHEEL, DIALOG and CLI (per layout-templates §Surface: cli, Output structure — `viola list` (human TTY)).
- (layouts) The `claude plugin marketplace add` argument the rig runs equals the `<dir>` that `viola plugin install` printed for that home (per layout-templates §Surface: cli, Output structure — `viola plugin install`).
