@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules
@leaf_read leaf=.claude/rules/testing.md lines=121
last: - 2026-11-18: When a change makes the product write into a
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: draw the named screen after the tag-like paste's turn alone and after the long paste's turn alone)
source: ctr-hxsirlqm «contract/fake-agent-cli/start-screen · viola-fake-agent command line · --start-screen — Under --screens, a trusted start given --start-screen <phase> draws Screen.<phase> in place of its ready screen and keeps it standing»
replace-with: draw the named screen after the tag-like paste's turn alone and after the long paste's turn alone; under `--screens` a trusted start given `--start-screen <phase>` draws `Screen.<phase>` in place of its ready screen and keeps it standing, each stdin byte one key receipt line, Ctrl-C ending it with no key line, and no byte reaching the input box)
