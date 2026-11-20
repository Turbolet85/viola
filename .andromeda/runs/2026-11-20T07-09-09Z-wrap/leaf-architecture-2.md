@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPT
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope, i
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rules
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=27 class=omission
leaf-text: a human prompt takes the wheel, so the send after it reads `human-typing`
source: dec-xbbudrvy «except: On a verified CLI, an unsent human-filed prompt with no key typed since the last human-filed prompt and a scheduled wake counted at the last turn end is a fired wake: relabelled harness, wheel kept.»
replace-with: a human prompt takes the wheel, so the send after it reads `human-typing` — except, on a verified CLI, an unsent human-filed prompt with no key typed since the last human-filed prompt and a scheduled wake counted at the last turn end: a fired wake, relabelled `harness`, the wheel kept
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=41 class=omission
leaf-text: `append_hook_event` with the driver relabel (
source: ctr-yecqt6py «`origin` is `driver` · `human` · `harness`: the hook files only `human` · `harness`; the wrapper sets `driver` only on the in-flight send's own prompt, and relabels a CLI-fired scheduled wake `harness`.»
replace-with: `append_hook_event` with the fired-wake relabel (a CLI-fired scheduled wake appended `origin:"harness"`, the wheel kept) and the driver relabel (
