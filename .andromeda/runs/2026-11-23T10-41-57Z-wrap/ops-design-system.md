@row_add n=1 row=decision/launch-pair-flags area=surface-cli fact=F1 detector=none
name: Launch pair given by --model and --effort
rule: `viola run` and `viola revive` take `--model` and `--effort`; a missing half comes from `config.json` `default_model` or `default_effort`, else from viola's compiled default pair.
basis: fct-ydz7awsz, fct-xb6c4igy
because: The chunk adds the two start flags and their default resolution to the CLI surface, and no surface-cli row states them.
@row_add n=2 row=contract/run-revive-usage/pair-after-dashes area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and revive usage
item: a pair after the dashes
promise: A `--model` or `--effort` word after the `--` of `run` or `revive` is refused as usage exit 2 with one fixed stderr line naming `--model <MODEL>` and `--effort <EFFORT>`, echoing none of the caller's words.
basis: fct-ydz7awsz, fct-dgfm4vwr
because: The chunk adds a fixed exit-2 stderr wording that no master states, and the exit-2 row's promise cannot take it within its bound.
@row_add n=3 row=decision/revive-hands-recorded-pair area=surface-cli fact=F3 detector=none
name: Revive hands the recorded launch pair
rule: `viola revive` hands the child its recorded model and effort unless `--model` or `--effort` overrides a half; a recorded half that no longer parses takes the default half, and the two flags conflict with `--list`.
state: provisional
basis: fct-ydz7awsz, fct-ckuwa7cb, fct-z6wqhaac
because: The chunk gives `viola revive` a recorded pair and an override, the override and the stale-half rule a delegate's word, and no surface-cli row states it.
@row_add n=4 row=decision/board-shows-no-launch-pair area=surface-cli fact=F4 detector=none
name: Human board shows no launch pair
rule: The `viola list` human board shows no model or effort; the pair appears only as `model` and `effort` after `role` in the `list --json` and MCP `list` items.
state: provisional
basis: fct-aahvlfhn, fct-xb6c4igy, fct-z6wqhaac
because: The chunk carries the pair into the machine list items while keeping the six-column board unchanged, a delegate's word no surface-cli row states.
