@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_stands n=1 leaf=.claude/docs/gotchas.md line=75
because: The line cites the ExitPlanMode ledger row and requirement v1-15, not the `viola list` columns, so the dropped "in v1" of ctr-q5dspexy bears on nothing it says.
@leaf_stands n=2 leaf=CLAUDE.md line=36
because: The line bars secret values (the messaging token and socket, R8-stripped CLAUDE* values, the transcript path, Hyprland strings) from logs and events.ndjson, and none of the named rows changed that: revive and replay only gained lineage, the start order only gained a lineage line, and the link-name row now only drops its events.ndjson location, which the line never cited.
