@leaf_group group=test-plan+obs-plan-1
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:ar
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=141 class=omission
leaf-text: the fake agent dialog and framing replay tests over every recorded set take the `verify_window_` kill too (the override names them, in both the `ci` and `mutants` profiles)
source: dec-5eznx6xr «Tests named verify_window_, the fake agent dialog and framing replay tests over every recorded set, and the verify-driven tests the class's filter line names one by one, take an 80 s per-test kill»
replace-with: the fake agent dialog and framing replay tests over every recorded set, and the verify-driven tests the class's filter line names one by one, take the `verify_window_` kill too (the override names them, in both the `ci` and `mutants` profiles)
