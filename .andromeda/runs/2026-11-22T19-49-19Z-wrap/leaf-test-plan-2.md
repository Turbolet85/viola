@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=80
last: - 2026-11-20: A plan's non-leg entry that reads a file a rou
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=29 class=omission
leaf-text: (`build-failed`, `verify-failed`, `run-exited`, `readiness-timeout`).
source: dec-o5rnvpkq «After a failed readiness the harness boot waits at most 15 s for its abort, so a stuck boot ends within 17 s, below the 20 s ci per-test kill of the harness lifecycle binary»
replace-with: (`build-failed`, `verify-failed`, `run-exited`, `readiness-timeout`). After a failed readiness boot waits at most 15 s for its abort, so a stuck boot ends within 17 s, below the 20 s ci per-test kill of the harness lifecycle binary; the 20 s readiness bound per instance stays.
