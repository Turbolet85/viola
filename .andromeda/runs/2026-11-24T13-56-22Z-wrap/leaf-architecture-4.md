@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=16 class=stale
leaf-text: (both strings, `source: wrapper`, appended once at every start, after `lineage`)
source: ctr-yemjpoxs «The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`, and once at every switch, after the old child is reaped and before the respawn; it is log-only and never wakes `wait`.»
replace-with: (both strings, `source: wrapper`, appended once at every start, after `lineage`, and once at every switch, after the old child is reaped and before the respawn)
@leaf_edit n=2 leaf=.claude/rules/events.md line=16 class=stale
leaf-text: appended once after `launch` only when started under --origin)
source: ctr-t3m4inxx «The wrapper appends one `window` line with `source: wrapper` after the `launch` line, only when started under --origin, and again after a switch's `launch` line when the snapshot holds a window»
replace-with: appended once after `launch` only when started under --origin, and again after a switch's `launch` line when the snapshot holds a window)
@leaf_edit n=3 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: SessionEnd → `session-end`; Notification
source: ctr-u5chf3br «after a switch's respawn the old child's late `session-end` of the switched id is dropped once while the new child's pid is alive.»
replace-with: SessionEnd → `session-end` (after a switch's respawn the old child's late `session-end` of the switched id is dropped once while the new child's pid is alive); Notification
