@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once a
@leaf_edit n=1 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: SessionEnd → `session-end`; Notification / PostToolUse / PostToolUseFailure → `activity`.
source: ctr-zgw4cnv3 «The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads and writes nothing.»
replace-with: SessionEnd → `session-end`; Notification / PostToolUse / PostToolUseFailure → `activity`. The retired hook word `statusline` maps to no event: `viola hook statusline` takes the unknown-event exit 0 and reads and writes nothing.
