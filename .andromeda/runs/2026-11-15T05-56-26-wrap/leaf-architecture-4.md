@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/events.md line=30 class=retired
leaf-text: with six product readers
source: dec-xicog4jc «Product readers read a snapshot only through the replay, which meets an unreadable or newer snapshot as absent, so it gives no endpoint and no frame is written.»
replace-with: with five product readers
@leaf_edit n=2 leaf=.claude/rules/events.md line=30 class=retired
leaf-text: so the hook fails open), `hook statusline`'s `serve` (a replayed snapshot holds no `statusline_command`, so nothing runs) and
source: ctr-zgw4cnv3 «The retired hook word `statusline` maps to no hook event: `viola hook statusline` takes the unknown-event exit 0 and reads and writes nothing.»
replace-with: so the hook fails open) and
