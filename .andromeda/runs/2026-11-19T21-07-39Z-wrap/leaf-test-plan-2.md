@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=79
last: - 2026-11-19: Sweep hazard: `snapshot.json` keeps `pid` and 
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=stale
leaf-text: and under `--framing` the exact verify echo probe command, append one stdout tag line
source: ctr-6gfalgaz «Under --local-command-mode a submitted local command whose post-condition is the stdout tag, and under --framing the exact verify echo probe command or the exact /context probe command, append one stdout tag line»
replace-with: and under `--framing` the exact verify echo probe command or the exact `/context` probe command, append one stdout tag line
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: (replayed framing variants not rewritten);
source: dec-6regymvl «A test that must have no fake-agent transcript removes none from the shared build; it runs its own private copy of the fake agent in a test-owned directory.»
replace-with: (replayed framing variants not rewritten; a test that must have no transcript removes none from the shared build, it runs its own private copy of the fake agent in a test-owned directory);
