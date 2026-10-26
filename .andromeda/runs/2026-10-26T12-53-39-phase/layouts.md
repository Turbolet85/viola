# layouts extract

## Relevance
relevant: the chunk adds two human-surface verbs (`viola link` / `viola unlink`), their result lines and the `handoff:` row of `viola --help`. All of this is conditional on the founder's P4 form; under the "no explicit verbs" form nothing of this domain is built.

## Constraints
- The result lines take the wireframe's exact forms: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, the `already linked` variant for an existing pair, and `unlink  <driver> -> <driven>`. The word column is padded so `link` and `unlink` line up, and fields are separated by the two-space gutter (per layout-templates §Output structure — wheel, handoff and dialog verbs). The wireframe shows only the `HH:MM:SSZ` form of `since`. The plan does not say whether a `since` on another UTC date takes `release --budget`'s `YYYY-MM-DD HH:MM:SSZ` form, so the design needs to settle that, not the implementation.
- Argument order follows the channel `params`: `viola link|unlink <driver> <driven>`. Every name is a `ViolaName`, and unwrapped names are never valid targets (per layout-templates §Component — Primary navigation (verb structure)). `--json` is the machine view of every data verb, these two included (per layout-templates §Component — Primary navigation (verb structure)).
- Link output is plain. The cli token table marks `color-role-handoff` as "Not used", so no SGR goes on these lines (per layout-templates §Surface: cli). The arrow is ASCII `->`, and there is no `<-` inbound form (per layout-templates §IA notes).
- `viola --help` gains `handoff:  link, unlink` between `wheel:` and `setup:` only when this entry serves the verbs. The help lists served verbs only (per layout-templates §Output structure — `viola --help`). The `--help` grouping is board / traffic / wheel / handoff / setup, with the handoff group joining when its route entry serves its verbs (per layout-templates §Component — Primary navigation (verb structure)).
- Refusals use the `unable  <name>  <reason>  <detail>` line on stderr with its own `hint:` line last. Results go to stdout and refusals to stderr, never mixed. Under `--json` there is one stdout document, with no stderr line and no hint (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column). That section's server-check exit-21 hint (`the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`) and its `cli` catch-site list (`error: internal error`) both name the frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` and the cli verbs one by one. Neither list names `link` / `unlink` yet, so both extend with the verbs (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column). No hint names `viola release` (per layout-templates §Component — Primary navigation (verb structure)).
- The board shows no link column. The six columns stay fixed, and links are visible only as `link` / `unlink` verb output (per layout-templates §Component — Primary content block 1: `viola list` strip rows).
- The human words and columns are a stable output contract that drivers may read. A new field goes into `--json` first (per layout-templates §IA notes).

## Patterns to follow
- The wheel verbs' output (`viola pause` / `viola release`): one static stdout line, exit 0, spoken phraseology, plain ASCII, never redrawn (per layout-templates §Output structure — wheel, handoff and dialog verbs; §Primary screens (commands)).
- The as-built `viola --help` pin: the `HELP` literal in `tests/cli_output_plain.rs`, with `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` moving with the new row (per layout-templates §Output structure — `viola --help`).
- The exit-21 document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, used by the other frame verbs under `--json` (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- No terminator word: the last printed line says what happened, to whom and where, for example `link    overseer -> builder  since 19:43:58Z` (per layout-templates §Component — Footer / terminator).

## Anti-patterns to avoid
- No colour, glyph, spinner, `done` or `✓` on link / unlink output. Colour on the cli is reserved for `DIALOG` and `stale` (per layout-templates §Component — Hero / signature output line; §Surface: cli).
- No link column, link marker or link count on `viola list` (per layout-templates §Component — Primary content block 1: `viola list` strip rows).
- No `link` / `unlink` row in `--help`, and no help mention of them, before the verbs are served. No MCP tool, description or result names `link` or `unlink` (per layout-templates §Primary screens (commands)).

## Contract bindings
- arch §Standard Contracts ↔ the result lines: the `link` reply `{since, already_linked}` is the source of the `since` and `already linked` forms. `v1-13`'s `link` half requires every human line to be derivable from the `--json` result.
- layout-templates §Primary screens (commands) ↔ `v1-13`'s MCP half: the plan states that no MCP tool, description or result names `link` / `unlink`. So an MCP result carrying link facts conflicts with this domain's mandate unless the plan is amended. Whether `v1-13` claims an MCP half is P3's question.
- The second CARRY ↔ the result lines: these lines are the first surface to show a logged link name. Re-parsing it through `ViolaName::try_new` is the security/arch binding, and layouts supplies only the line forms.
- test-plan harness ↔ the `HELP` literal and the per-verb output pins in `tests/cli_output_plain.rs`. a11y cli landmarks ↔ the help row and the plain line forms.

## Acceptance criteria contributions
- `viola link <driver> <driven>` prints exactly one stdout line `link    <driver> -> <driven>  since <HH:MM:SSZ>`, or `link    <driver> -> <driven>  already linked` for an existing pair, exit 0. `viola unlink <driver> <driven>` prints `unlink  <driver> -> <driven>`, exit 0. Each line is plain ASCII with no SGR, and stderr is empty (per layout-templates §Output structure — wheel, handoff and dialog verbs).
- `viola --help` prints `handoff:  link, unlink` between the `wheel:` and `setup:` rows, and the `HELP` literal, `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` match it. With the verbs unserved, the row is absent (per layout-templates §Output structure — `viola --help`).
- A link or unlink refusal prints `unable  <name>  <reason>  <detail>` and then its own `hint:` line, last on stderr, with stdout empty. Under `--json` it prints one stdout document, stderr empty. A failed server check gives exit 21 with the endpoint hint (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- `viola list` output keeps its six captions `NAME LIVE STATUS WHEEL DIALOG CLI` with no link column after a link is recorded (per layout-templates §Component — Primary content block 1: `viola list` strip rows).
