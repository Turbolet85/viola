@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=111
last: - 2026-11-13: No gate builds the separate `fuzz/` workspace,
@leaf_read leaf=.claude/rules/verification-harness.md lines=72
last: - 2026-10-20: `target/debug/viola` is a hard link cargo re-p
@leaf_edit n=1 leaf=.claude/rules/testing.md line=19 class=omission
leaf-text: a fault even when a row names it)
source: dec-diyjn3gm «The OS-gated census walks only files directly under tests/ and crates/*/tests/, so an integration binary split into a module directory keeps every gated case and its gated helpers in its root file.»
replace-with: a fault even when a row names it; the census walks only files directly under `tests/` and `crates/*/tests/`, so an integration binary split into a module directory keeps every gated case and its gated helpers in its root file)
