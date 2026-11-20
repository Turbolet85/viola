@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_read leaf=.claude/rules/events.md lines=41
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_read leaf=.claude/rules/frontend.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: the hook files only human|harness, and `driver` comes only from the wrapper's relabel of the in-flight send's own prompt,
source: ctr-yecqt6py «`origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`; the wrapper sets `driver` only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake `harness`.»
replace-with: the hook files only human|harness; the wrapper relabels a CLI-fired scheduled wake `harness`, and `driver` comes only from the wrapper's relabel of the in-flight send's own prompt,
@leaf_edit n=2 leaf=.claude/rules/events.md line=22 class=omission
leaf-text: are `origin:"harness"` and never flip the wheel.
source: dec-xbbudrvy «On a verified CLI, an unsent human-filed prompt with no key typed since the last human-filed prompt and a scheduled wake counted at the last turn end is a fired wake: relabelled harness, wheel kept.»
replace-with: are `origin:"harness"` and never flip the wheel. So is a fired scheduled wake: on a verified CLI, an unsent human-filed prompt with no key typed since the last human-filed prompt and a scheduled wake counted at the last turn end is relabelled `harness`, the wheel kept.
