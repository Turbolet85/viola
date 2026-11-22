@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=stale
leaf-text: a print-mode or `--version` probe child killed at the stop is not measured
source: ctr-a52um677 «Each verify spawn pair's process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.»
replace-with: every other spawn pair it started closes with `child_exit_status` and `duration_ms` as well, and a stopped run writes only the pairs it started
@leaf_edit n=2 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: The wrapper's stdin typed mark and its held scheduled-wakes count are in-memory state that open no span and write no line of their own, and carry no key and no prompt text.
source: dec-mioijqjj «The send's transcript echo read, the end offset before the paste and the tag count after it, opens no span and writes no line or span field of its own»
replace-with: The wrapper's stdin typed mark and its held scheduled-wakes count are in-memory state that open no span and write no line of their own, and carry no key and no prompt text. The send's transcript echo read (the end offset before the paste and the tag count after it) opens no span and writes no line or span field of its own; its result shows only as the `send-confirmed` line inside `run.confirm_window`.
