@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=134
last: - 2026-11-25: A case whose start must end before its session
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: running its stand-in agent from a target dir of its own; provisional)
source: dec-athrwpuo «decision/live-rig-root-outside-repositories · Live rig root outside every repository — A live rig's live run refuses a root inside a repository, and runs its stand-in agent from a target dir of its own.»
replace-with: running its stand-in agent from a target dir of its own)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=26 class=stale
leaf-text: inside its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL)
source: dec-dnqm5ifj «decision/compact-confirm-window · Compact confirmation window — `/compact` on a verified CLI awaits its post-condition inside its own window, `COMPACT_CONFIRM_WINDOW`, of 40 s, clock-driven;»
replace-with: inside its own `COMPACT_CONFIRM_WINDOW` (40 s)
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=44 class=stale
leaf-text: the hook's own connect deadline is the provisional 750 ms `CONNECT_DEADLINE`
source: dec-jg7yzzuk «decision/hook-connect-deadline · Hook connect deadline below the spine deadline — The hook's own connect deadline is 750 ms, below the spine deadline the gate enforces.»
replace-with: the hook's own connect deadline is the 750 ms `CONNECT_DEADLINE`
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=61 class=stale
leaf-text: which waits its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL)
source: dec-dnqm5ifj «decision/compact-confirm-window · Compact confirmation window — `/compact` on a verified CLI awaits its post-condition inside its own window, `COMPACT_CONFIRM_WINDOW`, of 40 s, clock-driven;»
replace-with: which waits its own `COMPACT_CONFIRM_WINDOW` (40 s)
