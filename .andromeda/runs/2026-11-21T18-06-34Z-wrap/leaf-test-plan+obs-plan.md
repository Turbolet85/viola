@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=42 class=omission
leaf-text: the exit carrying `child_exit_status` and `duration_ms`.
source: ctr-wtqujetv «The print-mode claude sender of Run B's cross-session turn is logged as one process-start / process-exit pair with subject verify-sender; its prompt and the composed socket address appear in no line.»
replace-with: the exit carrying `child_exit_status` and `duration_ms`. `viola verify`'s print-mode claude sender of Run B's cross-session turn writes one `process-start` / `process-exit{subject:"verify-sender"}` pair; its prompt and the composed socket address appear in no line.
