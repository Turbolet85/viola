@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/testing.md lines=121
last: - 2026-11-18: When a change makes the product write into a us
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: `/mcp` on a verified CLI is confirmed only by a new stdout tag in the transcript past its pre-paste end
source: dec-g2cvvbdj «On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, /compact only by one of cause compact with the same id, and /mcp and /context only by a new stdout»
replace-with: `/mcp` and `/context` on a verified CLI are confirmed only by a new stdout tag in the transcript past its pre-paste end
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: the exact verify echo probe command, like a stdout-tag local command
source: ctr-6gfalgaz «under --framing the exact verify echo probe command or the exact /context probe command, append one stdout tag line to the last started session's transcript and fire no hook.»
replace-with: the exact verify echo probe command or the exact `/context` probe command, like a stdout-tag local command
@leaf_edit n=3 leaf=.claude/rules/testing.md line=18 class=omission
leaf-text: `claude_bin` and `claude_in` in `fake.rs`, the one place a test plants the fake agent as `claude`
source: dec-6regymvl «A test that must have no fake-agent transcript removes none from the shared build; it runs its own private copy of the fake agent in a test-owned directory.»
replace-with: `claude_bin` and `claude_in` in `fake.rs`, the one place a test plants the fake agent as `claude` (a test that must have no fake-agent transcript removes none from the shared build, whose transcripts every concurrent test shares, and runs its own private copy of the fake agent in a test-owned directory)
