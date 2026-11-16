@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once a
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pr
@leaf_edit n=1 leaf=.claude/rules/events.md line=17 class=omission
leaf-text: `activity`, `link`, `unlink`, `wheel` and the send records are log-only.
source: ctr-yns6vswp «Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake `wait`, by event kind alone, so every `turn-ended` wakes whatever its `background_tasks`; telling a finished»
replace-with: `activity`, `link`, `unlink`, `wheel`, a harness-origin `prompt-submitted` and the send records are log-only. The wake is by event kind alone: every `turn-ended` wakes whatever its `background_tasks`, and telling a finished turn from a paused one is the driver's.
