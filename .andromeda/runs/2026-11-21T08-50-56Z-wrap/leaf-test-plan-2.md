@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=81
last: - 2026-11-20: A plan's non-leg entry that reads a file a round
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=38 class=omission
leaf-text: A non-absolute command is receipted `command_absolute:false, ran:false` and never run.
source: ctr-2cypc2sa «contract/fake-agent-hooks/session-start-env · viola-fake-agent hook execution · SessionStart env — The fake agent gives every SessionStart hook child each name of its set's recorded env names, always with the fixed value fake-agent.»
replace-with: A non-absolute command is receipted `command_absolute:false, ran:false` and never run. Every SessionStart hook child is given each name of its set's recorded env names (`Env.recorded-1.json`), always with the fixed value `fake-agent`.
