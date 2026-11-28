@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=36 class=stale
leaf-text: no `viola console` key byte reaches any log line or output.
source: dec-dsjweani «decision/console-key-reader-bounded · Console key reader bounded and closed — viola console reads the human's keys from its own terminal in raw mode in reads of at most 64 bytes, each read decoded into keys»
replace-with: no `viola console` key byte reaches any log line (its editing line echoes the typed text on its own screen, controls escaped).
@leaf_edit n=2 leaf=CLAUDE.md line=9 class=omission
leaf-text: vt100 `=0.16.2` for readiness,
source: dec-ou6ppzpr «decision/unicode-width-column-widths · unicode-width for terminal column widths — unicode-width is a direct, exactly pinned dependency of the root bin, used for terminal column widths of the console's editing line.»
replace-with: vt100 `=0.16.2` for readiness, unicode-width (a direct, exactly pinned dependency of the root bin) for the console editing line's column widths,
