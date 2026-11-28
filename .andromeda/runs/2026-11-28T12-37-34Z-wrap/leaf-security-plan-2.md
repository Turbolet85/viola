@leaf_group group=security-plan-2
@leaf_read leaf=.claude/rules/security.md lines=47
last: - 2026-11-24: A new verb that starts through `run::start` inher
@leaf_edit n=1 leaf=.claude/rules/security.md line=14 class=stale
leaf-text: either refusal exit 1, no line, modes left as found,
source: ctr-fh4crnmq «on stderr; at start with no line written and the mode left as found, mid-run with one process-exit line of detail strict-modes-failed.»
replace-with: either refusal exit 1, at start no line and modes left as found, a strict-modes refusal mid-run one `process-exit` line of detail `strict-modes-failed`,
