@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pres
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=53 class=stale
leaf-text: (`RESPAWN_REAP_BOUND`, PROVISIONAL)
source: dec-khujfvwn «decision/respawn-reap-bound · A switch's reap wait is bounded at 10 s — A `viola switch` waits at most 10 s (RESPAWN_REAP_BOUND) for the old child to exit after its hang-up;»
replace-with: (`RESPAWN_REAP_BOUND`)
