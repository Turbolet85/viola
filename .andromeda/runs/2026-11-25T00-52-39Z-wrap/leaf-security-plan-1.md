@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/ci.md lines=36
last: - 2026-11-20: A WATCH's green run read from CI counts only on
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=65 class=stale
leaf-text: `viola adopt` (provisional) resumes only a session no live claude holds
source: dec-7vcwrkel «decision/adopt-signals-no-foreign-process · Adopt acts on no process it did not start — viola adopt resumes only a session no live claude holds, spawns its own child in its own terminal, and never ends or signals»
replace-with: `viola adopt` resumes only a session no live claude holds
