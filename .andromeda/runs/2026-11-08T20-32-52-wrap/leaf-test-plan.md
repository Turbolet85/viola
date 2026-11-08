@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=109
last: - 2026-11-06: A sweep for a `cfg` predicate reads the predic
@leaf_read leaf=.claude/rules/verification-harness.md lines=72
last: - 2026-10-20: `target/debug/viola` is a hard link cargo re-p
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=32 class=retired
leaf-text: E4 none in 0.1.0 (N/A: no frontend in 0.1.0; the frontend is deferred to 0.2.0, its technology not chosen)
source: fct-t57awnjr «N/A — viola has no web or graphical frontend.»
replace-with: E4 none (N/A: viola has no web or graphical frontend)
