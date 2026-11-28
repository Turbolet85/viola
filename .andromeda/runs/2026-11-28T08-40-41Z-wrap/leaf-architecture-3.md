@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectur
@leaf_edit n=1 leaf=.claude/docs/stack.md line=11 class=omission
leaf-text: `run · revive · adopt · open · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · focus · up · switch · plugin install`
source: nam-esoroze6 «name/command/console — console»
replace-with: `run · revive · adopt · open · send · wait · last · list · answer · hook · mcp · verify · pause · release · link · unlink · focus · up · switch · console · plugin install`
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=34 class=stale
leaf-text: `role_of` files every first word but `run`, `revive`, `adopt`, `mcp`, `hook` and a leading `-` flag under `cli`
source: dec-yfvq2f3a «decision/codes-only-process-logs · Codes-only process logs per role — Every role's codes-only process log goes to its home-level role file under diagnostics (run, hook, mcp, cli or console ndjson).»
replace-with: `role_of` files every first word but `run`, `revive`, `adopt`, `mcp`, `hook`, `console` (its own `console` role file) and a leading `-` flag under `cli`
