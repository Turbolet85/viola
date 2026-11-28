@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=42 class=stale
leaf-text: `console-frame` (once per draw: required `cols`, `rows`, `fits`; an unread size logs 0x0, not fitting)
source: ctr-qb5gch45 «Logged on the first draw and then only when one of its fields changes, with cols, rows, shown and hidden (counts) and fits (boolean), all required; an unread terminal size logs 0x0, not fitting.»
replace-with: `console-frame` (on the first draw, then only when one of its fields changes: required `cols`, `rows`, `shown`, `hidden`, `fits`; an unread size logs 0x0, not fitting)
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: `viola console`'s refused terminal check writes no line, its cause the fixed stderr pair;
source: ctr-bobpud6a «by `viola console`'s `process-exit` for a board scan refused mid-run, each with exit 1.»
replace-with: `viola console`'s refused terminal check writes no line, its cause the fixed stderr pair; `viola console`'s strict-modes refusal at its start (exit 1, the home pair) writes no line, and a board scan refused mid-run exits 1 with the home pair after the terminal's restore and one `process-exit` line of detail `strict-modes-failed` in `console.ndjson`;
