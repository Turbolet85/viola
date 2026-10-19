# layouts extract

## Relevance
partial — no surface is created; the chunk touches only the cli surface's existing `viola wait` result line, the `viola answer` refusal and hint, the `viola list` DIALOG cell and the `viola run` passthrough (the web-spa surface is deferred, per layout-templates §Surface: web-spa).

## Constraints
- layout-templates §Output structure — `viola wait` / `viola last` requires a `wait` result to be one stdout line naming the kind, the NAME and the cursor. A dialog kind takes the form `<kind>  <name>  dialog <id>  cursor <n>` (`dialog unknown` when the event carries no `dialog_id`), and every non-dialog kind takes the `turn-ended` form. The plan names no form for an expired-hold record. Whether the new kind takes the dialog form (it carries `dialog_id`) or the `turn-ended` form is a layout decision this chunk owes, and the chosen form is a layout-templates amendment.
- layout-templates §Output structure — `viola wait` / `viola last` requires no spinner and no elapsed-time counter. `waiting:` goes to stderr only on a TTY without `--json`, and is printed once. Under `--json` the verb prints exactly one document on stdout and nothing on stderr. A longer hold therefore never gains a countdown or a redraw (also per layout-templates §Surface: cli, the Motion tokens row: "Nothing redraws in place").
- layout-templates §Output structure — `viola wait` / `viola last` requires the dialog line in `wait` output to stay uncoloured ("this is a result, not the board"). An expired-hold line takes no SGR either, and amber stays reserved for the board's `DIALOG` word (layout-templates §IA notes, shared vocabulary).
- layout-templates §Output structure — wheel, handoff and dialog verbs requires `viola answer`'s refusal `unable  <name>  not-delivered  unknown-dialog` on stderr with exit 13, followed by the hint `that dialog is not pending; viola list shows the current DIALOG`. If a late answer to an expired hold gets a detail code of its own, layout-templates §Component — Primary content block 2 requires that cause to have its own `hint:` line as the last stderr line. The hint quotes no upstream text, names no path or pid, and never names `viola release`.
- layout-templates §IA notes requires the human columns and words to stay stable as a contract that drivers read. New fields go into `--json` first. Adding a human column, such as a hold length or an expiry time on the board, is a design change and not an implementation choice.
- layout-templates §Output structure — `viola run` requires `viola run` to print nothing while the child runs. When a hold expires inside the wrapper, no byte of viola's own reaches the terminal (expression level 0.0 for the passthrough, per layout-templates §Surface: cli, Expression level).
- layout-templates §Primary screens (commands) requires each `viola mcp` tool success and refusal to be the CLI verb's `--json` document less `v`. The `mcp` `wait` tool's result for the new kind is therefore the CLI `wait --json` document for that kind, and no tool result names `release`, `pause`, `link` or `unlink`.

## Patterns to follow
- The `wait` result line form `<kind>  <name>  …  cursor <n>`, with two-space gutters and one line appended per result, never redrawn (layout-templates §Output structure — `viola wait` / `viola last`).
- The refusal pair `unable  <name>  <reason>  <detail>` followed by a distinct `hint: <one plain instruction>` keyed by reason · detail, with the exit code as the typed tail (layout-templates §Component — Primary content block 2).
- The board's DIALOG cell reads `DIALOG <kind>` for a pending dialog and `none` otherwise. Whether an expired hold returns the cell to `none` follows from what the board read model takes as pending, and that is research's question (layout-templates §Component — Primary content block 1).
- The last printed line says what happened, to whom and where, with no terminator word (layout-templates §Component — Footer / terminator).

## Anti-patterns to avoid
- A spinner, countdown, elapsed counter or in-place redraw for a held dialog, on any verb (layout-templates §Output structure — `viola wait` / `viola last`; layout-templates §Surface: cli, the Motion tokens row).
- Colour on a `wait` result line, or amber on anything but the board's `DIALOG` word (layout-templates §IA notes).
- A hint that names `viola release`, a path, a pid or upstream text (layout-templates §Component — Primary navigation (verb structure); layout-templates §Component — Primary content block 2).

## Contract bindings
- The `wait` human line ↔ the event-line contract (architecture §Standard Contracts) and the obs event catalog: the new kind's normalised fields (`dialog_id`, the dialog kind) are what the human line may print. layout-templates §Output structure — `viola wait` / `viola last` fixes only the line's shape.
- The `mcp` `wait` tool ↔ CLI `wait --json` (layout-templates §Primary screens (commands)): one document shape serves both.
- The hint keyed by reason · detail ↔ design-system cli pattern 2 (layout-templates §Component — Primary content block 2).

## Acceptance criteria contributions
- (layouts) `viola wait <name>` woken by an expired-hold record prints exactly one stdout line in the form chosen and amended into layout-templates. The line names the kind word, the NAME, the `dialog <id>` (or `dialog unknown`) if the dialog form is chosen, and the cursor. It holds no SGR byte, and no spinner or redraw comes before it (per layout-templates §Output structure — `viola wait` / `viola last`).
- (layouts) `viola wait <name> --json` on the same record prints exactly one `{"v":1,"ok":…}` document on stdout with stderr empty, and the `mcp` `wait` tool returns that document less `v` (per layout-templates §Output structure — `viola wait` / `viola last`; layout-templates §Primary screens (commands)).
- (layouts) A `viola answer` to an expired dialog exits 13 with an `unable  <name>  not-delivered  <detail>` line, and its `hint:` line is the last stderr line. The hint names neither `viola release` nor a path nor a pid (per layout-templates §Component — Primary content block 2).
- (layouts) A hold that expires during `viola run` writes no byte of viola's own to the session terminal (per layout-templates §Output structure — `viola run`).
