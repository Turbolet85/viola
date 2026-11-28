@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=47
last: - 2026-11-28: A new `ObsEvent` moves three lists, not two: b
@leaf_edit n=1 leaf=.claude/rules/observability.md line=42 class=omission
leaf-text: into that verb's `cli` role file; the terminal outlives the verb.
source: dec-xthi4gwx «A console action opens no span of its own; it is observed by the console's console-action line and the child verb's own cli process log, the console running each verb as a child of its own executable, one at a time.»
replace-with: into that verb's `cli` role file; the terminal outlives the verb. `viola console` runs each action verb as a child of its own executable, one at a time, and a console action opens no span of its own: it is observed by one `console-action{verb, exit_code}` line per finished child in `console.ndjson` (no name, argument or output) and by the child verb's own `cli` process log.
