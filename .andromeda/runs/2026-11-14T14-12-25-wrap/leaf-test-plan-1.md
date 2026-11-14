@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: reading its mode and reply set from files beside its exe.
source: dec-k5rwj7no «except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.»
replace-with: reading its mode and reply set from files beside its exe, except in viola-hyprland's own unit tests, which serve `hyprctl` by a planted shell stub because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
