@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/ci.md lines=35
last: - 2026-10-13: A CI job that reads `cancelled` with no steps 
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: each only a dir the test side itself made
source: dec-oo72d7us «A test-side removal under the e2e home takes only a directory the test side itself created, by its owner record, its own temp dir or its session name prefix, or another build's stamp dir whose lock it takes with try_lock.»
replace-with: each only a dir the test side itself made (by its owner record, its own temp dir or its session name prefix) or another build's stamp dir whose lock it takes with try_lock (provisional)
