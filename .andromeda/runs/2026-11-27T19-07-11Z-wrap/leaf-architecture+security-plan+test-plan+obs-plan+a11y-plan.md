@leaf_group group=architecture+security-plan+test-plan+obs-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=84 class=stale
leaf-text: `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL)
source: dec-xr4hahiu «decision/compact-confirm-window · /compact confirmation window — A /compact send waits a compiled 40 s window for its post-condition, validated per CLI version by the local-command-compact row against»
replace-with: `COMPACT_CONFIRM_WINDOW` (40 s, compiled)
