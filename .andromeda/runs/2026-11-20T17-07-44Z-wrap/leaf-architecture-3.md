@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_read leaf=.claude/rules/api.md lines=42
last: _This section is owned by `/wrap-session`. setup-project pres
@leaf_read leaf=.claude/rules/events.md lines=41
last: - 2026-10-16: A snapshot field that a reader expects once an 
@leaf_edit n=1 leaf=.claude/rules/events.md line=35 class=omission
leaf-text: Lock files are separate siblings (`<name>.lock`): a layout chosen
source: dec-z7zvfnpe «decision/on-disk-file-naming · On-disk file naming — On-disk files are lower-case, `.ndjson` for logs, `.json` for snapshots and `.lock` for lock files, one lock file per guarded file named `<name>.lock`.»
replace-with: Lock files are separate siblings (`<name>.lock`), save `ledger/verify.lock`, the automatic verify's attempt lock, which guards no file of its own name and stands apart from `stamps.json.lock`: a layout chosen
