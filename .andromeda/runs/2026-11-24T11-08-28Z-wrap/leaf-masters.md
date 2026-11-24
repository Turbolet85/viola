@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=38 class=omission
leaf-text: read by `viola verify` and by the `viola run` / `viola revive` start only when
source: dec-rlsdenig «viola verify and the start of run, revive and adopt read CLAUDE_CONFIG_DIR's value, only when claude_env_keep keeps that name, to choose the Claude config holding the trust record.»
replace-with: read by `viola verify` and by the `viola run` / `viola revive` / `viola adopt` start only when
@leaf_edit n=2 leaf=CLAUDE.md line=87 class=omission
leaf-text: each wrapper (a `viola run`, or a `viola revive` whose preflight passed) owns
source: ctr-3nzitcr6 «The passthrough is entered by viola run, by a viola revive whose preflight passed and by a viola adopt whose preflight passed.»
replace-with: each wrapper (a `viola run`, or a `viola revive` or `viola adopt` whose preflight passed) owns
