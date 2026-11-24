@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=59 class=stale
leaf-text: the exit-1 refusals of `run`, `revive` and `verify` and every exit-21 strict-modes refusal
source: ctr-pwvbecet «The exit-1 refusals of `run`, `revive`, `open` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.»
replace-with: the exit-1 refusals of `run`, `revive`, `open` and `verify` and every exit-21 strict-modes refusal
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: the `viola run` / `viola revive` passthrough (on a verified CLI
source: ctr-lsboc7mb «Starts a wrapped claude session in a new foot terminal window beside its parent; its flags are a closed allowlist (--parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort, --json), the only words allowed after `--` are `claude` alone, and no shell runs.»
replace-with: `viola open <name>` (a wrapped claude session in a new foot terminal window beside its parent; flags a closed allowlist, only `claude` after `--`, no shell runs; it prints one of eight closed placement words `placed` · `refused` · `unplaced` · `unsupported` · `window-unseen` · `unreachable` · `oversize` · `malformed`), the `viola run` / `viola revive` passthrough (on a verified CLI
