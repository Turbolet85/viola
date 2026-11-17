@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=74
last: - 2026-11-17: A change to what the fake agent draws under `-
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=28 class=stale
leaf-text: `viola-harness supervise` as an ordinary child (outlives `boot`, stops only via `stop.request`;
source: dec-fcf6eyb7 «The supervisor is spawned as an ordinary child with no detach flags and outlives boot; the agent-run CLI boot leaves it untied, stopping only through its stop request, while an in-test harness session»
replace-with: `viola-harness supervise` as an ordinary child (outlives `boot`; the agent-run CLI's boot leaves it untied, stopping only via `stop.request`;
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=30 class=omission
leaf-text: the library `boot` spawns the `supervise` process from the given binary dir;
source: dec-fcf6eyb7 «while an in-test harness session ties it and its stamping viola verify to the test thread by a parent-death kill.»
replace-with: the library `boot` spawns the `supervise` process from the given binary dir, tying it and its stamping `viola verify` to the test thread by a parent-death kill;
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: `Wrapper::stop` waits for the endpoint to be unconnectable, not only the exit;
source: dec-gzyga4om «Every long-lived viola child a root test spawns is tied to its spawning test thread by a parent-death kill, exiting when its recorded parent is already gone, so a killed or panicked test leaves no wrapper»
replace-with: every long-lived viola child a root test spawns is tied to its spawning test thread by a parent-death kill (spawned only on a thread that outlives it; exiting when its recorded parent is already gone; `cross_killed_test_census_finds_no_untied_viola_spawn` finds no untied viola spawn in any test source), so a killed or panicked test leaves no wrapper or fake agent, and a Drop guard that kills, then waits, still covers the normal end; `Wrapper::stop` waits for the endpoint to be unconnectable, not only the exit;
