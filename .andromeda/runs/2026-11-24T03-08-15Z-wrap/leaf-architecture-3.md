@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/stack.md line=11 class=omission
leaf-text: Subcommands `run · revive · open · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · plugin install`
source: ctr-tb7gbduz «contract/cli-verbs/focus · CLI verbs · focus — `viola focus <name> [--json]` takes one ViolaName and `--json` only.»
replace-with: Subcommands `run · revive · open · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · focus · plugin install`
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=34 class=omission
leaf-text: `unlink`, `verify`, `revive --list`, `plugin install`;
source: ctr-v52zy54b «contract/diagnostics-role-files/cli-name · home-level diagnostics role files · cli-<name>.ndjson — The CLI verbs' lines, <name> being the target instance for the frame verbs and focus and the caller's VIOLA_NAME»
replace-with: `unlink`, `focus`, `verify`, `revive --list`, `plugin install`;
