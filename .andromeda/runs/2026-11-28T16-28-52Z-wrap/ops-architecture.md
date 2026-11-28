@row_set n=1 row=decision/board-read-model-shape cell=rule seen=3 fact=F4 detector=none
value: The board read model is sync and std only, carries no Claude shape, and none of its types holds endpoint, pinned_bin, cwd, pid or started_at; its pending dialog id, beside the kind, comes from the live snapshot only and stays out of list --json and the MCP list item.
basis: fct-zcv5bkss
because: The board item now carries the pending dialog's id, read from the snapshot alone and skipped on serialisation, so the row on what the board's types hold must say where that id comes from and that no outward list shape takes it.
@row_add n=1 row=decision/console-runs-verbs-as-children area=established-decisions fact=F1 detector=none
name: Fleet console acts only through viola's own CLI verbs
rule: The fleet console acts only by running viola's own CLI verbs as child processes, its own exe with --home, no shell, VIOLA_NAME and VIOLA_DIR removed, one child at a time; a further action key while one runs reads busy.
basis: fct-zcv5bkss · fct-fzjwlezb · fct-pabjcb3k
because: The chunk gives the console its actions and fixes how it may act, a decision future console work must respect that no standing row holds.
@row_add n=2 row=decision/console-action-child-outlives-quit area=established-decisions fact=F2 detector=none
name: Console action child outlives the console
rule: A console action's child is not tied to the console's life: quitting the console leaves a running verb to finish on its own.
basis: fct-pabjcb3k
because: The action runner sets no parent-death tie, a chosen lifetime for every console verb that no standing row says.
@row_add n=3 row=decision/console-answers-permission-and-plan area=standard-contracts fact=F3 detector=none
name: Console answers only a permission or a plan
rule: The fleet console answers only a pending permission (y allow, n deny) or plan (y approve, n revise) through viola answer; a pending question is answered in the session, the console drawing one line that points to focus, and with nothing pending the answer key is unbound.
basis: fct-zcv5bkss · fct-ewvlmjfa
because: The console gains an answer key whose reach is bounded to two dialog kinds through the CLI verb, a scope decision no standing row holds.
@row_add n=4 row=decision/console-log-lines-codes-and-counts area=occupied-resources fact=F5 detector=D-arch-resources
name: Console log lines hold codes and counts only
rule: The console's process log writes one console-action line per finished child holding only the verb and its exit code, and on the editing line at most one key line per class per read holding a count, never a typed character.
basis: fct-vfoixk62 · fct-outvcy4j
because: The chunk adds the console-action event and a counted console-key line to the process log, and the registry must say what those lines may carry.
@row_add n=5 row=decision/unicode-width-column-widths area=stack-and-technologies fact=F6 detector=D-arch-decisions
name: unicode-width for terminal column widths
rule: unicode-width is a direct, exactly pinned dependency of the root bin, used for terminal column widths of the console's editing line.
why: already in the lock through vt100, so no crate is added
state: provisional
basis: fct-vz4oq6a2
because: A library new to the root bin's direct dependencies is chosen for a role, and the stack must name it.
