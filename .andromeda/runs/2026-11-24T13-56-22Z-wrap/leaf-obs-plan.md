@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=59 class=omission
leaf-text: a start under `--origin` writes into `events.ndjson` after its launch line into a process-log line
source: dec-zbecdqti «The window line a start under --origin writes after its launch line, and a switch writes again after its own launch line, is log-only product audit data: it wakes no wait, opens no span and is never»
replace-with: a start under `--origin` writes into `events.ndjson` after its launch line, and a switch writes again after its own launch line, into a process-log line
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=70 class=omission
leaf-text: inside the `cli.focus` dispatch span, which carries no fields.
source: ctr-ku2k4wbr «Logs as cli into the named instance's cli-<name>.ndjson, a process-start and a process-exit inside the cli.switch dispatch span; a refusal ends process-exit with no detail, as send's does.»
replace-with: inside the `cli.focus` dispatch span, which carries no fields. `viola switch` logs as `cli` into the named instance's `cli-<name>.ndjson`, a `process-start` and a `process-exit` inside the `cli.switch` dispatch span; a refusal ends `process-exit` with no detail, as `send`'s does.
@leaf_edit n=3 leaf=.claude/docs/obs-summary.md line=72 class=omission
leaf-text: on `send`, `wait`, `last`, `answer`, `pause`, `release` or `viola focus` (exit 21
source: dec-4vztpgai «Commands acting on an instance (revive's start arm, adopt's start, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus, switch) run the instance strict-modes check before opening any»
replace-with: on `send`, `wait`, `last`, `answer`, `pause`, `release`, `viola focus` or `viola switch` (exit 21
@leaf_edit n=4 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: the six frame verbs', `link` / `unlink`'s and `viola focus`'s instance check
source: ctr-7tjreakr «A refused instance check of send, wait, last, answer, pause, release, link, unlink or switch exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-m»
replace-with: the six frame verbs', `link` / `unlink`'s, `viola switch`'s and `viola focus`'s instance check
@leaf_edit n=5 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: `strict-modes-failed` on the frame verbs, `link` / `unlink`, `viola focus` and
source: ctr-7tjreakr «A refused instance check of send, wait, last, answer, pause, release, link, unlink or switch exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-m»
replace-with: `strict-modes-failed` on the frame verbs, `link` / `unlink`, `viola switch`, `viola focus` and
