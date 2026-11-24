@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=27 class=omission
leaf-text: `send` during a running turn → `turn-running`
source: dec-uggkfwrn «A `send` while a turn runs, while another `send` is in flight, or while a switch stands, from its request until the new child's `session-start`, is refused `not-delivered` with detail `turn-running`»
replace-with: `send` during a running turn, or while a switch stands (from its request until the new child's `session-start`) → `turn-running`
@leaf_edit n=2 leaf=.claude/docs/stack.md line=11 class=omission
leaf-text: focus · up · plugin install
source: nam-kzs35wvo «name/command/switch — switch»
replace-with: focus · up · switch · plugin install
