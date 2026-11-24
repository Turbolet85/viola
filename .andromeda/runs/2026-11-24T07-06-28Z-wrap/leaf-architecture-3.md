@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=21 class=omission
leaf-text: replays no launch spec but the recorded model and effort, keeps
source: dec-wy74jadi «viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch spec but the recorded model and effort, and under the hidden --origin the recorded workspace»
replace-with: replays no launch spec but the recorded model and effort, and under the hidden `--origin` the recorded workspace, keeps
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=34 class=omission
leaf-text: `focus`, `verify`, `revive --list`, `plugin install`;
source: ctr-v52zy54b «The CLI verbs' lines, <name> being the target instance for the frame verbs and focus and the caller's VIOLA_NAME for list, verify, revive --list, up and plugin install.»
replace-with: `focus`, `up`, `verify`, `revive --list`, `plugin install`;
@leaf_edit n=3 leaf=.claude/docs/stack.md line=11 class=omission
leaf-text: focus · plugin install
source: ctr-uv2g64ih «`viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; names given narrow the fleet and a name not scanned reads»
replace-with: focus · up · plugin install
