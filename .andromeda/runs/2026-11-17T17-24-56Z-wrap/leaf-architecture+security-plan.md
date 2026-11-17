@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_read leaf=CLAUDE.md lines=134
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=43 class=stale
leaf-text: Claude shapes only in `viola-agent-claude`; Tokio only in `viola-mcp`.
source: dec-uun4jqhm «decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json.»
replace-with: Claude shapes only in `viola-agent-claude` (save the test-only live rig in `viola-e2e`, which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value); Tokio only in `viola-mcp`.
@leaf_edit n=2 leaf=CLAUDE.md line=25 class=omission
leaf-text: the only crate that knows Claude: hook JSON and `claude agents --json` shapes
source: dec-zypgxbrh «The test-only live rig in viola-e2e reads the key names and the source word of its own captured UserPromptSubmit payload, never a value.»
replace-with: the only crate that knows Claude (save the test-only live rig's read of the key names and source word of its own captured UserPromptSubmit payload, never a value): hook JSON and `claude agents --json` shapes
