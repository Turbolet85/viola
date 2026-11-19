@leaf_group group=architecture+security-plan+test-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=135
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=25 class=omission
leaf-text: save the test-only live rig's read of the key names and source word of its own captured UserPromptSubmit payload, never a value,
source: dec-uun4jqhm «decision/claude-shapes-in-one-crate · Claude payload shapes in one crate — viola-agent-claude is the only crate that knows Claude payload shapes, including those of claude agents --json. · except: Test-only rig seeds config; UserPromptSubmit/StopFailure key names, source word, error word 13|other, 2 lengths»
replace-with: save the test-only live rig's read of the key names and source word of its own captured UserPromptSubmit payload and the key names, error word (one of the CLI's 13, else other) and two text lengths of its own captured StopFailure payload, never a value,
@leaf_edit n=2 leaf=CLAUDE.md line=43 class=omission
leaf-text: which reads the key names and the source word of its own captured UserPromptSubmit payload, never a value,
source: dec-y3zoany4 «decision/claude-shapes-isolated · Claude-specific shapes isolated — Claude-specific shapes live only in viola-agent-claude. · except: Test-only rig seeds config; UserPromptSubmit/StopFailure key names, source word, error word 13|other, 2 lengths»
replace-with: which reads the key names and the source word of its own captured UserPromptSubmit payload and the key names, error word (one of the CLI's 13, else other) and two text lengths of its own captured StopFailure payload, never a value,
