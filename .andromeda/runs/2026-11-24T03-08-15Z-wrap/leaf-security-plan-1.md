@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=17 class=omission
leaf-text: every frame client (CLI verbs, `viola hook`'s event and dialog paths, `viola mcp`'s tools) through `check_instance` before its snapshot read;
source: dec-pmdeyzjd «CLI send, wait, last, answer, pause, release, link, unlink and focus run the instance strict-modes check before any frame or compositor read; a refusal exits 21, opens nothing under the home and writes no line.»
replace-with: every frame client (CLI verbs, `viola hook`'s event and dialog paths, `viola mcp`'s tools) through `check_instance` before its snapshot read, and `viola focus` through `check_instance` before any compositor read (a refusal exit 21, nothing opened under the home, no line);
