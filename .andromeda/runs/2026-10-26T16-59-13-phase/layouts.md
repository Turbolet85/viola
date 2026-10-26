# layouts extract

## Relevance
partial: the chunk is security hardening with no new screen. Its only layout footprint is the human and `--json` refusal output that each entry point gains when it adds a strict-modes check (`send`, `wait`, `last`, `answer`, `pause`, `release`, `run`, `verify`). `hook` and `mcp` have no human surface.

## Constraints
- layout-templates §Component — Primary content block 2 requires the following for a strict-modes refusal already planned (`viola link` / `viola unlink`): it is made before any role file or log opens, exits 21 with stdout empty, prints `revive`'s instance pair on stderr, writes no line to any log and leaves the directory's mode as found. Under `--json` it prints only the exit-21 document with stderr empty. The plan names no strict-modes refusal line for `send`, `wait`, `last`, `answer`, `pause` or `release`. Whether these verbs take that instance pair or the `unable  <name>  instance-unreachable` line is not settled in the layout plan. It is a design decision for P4, not one for implement.
- layout-templates §Component — Primary content block 2 (Hint line) requires every exit-21 and exit-1 cause to have its own hint, as the last stderr line. The hint never names a path or a pid and never quotes upstream text. A new strict-modes cause on a frame verb must stay distinguishable by its last stderr line from the not-running hint and from the server-check hint.
- layout-templates §Component — Primary content block 2 (Exit codes as the typed tail) requires the following: exit 21 is the strict-modes refusal for `list`, `plugin install` and `link` / `unlink`, with the fixed state pair. Under `--json` the same outcome is one stdout document with `"detail":null` and no stderr line until architecture names detail codes.
- layout-templates §Component — Primary content block 2 (Streams) requires results on stdout and refusals, hints and errors on stderr, never mixed.
- layout-templates §Component — Hero / signature output line requires `viola send` to keep its `[/ ] unable` mirror in the padded word column, with no colour, for any refusal it prints. Primary content block 2 states that `send`'s exit 21 keeps that mirror.
- layout-templates §Output structure — `viola run` requires a start to print nothing while the child runs. A start refusal is an `unable:` line and a `hint:` line on stderr, exit 1, with no paths and no pids. Primary content block 2 lists "run's pinned-copy refusal" among the fixed-message exceptions. Whether a re-hash or home-check refusal at start already has its pair is research's question.
- layout-templates §Component — Header / banner states that `viola hook` / `viola mcp` have no human surface. A fail-open hook refusal or an MCP refusal adds no human line. layout-templates §Primary screens requires MCP refusals to carry codes only (`refused: <code>`).

## Patterns to follow
- The `link` / `unlink` strict-modes refusal: check the driven instance before any role file or log opens, use `revive`'s instance pair, print the exit-21 `--json` document, and write no line (per layout-templates §Component — Primary content block 2).
- `viola list`'s whole-board refusal: the home pair for the home or `instances/` and the instance pair for one instance, exit 21, stdout empty, and the home's mode left as found (per layout-templates §Output structure — `viola list --json`).
- `viola plugin install`'s home refusal: the home pair, exit 21, nothing created, no log opened, and the `instance-unreachable` document under `--json` (per layout-templates §Output structure — `viola plugin install`).
- `viola revive`'s state-files pair and its `strict-modes-failed` `--json` document for an exit-1 verb (per layout-templates §Output structure — `viola revive`).

## Anti-patterns to avoid
- Do not let a hint or refusal line name a path, a pid, an endpoint or `viola release` (per layout-templates §Component — Primary content block 2; §Component — Primary navigation).
- Do not add a new human word or column without a design change. The human output is a contract that LLM drivers read, and new fields go into `--json` first (per layout-templates §IA notes).
- Do not put colour on a refusal. On the CLI, colour belongs only to `DIALOG` and `stale` (per layout-templates §Component — Hero / signature output line).

## Contract bindings
- The CLI refusal pairs bind to design-system cli pattern 2, which keys a hint to each cause, as cited by layout-templates §Component — Primary content block 2.
- The exit-21 `--json` document binds to the architecture's `--json` / exit-code conventions (§Standard Contracts / §Conventions), through layout-templates §Component — Primary content block 2 (Exit codes).
- "No line written to any log" on a strict-modes refusal (layout-templates §Component — Primary content block 2) binds to obs-plan §10's no-line exemption for a refused check, as named in the chunk scope.
- `viola --help` is pinned by the `HELP` literal in `tests/cli_output_plain.rs` (layout-templates §Output structure — `viola --help`). This binds to the tests harness.

## Acceptance criteria contributions
- A frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) whose new strict-modes check refuses exits 21 with stdout empty and writes no frame. Its refusal line (`send` inside its `[/ ] unable` mirror) is followed by a cause-specific hint as the last stderr line, naming no path or pid. Under `--json` it prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty (per layout-templates §Component — Primary content block 2).
- A `viola run` start refused by the home check or by the pinned-binary re-hash prints one `unable:` / `hint:` pair on stderr, exit 1, with no path and no pid, and nothing else (per layout-templates §Output structure — `viola run`).
- `viola --help` output stays byte-identical to the `HELP` literal: this chunk adds no verb (per layout-templates §Output structure — `viola --help`).
- `viola hook` and `viola mcp` print no human line on a refused check (per layout-templates §Component — Header / banner).
