# layouts extract

## Relevance
partial: `viola mcp` has no human surface, but each tool returns the same result and refusal as its CLI verb's `--json` form, and layout-templates fixes those machine-view documents and the `--help` table that a new `viola mcp` subcommand would touch.

## Constraints
- layout-templates §Component — Header / banner (the BAY context line) says `viola mcp` has no human surface. The server prints no banner, no version line, no context header and no human line of any kind.
- layout-templates §Surface: cli (Expression level) requires expression 0.0 for every machine view: no colour, no non-ASCII glyph and no cursor control. An MCP tool result is a machine view, so no SGR byte reaches it.
- layout-templates §Component — Hero / signature output line (Machine view) requires the `send` machine view to carry only the typed document: `ok` with `submitted_at` + `cursor`, `ok` with `confirmed:false` + `unconfirmable`, or `refusal` + `detail`. It has no readback glyph (`[RB]` / `[  ]` / `[/ ]`) and no hint. The `send` tool returns these same three shapes.
- layout-templates §Output structure — `viola wait` / `viola last` requires each verb under `--json` to print exactly one document (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and no `waiting:` line, TTY header or `no message` line. The `wait` and `last` tools mirror those single documents. A dialog result whose event has no `dialog_id` stays the "dialog unknown" case of that section.
- layout-templates §Output structure — `viola list --json` requires the `list` machine view to be `{"v":1,"ok":{"items":[…],"budget":{…},"skipped":{…}}}`: no header line, no rack separator and no column padding. A failed strict-modes check is `{"v":1,"error":"instance-unreachable","detail":null}`. The `list` tool mirrors both documents and never the human board with its BAY line, captions or `-- UNWRAPPED - READ-ONLY --` rack.
- layout-templates §Component — Primary content block 2 requires the same outcome under `--json` to be one document with no hint text. The client-side `not-delivered` refusals (`control-character`, `empty-text`) also take that one-document form, and the machine view keeps arch's shape (`"detail":null` for exit 21) until arch names the detail codes. A hint never quotes sent or upstream text, a path or a pid.
- layout-templates §Component — Primary navigation (verb structure) says `<target>` is always a `ViolaName` and unwrapped names are never valid targets. It also says no hint names `viola release`, because the refusals that carry hints reach drivers. The tools' target parameter and every refusal text follow both rules.

## Patterns to follow
- The `--json` envelope used by every data verb: `{"v":1,"ok":…}` on success and `{"v":1,"error":"<code>","detail":null}` / the typed `refusal` + `detail` on a refusal. Codes and closed words only (per layout-templates §Output structure — `viola revive` and §Output structure — `viola list --json`). The MCP results reuse the CLI verb's own document. They are not a new shape.
- "New fields go into `--json` first" (per layout-templates §IA notes): a field an MCP result needs and the CLI `--json` lacks is a change to the `--json` contract, and the MCP result does not add it alone.
- Served verbs only in `--help` (per layout-templates §Output structure — `viola --help`): the grouped table lists only verbs that serve a human, and the `HELP` literal in `tests/cli_output_plain.rs` pins it byte-for-byte.

## Anti-patterns to avoid
- A human-surface artefact in a tool result: the readback glyph, an `open` issue line, `waiting:`, a `hint:` line, `error: internal error`, a padded column or an SGR byte (per layout-templates §Component — Hero / signature output line and §Surface: cli Expression level).
- A wheel or handoff verb on the driver surface: no `pause` / `release` / `link` / `unlink` tool, and no refusal text that points a driver to `viola release` (per layout-templates §Component — Primary navigation (verb structure)).
- Adding a `viola mcp` row or verb to the grouped `--help` table without a plan amendment. The plan's table (board / traffic / wheel / handoff / setup) names no `mcp`, and per layout-templates §IA notes a human-output change is a design change, not an implementation choice. Research must decide whether the subcommand is hidden from `--help`, and adding it to the table is raised as a design decision.

## Contract bindings
- layouts ↔ architecture: the `list` item / `budget` / `skipped` shapes and the `send` / `wait` / `last` / `answer` result and refusal documents are owned by architecture §Standard Contracts. layout-templates §Output structure — `viola list --json` and §Component — Primary content block 2 defer to it. The MCP tool results bind to the same contract. The `answer` `--json` document's shape is not drawn in layout-templates; it is architecture's.
- layouts ↔ tests: the `--help` table is pinned by the `HELP` literal in `tests/cli_output_plain.rs` (per layout-templates §Output structure — `viola --help`). Whether a new `viola mcp` clap subcommand changes that output is research's question.

## Acceptance criteria contributions
- For each of `send · wait · last · list`, the tool's success and refusal payload equals the document its CLI verb prints under `--json` for the same outcome. That includes `send`'s `confirmed:false` + `unconfirmable` form and `list`'s `instance-unreachable` document (per layout-templates §Component — Hero / signature output line and §Output structure — `viola list --json`).
- No tool result holds an ESC byte, a readback glyph (`[RB]`, `[  ]`, `[/ ]`), a `hint:` / `waiting:` / `unable` human line, or the text `release` (per layout-templates §Surface: cli Expression level and §Component — Primary navigation (verb structure)).
- `viola --help` stays byte-identical to the `HELP` literal after `viola mcp` is added, unless a layout-templates amendment adds a row (per layout-templates §Output structure — `viola --help`).
- `viola mcp` writes nothing to its stdio except MCP frames: no banner, version line or human line (per layout-templates §Component — Header / banner (the BAY context line)).
