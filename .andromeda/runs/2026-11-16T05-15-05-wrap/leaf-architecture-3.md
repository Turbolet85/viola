@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rule
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectur
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=41 class=omission
leaf-text: `append_hook_event` with the driver relabel
source: ctr-u4sxg2hi «A prompt-submitted whose text is the in-flight send's typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with origin driver, its text kept as submitted»
replace-with: `append_hook_event` with the driver relabel (a `prompt-submitted` that is the typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, appended with `origin:"driver"`, its text kept as submitted)
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=27 class=omission
leaf-text: `send` during a running turn → `turn-running` (the turn lives
source: dec-5khspss2 «A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take»
replace-with: `send` during a running turn → `turn-running` (a send's post-gate section — the human-typing and turn-running reads, the link record, `send-issued` and the paste — runs under one wrapper flight lock that the `hook.event` claim, turn start and end and wheel moves also take, a human key never; the turn lives
