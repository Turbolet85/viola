@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=42 class=omission
leaf-text: `run`'s child screen, before it only a keyed start's automatic-verify lines and the verify child's own output, the CLI's `--json`)
source: ctr-krgnhdj4 «viola console writes only its screen to stdout, in the alternate screen, plain ASCII with no colour; its refusals and its internal error go to stderr.»
replace-with: `run`'s child screen, before it only a keyed start's automatic-verify lines and the verify child's own output, the CLI's `--json`, `viola console`'s own screen)
@leaf_edit n=2 leaf=CLAUDE.md line=60 class=stale
leaf-text: | CLI layouts (no web surface: viola renders no page; the fleet console's surface is owed by «Console scaffold») | `doc:layout-templates` §Surface: cli |
source: dec-vvpcu2np «The terminal fleet console, `viola console`, is its own surface in the human's terminal, entered on the alternate screen with the cursor hidden and restored on every end, in plain ASCII with no colour.»
replace-with: | CLI and fleet console layouts (no web surface: viola renders no page) | `doc:layout-templates` §Surface: cli / §Surface: console |
@leaf_edit n=3 leaf=CLAUDE.md line=59 class=omission
leaf-text: | `doc:design-system` §Color Palette / §Surface: cli |
source: ctr-twdprhag «`q` quits the fleet console with exit 0.»
replace-with: | `doc:design-system` §Color Palette / §Surface: cli / §Surface: console (the fleet console's keys) |
@leaf_edit n=4 leaf=CLAUDE.md line=36 class=omission
leaf-text: and no address, pid or workspace enters a process-log line.
source: dec-dsjweani «no key byte reaches any log line or output.»
replace-with: and no address, pid or workspace enters a process-log line; no `viola console` key byte reaches any log line or output.
