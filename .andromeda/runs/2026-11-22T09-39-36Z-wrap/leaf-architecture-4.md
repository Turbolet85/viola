@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=30 class=omission
leaf-text: The replay writes no file, gives no `cwd`
source: dec-vnqzmebd «Replay of an unreadable or unsupported snapshot writes no file and logs one state-recovered line; an absent snapshot is neither replayed nor logged.»
replace-with: The replay writes no file but logs one `state-recovered` line (an absent snapshot is neither replayed nor logged), gives no `cwd`
