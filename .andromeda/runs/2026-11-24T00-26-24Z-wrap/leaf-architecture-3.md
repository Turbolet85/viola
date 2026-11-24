@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rules
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/stack.md line=11 class=omission
leaf-text: Subcommands `run · revive · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · plugin install`
source: ctr-3g45airy «contract/cli-verbs/open · CLI verbs · open — `viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude`.»
replace-with: Subcommands `run · revive · open · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · plugin install`
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=20 class=omission
leaf-text: a `launch` line appended once after `lineage`)
source: ctr-t3m4inxx «contract/wrapper-event-appends/window · wrapper event appends · `window` — The wrapper appends one `window` line with `source: wrapper` after the `launch` line, only when started under --origin; it is log-only and never wakes `wait`.»
replace-with: a `launch` line appended once after `lineage`, and, only when started under the hidden `--origin`, one log-only `window` line (`source: wrapper`) after the `launch` line, the snapshot's `window` record)
