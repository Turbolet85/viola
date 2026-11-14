@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_read leaf=.claude/rules/testing.md lines=115
last: - 2026-11-14: A remove-the-guard control whose entry runs a
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=38 class=omission
leaf-text: (in a test-scoped bin dir, in place of the real `hyprctl`)
source: dec-k5rwj7no «except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.»
replace-with: (in a test-scoped bin dir, in place of the real `hyprctl`; viola-hyprland's own unit tests serve `hyprctl` by a planted shell stub instead, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants)
