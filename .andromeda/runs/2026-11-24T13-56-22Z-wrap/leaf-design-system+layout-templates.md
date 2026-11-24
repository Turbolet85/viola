@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=74
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame
source: ctr-pdamcdgo «contract/strict-modes-refusal/exit-21 · Instance strict-modes refusal · exit 21 — A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch`»
replace-with: `viola send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` make of their instance before any frame
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: a `pump_child` error under `--json` prints no document), the `viola run`
source: ctr-wwq55wnt «contract/cli-verbs/switch · CLI verbs · viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json] — Switches a running wrapped session to a new model and effort in place, resuming the same session id»
replace-with: a `pump_child` error under `--json` prints no document), `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` (switches a running wrapped session to a new model and effort in place, resuming the same session id; at least one of `--model` and `--effort` is required, else a usage error, exit 2; a success prints `<name>  switched  <model> <effort>` on stdout, exit 0, or under `--json` one stdout document whose `ok` carries the new `model` and `effort`; it exits 0, 10, 12, 13, 14, 20 or 21 and writes nothing to the user's own settings), the `viola run`
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: nothing printed while the child runs;
source: ctr-mwmuxurd «contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands»
replace-with: nothing printed while the child runs, nor at a switch's respawn of the child;
