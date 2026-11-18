@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=74
last: - 2026-11-17: A change to what the fake agent draws under `-
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: under a 24×1 PTY that panics vt100's feed)
source: ctr-hxsirlqm «contract/fake-agent-cli/start-screen · viola-fake-agent command line · --start-screen — Under --screens, a trusted start given --start-screen <phase> draws Screen.<phase> in place of its ready screen and keeps it standing;»
replace-with: under a 24×1 PTY that panics vt100's feed), `--start-screen <phase>` (under `--screens`, a trusted start draws `Screen.<phase>` in place of its ready screen and keeps it standing; each stdin byte is one `key` receipt line, Ctrl-C ends it with no key line, and no byte reaches the input box)
